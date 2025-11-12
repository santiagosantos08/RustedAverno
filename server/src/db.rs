use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Pool, Postgres, Row as SqlxRow};
use gns::GnsConnection;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DbStmt {
    GetUser,
    Custom(String),
}

impl DbStmt {
    pub fn as_str(&self) -> &str {
        match self {
            DbStmt::GetUser => "get_user",
            DbStmt::Custom(s) => s,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "get_user" => DbStmt::GetUser,
            other => DbStmt::Custom(other.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DbJob {
    pub id: u64,
    pub stmt: DbStmt,
    pub params: Vec<String>,
    pub respond_to: GnsConnection,
}

#[derive(Debug, Clone)]
pub struct DbResult {
    pub id: u64,
    pub stmt: DbStmt,
    pub success: bool,
    pub rows: Option<Vec<Vec<String>>>,
    pub message: Option<String>,
    pub respond_to: GnsConnection,
}

struct WorkerThread {
    pending_count: usize,
    idle_calls: usize,
    last_activity: Instant,
}

struct WorkerState {
    input_queue: VecDeque<DbJob>,
    output_queue: VecDeque<DbResult>,
    workers: Vec<WorkerThread>,
    pool: Pool<Postgres>,
    prepared: HashMap<DbStmt, String>,
    max_pending_per_thread: usize,
    max_idle_calls: usize,
    next_worker_id: usize,
}

pub struct DbWorker {
    state: Arc<Mutex<WorkerState>>,
}

impl DbWorker {
    pub fn new(conn_str: &str, prepared: HashMap<DbStmt, &str>) -> Self {
        let conn_str = conn_str.to_string();
        let prepared_map: HashMap<DbStmt, String> = prepared
            .into_iter()
            .map(|(k, v)| (k, v.to_string()))
            .collect();

        // Create pool in a blocking context
        let pool = std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            runtime.block_on(async {
                PgPoolOptions::new()
                    .max_connections(16)
                    .connect(&conn_str)
                    .await
                    .expect("Failed to create pool")
            })
        })
            .join()
            .unwrap();

        let state = Arc::new(Mutex::new(WorkerState {
            input_queue: VecDeque::new(),
            output_queue: VecDeque::new(),
            workers: Vec::new(),
            pool,
            prepared: prepared_map,
            max_pending_per_thread: 5,
            max_idle_calls: 25,
            next_worker_id: 0,
        }));

        // Spawn initial worker thread
        Self::spawn_worker_thread(state.clone(), 0);

        Self { state }
    }

    fn spawn_worker_thread(state: Arc<Mutex<WorkerState>>, worker_id: usize) {
        thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            runtime.block_on(async {
                loop {
                    thread::sleep(Duration::from_millis(5));

                    let job_opt = {
                        let mut state = state.lock().unwrap();

                        // Check if this worker should be terminated
                        if worker_id < state.workers.len() {
                            if state.workers[worker_id].idle_calls >= state.max_idle_calls
                                && state.workers.len() > 1 {
                                // Mark for removal
                                state.workers.remove(worker_id);
                                println!("Worker {} terminated due to inactivity", worker_id);
                                return;
                            }
                        }

                        state.input_queue.pop_front()
                    };

                    if let Some(job) = job_opt {
                        let pool = {
                            let state = state.lock().unwrap();
                            state.pool.clone()
                        };

                        let prepared = {
                            let state = state.lock().unwrap();
                            state.prepared.clone()
                        };

                        // Update worker stats
                        {
                            let mut state = state.lock().unwrap();
                            if worker_id < state.workers.len() {
                                state.workers[worker_id].pending_count += 1;
                                state.workers[worker_id].last_activity = Instant::now();
                            }
                        }

                        let result = Self::execute_job(&pool, &prepared, job).await;

                        // Update worker stats and push result via callback
                        {
                            let mut state = state.lock().unwrap();
                            if worker_id < state.workers.len() {
                                state.workers[worker_id].pending_count =
                                    state.workers[worker_id].pending_count.saturating_sub(1);
                            }
                            state.output_queue.push_back(result);
                        }
                    } else {
                        // No job available, increment idle counter
                        let mut state = state.lock().unwrap();
                        if worker_id < state.workers.len() {
                            state.workers[worker_id].idle_calls += 1;
                        }
                    }
                }
            });
        });
    }

    async fn execute_job(
        pool: &Pool<Postgres>,
        prepared: &HashMap<DbStmt, String>,
        job: DbJob,
    ) -> DbResult {
        println!("Sent job for execution");
        if let Some(sql) = prepared.get(&job.stmt) {
            let query = sqlx::query(sql);

            // Bind parameters
            let mut query = query;
            for param in &job.params {
                query = query.bind(param);
            }
            println!("Got statement");

            match query.fetch_all(pool).await {
                Ok(rows) => DbResult {
                    id: job.id,
                    success: true,
                    stmt: job.stmt,
                    respond_to: job.respond_to,
                    rows: Some(Self::rows_to_strings(rows)),
                    message: None,
                },
                Err(e) => DbResult {
                    id: job.id,
                    success: false,
                    stmt: job.stmt,
                    respond_to: job.respond_to,
                    rows: None,
                    message: Some(e.to_string()),
                },
            }

        } else {
            DbResult {
                id: job.id,
                success: false,
                stmt: job.stmt,
                respond_to: job.respond_to,
                rows: None,
                message: Some("Unknown statement".into()),
            }
        }
    }

    fn rows_to_strings(rows: Vec<PgRow>) -> Vec<Vec<String>> {
        rows.iter()
            .map(|row| {
                (0..row.len())
                    .map(|i| {
                        row.try_get::<String, _>(i)
                            .unwrap_or_else(|_| "<NULL>".to_string())
                    })
                    .collect::<Vec<String>>()
            })
            .collect()
    }

    pub fn queue_job(&self, id: u64, stmt: DbStmt, params: Vec<String>, respond_to: GnsConnection) -> bool {
        println!("Entered queue");

        let job = DbJob { id, stmt, params, respond_to };

        if let Ok(mut state) = self.state.try_lock() {
            // Find worker with least pending jobs
            let min_pending_worker = state.workers.iter()
                .enumerate()
                .min_by_key(|(_, w)| w.pending_count)
                .map(|(idx, w)| (idx, w.pending_count));

            // Check if we need to spawn a new worker
            if let Some((_, min_pending)) = min_pending_worker {
                if min_pending >= state.max_pending_per_thread {
                    // Spawn new worker
                    let new_id = state.next_worker_id;
                    state.next_worker_id += 1;

                    state.workers.push(WorkerThread {
                        pending_count: 0,
                        idle_calls: 0,
                        last_activity: Instant::now(),
                    });

                    println!("Spawning new worker thread {}", new_id);
                    Self::spawn_worker_thread(self.state.clone(), new_id);
                }
            } else {
                // No workers, spawn first one
                state.workers.push(WorkerThread {
                    pending_count: 0,
                    idle_calls: 0,
                    last_activity: Instant::now(),
                });
                Self::spawn_worker_thread(self.state.clone(), 0);
            }

            // Reset idle calls for all workers when there's work
            for worker in &mut state.workers {
                worker.idle_calls = 0;
            }

            state.input_queue.push_back(job);
            println!("Awaited issue queue");
            true
        } else {
            // Could not acquire lock, return false
            false
        }
    }

    pub fn poll_result_sync(&self) -> Option<DbResult> {
        if let Ok(mut state) = self.state.try_lock() {
            state.output_queue.pop_front()
        } else {
            None
        }
    }
}