use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};
use gns::GnsConnection;
use password_hash::{PasswordHash, PasswordVerifier};
use argon2::Argon2;

#[derive(Debug)]
pub struct VerifyJob {
    pub id: u64,
    pub candidate: String,
    pub hash: String,
    pub respond_to: GnsConnection,
}

#[derive(Debug)]
pub struct VerifyResult {
    pub id: u64,
    pub ok: bool,
    pub respond_to: GnsConnection,
}

struct WorkerThread {
    pending_count: usize,
    idle_calls: usize,
    last_activity: Instant,
}

struct WorkerState {
    workers: Vec<WorkerThread>,
    max_pending_per_thread: usize,
    max_idle_calls: usize,
    next_worker_id: usize,
}

pub struct Argon2Worker {
    input_tx: Sender<VerifyJob>,
    input_rx: Arc<Mutex<Receiver<VerifyJob>>>,
    output_tx: Arc<Sender<VerifyResult>>,
    output_rx: Arc<Mutex<Receiver<VerifyResult>>>,
    state: Arc<Mutex<WorkerState>>,
}

impl Argon2Worker {
    pub fn new(parallelism: usize) -> Self {
        let (input_tx, input_rx) = channel::<VerifyJob>();
        let (output_tx, output_rx) = channel::<VerifyResult>();

        let input_rx = Arc::new(Mutex::new(input_rx));
        let output_tx = Arc::new(output_tx);

        let state = Arc::new(Mutex::new(WorkerState {
            workers: Vec::new(),
            max_pending_per_thread: 5,
            max_idle_calls: 25,
            next_worker_id: 0,
        }));

        // Spawn initial worker threads
        for i in 0..parallelism {
            {
                let mut state_lock = state.lock().unwrap();
                state_lock.workers.push(WorkerThread {
                    pending_count: 0,
                    idle_calls: 0,
                    last_activity: Instant::now(),
                });
            }
            Self::spawn_worker_thread(
                state.clone(),
                input_rx.clone(),
                output_tx.clone(),
                i
            );
        }

        Self {
            input_tx,
            input_rx: input_rx.clone(),
            output_tx: output_tx.clone(),
            output_rx: Arc::new(Mutex::new(output_rx)),
            state,
        }
    }

    fn spawn_worker_thread(
        state: Arc<Mutex<WorkerState>>,
        input_rx: Arc<Mutex<Receiver<VerifyJob>>>,
        output_tx: Arc<Sender<VerifyResult>>,
        worker_id: usize,
    ) {
        thread::spawn(move || {
            loop {
                let job_opt = {
                    let rx = input_rx.lock().unwrap();
                    // Use recv_timeout to avoid busy waiting but still check idle status
                    match rx.recv_timeout(Duration::from_millis(100)) {
                        Ok(job) => Some(job),
                        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => None,
                        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                            println!("Argon2 Worker {} shutting down (channel closed)", worker_id);
                            return;
                        }
                    }
                };

                if let Some(job) = job_opt {
                    // Update worker stats - got a job
                    {
                        let mut state = state.lock().unwrap();
                        if worker_id < state.workers.len() {
                            state.workers[worker_id].pending_count += 1;
                            state.workers[worker_id].last_activity = Instant::now();
                            state.workers[worker_id].idle_calls = 0;
                        }
                    }

                    // Do the actual password verification (CPU intensive)
                    let result = Self::verify_password(job);

                    // Send result back
                    let _ = output_tx.send(result);

                    // Update worker stats - job done
                    {
                        let mut state = state.lock().unwrap();
                        if worker_id < state.workers.len() {
                            state.workers[worker_id].pending_count =
                                state.workers[worker_id].pending_count.saturating_sub(1);
                        }
                    }
                } else {
                    // Timeout - no job available, check if should terminate
                    let should_terminate = {
                        let mut state = state.lock().unwrap();
                        if worker_id < state.workers.len() {
                            state.workers[worker_id].idle_calls += 1;

                            if state.workers[worker_id].idle_calls >= state.max_idle_calls
                                && state.workers.len() > 1 {
                                state.workers.remove(worker_id);
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    };

                    if should_terminate {
                        println!("Argon2 Worker {} terminated due to inactivity", worker_id);
                        return;
                    }
                }
            }
        });
    }

    fn verify_password(job: VerifyJob) -> VerifyResult {
        let VerifyJob { id, candidate, hash, respond_to } = job;

        let ok = match PasswordHash::new(&hash) {
            Ok(parsed) => {
                let argon2 = Argon2::default();
                argon2.verify_password(candidate.as_bytes(), &parsed).is_ok()
            }
            Err(_) => false,
        };

        VerifyResult { id, ok, respond_to }
    }

    pub fn queue_job(&self, candidate: impl Into<String>, hash: impl Into<String>, id: u64, respond_to: GnsConnection) -> bool {
        let job = VerifyJob {
            id,
            candidate: candidate.into(),
            hash: hash.into(),
            respond_to,
        };

        // Check if we need to spawn a new worker based on pending count
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

                    println!("Spawning new Argon2 worker thread {}", new_id);
                    Self::spawn_worker_thread(
                        self.state.clone(),
                        self.input_rx.clone(),
                        self.output_tx.clone(),
                        new_id
                    );
                }
            }

            // Reset idle calls for all workers when there's work
            for worker in &mut state.workers {
                worker.idle_calls = 0;
            }

            // Drop the lock before sending to avoid holding it during channel send
            drop(state);
        } else {
            // Could not acquire lock, but we can still try to queue the job
            // since the channel send itself is independent of the state lock
        }

        // Send job to workers (non-blocking from caller's perspective)
        // Return true if successfully sent, false if channel is full/disconnected
        self.input_tx.send(job).is_ok()
    }

    pub fn poll_result_sync(&self) -> Option<VerifyResult> {
        if let Ok(rx) = self.output_rx.try_lock() {
            match rx.try_recv() {
                Ok(result) => Some(result),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}