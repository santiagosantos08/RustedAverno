use raylib::prelude::*;

const MENU_GAP: f32 = 8.0;
const MENU_WIDTH: f32 = 400.0;
const MENU_ELEMENT_HEIGHT: f32 = 24.0;
const MENU_ELEMENT_FULLWIDTH: f32 = MENU_WIDTH - 2.0 * MENU_GAP;
const MENU_ELEMENT_FULLWIDTH_INSIDEGROUP: f32 = MENU_WIDTH - 4.0 * MENU_GAP;
const MENU_ELEMENT_HALFWIDTH: f32 = MENU_ELEMENT_FULLWIDTH / 2.0 - MENU_GAP / 2.0;
const MENU_ELEMENT_HALFWIDTH_INSIDEGROUP: f32 =
    MENU_ELEMENT_FULLWIDTH_INSIDEGROUP / 2.0 - MENU_GAP / 2.0;
const MENU_HEIGHT: f32 = MENU_ELEMENT_HEIGHT * 11.0 + 13.0 * MENU_GAP;

const HUD_GAP: f32 = 8.0;
const HUD_SQUARESIZE: f32 = 24.0;
const HUD_WIDTH: f32 = 13.0 * HUD_SQUARESIZE + 14.0 * HUD_GAP;
const HUD_HEIGHT: f32 = 3.0 * HUD_GAP + 2.0 * HUD_SQUARESIZE;
const HUD_BAR_WIDTH: f32 = 10.0 * HUD_SQUARESIZE + 9.0 * HUD_GAP;
const HUD_BAR_HEIGHT: f32 = HUD_GAP * 1.0;

enum SettingsElements {
    SettingsWindow,
    SettingsVideoGroup,
    SettingsControlsGroup,
    SettingsInterfaceGroup,
    SettingsAspectRatioToggle,
    SettingsWindowModeToggle,
    SettingsResolutionDropdown,
    SettingsAspectRatioLabel,
    SettingsWindowModeLabel,
    SettingsResolutionLabel,
    SettingsRetainCursorCheckbox,
    SettingsQuickCastCheckbox,
    SettingsCustomCursorCheckbox,
    SettingsAutoAttackCheckbox,
    SettingsMenuScaleToggle,
    SettingsMenuScaleLabel,
    SettingsIngameScaleToggle,
    SettingsIngameScaleLabel,
    SettingsApplyButton,
    SettingsDiscardButton,
    SettingsQuitButton,
    SettingsElementsCount,
}

enum LoginElements {
    LoginUsernameBox,
    LoginPasswordBox,
    LoginConnectButton,
    LoginElementsCount,
}

enum MainHudElements {
    MhBackground,
    MhSpell1,
    MhSpell2,
    MhSpell3,
    MhSpell4,
    MhSpell5,
    MhSpell6,
    MhSkill1,
    MhSkill2,
    MhSkill3,
    MhPassive,
    MhItem1,
    MhItem2,
    MhItem3,
    MhItem4,
    MhItem5,
    MhItem6,
    MhHpBar,
    MhResourceBar,
    MainHudElementsCount,
}

enum StatsBarElements {
    SbPhysDamage,
    SbMagicDamage,
    SbArmor,
    SbMagicResist,
    SbAttackSpeed,
    SbCooldownReduction,
    SbLuck,
    SbCritMult,
    SbMoveSpeed,
    SbHpRegen,
    SbResourceRegen,
    SbBuffMult,
    SbFlatArmorPen,
    SbPercArmorPen,
    SbFlatMagicPen,
    SbPercMagicPen,
    SbBasicLifeSteal,
    SbSkillLifeSteal,
    SbAttackRange,
    SbTenacity,
    SbGold,
    SbLevelAdvancement,
    SbElementsCount,
}

const SETTINGS_LAYOUT_RECTANGLES: [Rectangle; SettingsElements::SettingsElementsCount as usize] = [
    Rectangle::new(0.0, 0.0, MENU_WIDTH, MENU_HEIGHT),
    Rectangle::new(
        MENU_GAP,
        MENU_GAP + MENU_ELEMENT_HEIGHT,
        MENU_ELEMENT_FULLWIDTH,
        MENU_ELEMENT_HEIGHT * 3.0 + 4.0 * MENU_GAP,
    ),
    Rectangle::new(
        MENU_GAP,
        MENU_ELEMENT_HEIGHT * 4.0 + 6.0 * MENU_GAP,
        MENU_ELEMENT_FULLWIDTH,
        MENU_ELEMENT_HEIGHT * 2.0 + 3.0 * MENU_GAP,
    ),
    Rectangle::new(
        MENU_GAP,
        MENU_ELEMENT_HEIGHT * 6.0 + 10.0 * MENU_GAP,
        MENU_ELEMENT_FULLWIDTH,
        MENU_ELEMENT_HEIGHT * 2.0 + 3.0 * MENU_GAP,
    ),
    Rectangle::new(
        MENU_GAP * 2.0,
        MENU_ELEMENT_HEIGHT * 1.0 + 2.0 * MENU_GAP,
        MENU_ELEMENT_HALFWIDTH_INSIDEGROUP / 2.0 - 1.0,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 2.0,
        MENU_ELEMENT_HEIGHT * 2.0 + 3.0 * MENU_GAP,
        MENU_ELEMENT_HALFWIDTH_INSIDEGROUP / 2.0 - 1.0,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 2.0,
        MENU_ELEMENT_HEIGHT * 3.0 + 4.0 * MENU_GAP,
        MENU_ELEMENT_HALFWIDTH_INSIDEGROUP,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 2.0 + MENU_ELEMENT_HALFWIDTH,
        MENU_ELEMENT_HEIGHT * 1.0 + 2.0 * MENU_GAP,
        MENU_ELEMENT_HALFWIDTH_INSIDEGROUP,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 2.0 + MENU_ELEMENT_HALFWIDTH,
        MENU_ELEMENT_HEIGHT * 2.0 + 3.0 * MENU_GAP,
        MENU_ELEMENT_HALFWIDTH_INSIDEGROUP,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 2.0 + MENU_ELEMENT_HALFWIDTH,
        MENU_ELEMENT_HEIGHT * 3.0 + 4.0 * MENU_GAP,
        MENU_ELEMENT_HALFWIDTH_INSIDEGROUP,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 2.0,
        MENU_ELEMENT_HEIGHT * 4.0 + 7.0 * MENU_GAP,
        MENU_ELEMENT_HEIGHT,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 2.0,
        MENU_ELEMENT_HEIGHT * 5.0 + 8.0 * MENU_GAP,
        MENU_ELEMENT_HEIGHT,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 3.0 + MENU_ELEMENT_HALFWIDTH_INSIDEGROUP,
        MENU_ELEMENT_HEIGHT * 5.0 + 8.0 * MENU_GAP,
        MENU_ELEMENT_HEIGHT,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 3.0 + MENU_ELEMENT_HALFWIDTH_INSIDEGROUP,
        MENU_ELEMENT_HEIGHT * 4.0 + 7.0 * MENU_GAP,
        MENU_ELEMENT_HEIGHT,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 2.0,
        MENU_ELEMENT_HEIGHT * 7.0 + 8.0 * MENU_GAP,
        MENU_ELEMENT_HALFWIDTH_INSIDEGROUP / 6.0 - 2.0,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 3.0 + MENU_ELEMENT_HALFWIDTH_INSIDEGROUP,
        MENU_ELEMENT_HEIGHT * 7.0 + 8.0 * MENU_GAP,
        MENU_ELEMENT_HALFWIDTH_INSIDEGROUP,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 2.0,
        MENU_ELEMENT_HEIGHT * 8.0 + 9.0 * MENU_GAP,
        MENU_ELEMENT_HALFWIDTH_INSIDEGROUP / 6.0 - 2.0,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 3.0 + MENU_ELEMENT_HALFWIDTH_INSIDEGROUP,
        MENU_ELEMENT_HEIGHT * 8.0 + 9.0 * MENU_GAP,
        MENU_ELEMENT_HALFWIDTH_INSIDEGROUP,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        2.0 * MENU_GAP + MENU_ELEMENT_HALFWIDTH,
        MENU_ELEMENT_HEIGHT * 9.0 + 11.0 * MENU_GAP,
        MENU_ELEMENT_HALFWIDTH,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP,
        MENU_ELEMENT_HEIGHT * 9.0 + 11.0 * MENU_GAP,
        MENU_ELEMENT_HALFWIDTH,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP,
        MENU_ELEMENT_HEIGHT * 10.0 + 12.0 * MENU_GAP,
        MENU_ELEMENT_FULLWIDTH,
        MENU_ELEMENT_HEIGHT,
    ),
];

const LOGIN_LAYOUT_RECTANGLES: [Rectangle; LoginElements::LoginElementsCount as usize] = [
    Rectangle::new(
        MENU_GAP,
        MENU_GAP,
        MENU_ELEMENT_FULLWIDTH,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP,
        MENU_ELEMENT_HEIGHT + 2.0 * MENU_GAP,
        MENU_ELEMENT_FULLWIDTH,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP,
        2.0 * MENU_ELEMENT_HEIGHT + 3.0 * MENU_GAP,
        MENU_ELEMENT_FULLWIDTH,
        MENU_ELEMENT_HEIGHT,
    ),
];

const MAIN_HUD_LAYOUT_RECTANGLES: [Rectangle; MainHudElements::MainHudElementsCount as usize] = [
    Rectangle::new(0.0, 0.0, HUD_WIDTH, HUD_HEIGHT),
    Rectangle::new(
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        2.0 * HUD_GAP + 1.0 * HUD_SQUARESIZE,
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        3.0 * HUD_GAP + 2.0 * HUD_SQUARESIZE,
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        4.0 * HUD_GAP + 3.0 * HUD_SQUARESIZE,
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        5.0 * HUD_GAP + 4.0 * HUD_SQUARESIZE,
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        6.0 * HUD_GAP + 5.0 * HUD_SQUARESIZE,
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        7.0 * HUD_GAP + 6.0 * HUD_SQUARESIZE,
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        8.0 * HUD_GAP + 7.0 * HUD_SQUARESIZE,
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        9.0 * HUD_GAP + 8.0 * HUD_SQUARESIZE,
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        10.0 * HUD_GAP + 9.0 * HUD_SQUARESIZE,
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        11.0 * HUD_GAP + 10.0 * HUD_SQUARESIZE,
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        12.0 * HUD_GAP + 11.0 * HUD_SQUARESIZE,
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        13.0 * HUD_GAP + 12.0 * HUD_SQUARESIZE,
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        11.0 * HUD_GAP + 10.0 * HUD_SQUARESIZE,
        2.0 * HUD_GAP + 1.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        12.0 * HUD_GAP + 11.0 * HUD_SQUARESIZE,
        2.0 * HUD_GAP + 1.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        13.0 * HUD_GAP + 12.0 * HUD_SQUARESIZE,
        2.0 * HUD_GAP + 1.0 * HUD_SQUARESIZE,
        HUD_SQUARESIZE,
        HUD_SQUARESIZE,
    ),
    Rectangle::new(
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        4.0 * HUD_GAP + 1.0 * HUD_SQUARESIZE,
        HUD_BAR_WIDTH,
        HUD_BAR_HEIGHT,
    ),
    Rectangle::new(
        1.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        5.0 * HUD_GAP + 0.0 * HUD_SQUARESIZE,
        HUD_BAR_WIDTH,
        HUD_BAR_HEIGHT,
    ),
];

const MODAL_LAYOUT_RECTANGLES: [Rectangle; 3] = [
    Rectangle::new(
        0.0,
        0.0,
        MENU_GAP * 2.0 + MENU_WIDTH,
        MENU_GAP * 5.0 + 2.0 * MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 2.0,
        MENU_GAP * 2.0,
        MENU_ELEMENT_FULLWIDTH,
        MENU_ELEMENT_HEIGHT,
    ),
    Rectangle::new(
        MENU_GAP * 2.0,
        MENU_GAP * 3.0 + MENU_ELEMENT_HEIGHT,
        MENU_ELEMENT_FULLWIDTH,
        MENU_ELEMENT_HEIGHT,
    ),
];

#[derive(Debug, Clone)]
pub struct UiState {
    pub settings: bool,
    pub main_hud: bool,
    pub scoreboard: bool,
    pub chat: bool,
    pub fps_ping: bool,
    pub stats_bar: bool,
    pub mini_map: bool,
    pub full_map: bool,
    pub inventory: bool,
    pub loading_screen: bool,
    pub login_screen: bool,
    pub character_creation: bool,
    pub character_selection: bool,
    pub modal_message: bool,
    pub dragging: bool,
}

#[derive(Debug, Clone)]
pub struct LoginScreenState {
    pub anchor: Vector2,
    pub window_title: String,
    pub username_input_text: String,
    pub password_input_text: String,
    pub address_input_text: String,
    pub port_input_text: String,
    pub username_input_active: bool,
    pub password_input_active: bool,
    pub address_input_active: bool,
    pub port_input_active: bool,
    pub connect_button_title: String,
    pub connect_button_active: bool,
}

fn make_login_screen_state() -> LoginScreenState {
    LoginScreenState {
        anchor: Vector2::new(0.0, 0.0),
        window_title: "Iniciar Sesión".to_string(),
        username_input_text:
            "-------------------------Nombre de usuario----------------------------".to_string(),
        password_input_text:
            "-------------------------------Contraseña-------------------------------".to_string(),
        address_input_text: "".to_string(),
        port_input_text: "".to_string(),
        username_input_active: false,
        password_input_active: false,
        address_input_active: false,
        port_input_active: false,
        connect_button_title: "DESCENDER AL AVERNO".to_string(),
        connect_button_active: false,
    }
}

#[derive(Debug, Clone)]
pub struct MainHudState {
    pub anchor: Vector2,
    pub scale: f32,
    pub spell1_cd: f32,
    pub spell2_cd: f32,
    pub spell3_cd: f32,
    pub spell4_cd: f32,
    pub spell5_cd: f32,
    pub spell6_cd: f32,
    pub passive_cd: f32,
    pub skill1_cd: f32,
    pub skill2_cd: f32,
    pub skill3_cd: f32,
    pub item1_cd: f32,
    pub item2_cd: f32,
    pub item3_cd: f32,
    pub item4_cd: f32,
    pub item5_cd: f32,
    pub item6_cd: f32,
    pub hp_percent: f32,
    pub resource_percent: f32,
}

fn make_main_hud_state() -> MainHudState {
    MainHudState {
        anchor: Vector2::new(0.0, 0.0),
        scale: 1.5,
        spell1_cd: 0.0,
        spell2_cd: 0.0,
        spell3_cd: 0.0,
        spell4_cd: 0.0,
        spell5_cd: 0.0,
        spell6_cd: 0.0,
        passive_cd: 0.0,
        skill1_cd: 0.0,
        skill2_cd: 0.0,
        skill3_cd: 0.0,
        item1_cd: 0.0,
        item2_cd: 0.0,
        item3_cd: 0.0,
        item4_cd: 0.0,
        item5_cd: 0.0,
        item6_cd: 0.0,
        hp_percent: 0.0,
        resource_percent: 0.0,
    }
}

#[derive(Debug, Clone)]
pub struct StatsBarState {
    pub anchor: Vector2,
    pub p_damage: f32,
    pub m_damage: f32,
    pub armor: f32,
    pub m_resist: f32,
    pub attack_speed: f32,
    pub cdr: f32,
    pub luck: f32,
    pub crit_mult: f32,
    pub move_speed: f32,
    pub hp_regen: f32,
    pub resource_regen: f32,
    pub buff_mult: f32,
    pub flat_armor_pen: f32,
    pub perc_armor_pen: f32,
    pub flat_magic_pen: f32,
    pub perc_magic_pen: f32,
    pub basic_life_steal: f32,
    pub skill_life_steal: f32,
    pub attack_range: f32,
    pub tenacity: f32,
    pub gold: f32,
    pub level_advancement: f32,
    pub scale: f32,
}

fn make_stats_bar_state() -> StatsBarState {
    StatsBarState {
        anchor: Vector2::new(0.0, 0.0),
        p_damage: 0.0,
        m_damage: 0.0,
        armor: 0.0,
        m_resist: 0.0,
        attack_speed: 0.0,
        cdr: 0.0,
        luck: 0.0,
        crit_mult: 0.0,
        move_speed: 0.0,
        hp_regen: 0.0,
        resource_regen: 0.0,
        buff_mult: 0.0,
        flat_armor_pen: 0.0,
        perc_armor_pen: 0.0,
        flat_magic_pen: 0.0,
        perc_magic_pen: 0.0,
        basic_life_steal: 0.0,
        skill_life_steal: 0.0,
        attack_range: 0.0,
        tenacity: 0.0,
        gold: 0.0,
        level_advancement: 0.0,
        scale: 2.0,
    }
}

#[derive(Debug, Clone)]
pub struct SettingsState {
    pub anchor: Vector2,
    pub resolution_dropdown_active: bool,
    pub resolution_dropdown_option: i32,
    pub aspect_ratio_toggle_option: i32,
    pub window_mode_toggle_option: i32,
    pub retain_cursor_checkbox: bool,
    pub custom_cursor_checkbox: bool,
    pub quick_cast_checkbox: bool,
    pub autoattack_checkbox: bool,
    pub in_game_scale_toggle_option: i32,
    pub menu_scale_toggle_option: i32,
    pub settings_window_title: String,
    pub aspect_ratio_toggle_options: String,
    pub resolution_dropdown_options: String,
    pub window_mode_toggle_options: String,
    pub retain_cursor_checkbox_title: String,
    pub custom_cursor_checkbox_title: String,
    pub video_group_title: String,
    pub window_mode_title: String,
    pub aspect_ratio_title: String,
    pub resolution_title: String,
    pub controls_group_title: String,
    pub quick_cast_checkbox_title: String,
    pub auto_attack_checkbox_title: String,
    pub interface_group_title: String,
    pub ingame_interface_scale_options: String,
    pub ingame_interface_scale_title: String,
    pub menu_scale_options: String,
    pub menu_scale_title: String,
    pub discard_button_title: String,
    pub apply_button_title: String,
    pub quit_button_title: String,
}

fn make_settings_state() -> SettingsState {
    SettingsState {
        anchor: Vector2::new(10.0, 10.0),
        resolution_dropdown_active: false,
        retain_cursor_checkbox: false,
        custom_cursor_checkbox: false,
        quick_cast_checkbox: false,
        autoattack_checkbox: false,
        resolution_dropdown_option: 0,
        aspect_ratio_toggle_option: 0,
        window_mode_toggle_option: 0,
        in_game_scale_toggle_option: 0,
        menu_scale_toggle_option: 0,
        settings_window_title: "Ajustes".to_string(),
        aspect_ratio_toggle_options: "16:9;4:3".to_string(),
        resolution_dropdown_options: "360p;720p;1080p;2k;1800p;4k".to_string(),
        window_mode_toggle_options: "Sin bordes;Ventana".to_string(),
        retain_cursor_checkbox_title: "Retener Cursor".to_string(),
        custom_cursor_checkbox_title: "Cursor personalizado".to_string(),
        video_group_title: "Video".to_string(),
        window_mode_title: "Modo de ventana".to_string(),
        aspect_ratio_title: "Relación de aspecto".to_string(),
        resolution_title: "Resolución".to_string(),
        controls_group_title: "Controles".to_string(),
        quick_cast_checkbox_title: "Casteo Rápido".to_string(),
        auto_attack_checkbox_title: "Auto Ataque".to_string(),
        interface_group_title: "Interfaz".to_string(),
        ingame_interface_scale_options: "x1;x2;x3;x4;x5;x6".to_string(),
        ingame_interface_scale_title: "Escala in-game".to_string(),
        menu_scale_options: "x1;x2;x3;x4;x5;x6".to_string(),
        menu_scale_title: "Escala de interfaz".to_string(),
        discard_button_title: "DESCARTAR".to_string(),
        apply_button_title: "APLICAR".to_string(),
        quit_button_title: "Salir del AVERNO".to_string(),
    }
}

pub struct ClientUi {
    pub ui_state: UiState,
    pub current_settings: SettingsState,
    pub current_login: LoginScreenState,
    pub current_hud_state: MainHudState,
    pub current_stats_bar: StatsBarState,
    pub modal_message: String,
    should_apply: bool,
    should_discard: bool,
    should_quit: bool,
}

impl ClientUi {
    pub fn new() -> Self {
        ClientUi {
            ui_state: UiState {
                settings: false,
                main_hud: false,
                scoreboard: false,
                chat: false,
                fps_ping: false,
                stats_bar: false,
                mini_map: false,
                full_map: false,
                inventory: false,
                loading_screen: false,
                login_screen: false,
                character_creation: false,
                character_selection: false,
                modal_message: false,
                dragging: false,
            },
            current_settings: make_settings_state(),
            current_login: make_login_screen_state(),
            current_hud_state: make_main_hud_state(),
            current_stats_bar: make_stats_bar_state(),
            modal_message: String::new(),
            should_apply: false,
            should_discard: false,
            should_quit: false,
        }
    }

    fn move_rectangle_to(rectangle: Rectangle, pos: Vector2) -> Rectangle {
        Rectangle {
            x: rectangle.x + pos.x,
            y: rectangle.y + pos.y,
            width: rectangle.width,
            height: rectangle.height,
        }
    }

    fn scale_and_move_rectangle(rectangle: Rectangle, scale: f32, pos: Vector2) -> Rectangle {
        Rectangle {
            x: rectangle.x * scale + pos.x,
            y: rectangle.y * scale + pos.y,
            width: rectangle.width * scale,
            height: rectangle.height * scale,
        }
    }

    pub fn load_theme(&self, rl: &mut RaylibHandle, path: &str) {
        rl.gui_load_style(path);
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.should_apply = false;
        self.should_discard = false;
        self.should_quit = false;

        let screen_width = rl.get_screen_width() as f32;
        let screen_height = rl.get_screen_height() as f32;

        self.current_hud_state.anchor = Vector2::new(
            screen_width / 2.0 - HUD_WIDTH * self.current_hud_state.scale / 2.0,
            screen_height - HUD_HEIGHT * self.current_hud_state.scale,
        );

        self.current_stats_bar.anchor = Vector2::new(
            0.0,
            screen_height / 2.0
                - (HUD_BAR_HEIGHT * f32::from(StatsBarElements::SbElementsCount as u16))
                    * self.current_stats_bar.scale
                    / 2.0,
        );

        let mouse_pos = rl.get_mouse_position();
        let mouse_delta = rl.get_mouse_delta();

        let settings_rect = ClientUi::move_rectangle_to(
            SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsWindow as usize],
            self.current_settings.anchor,
        );

        // Only drag if clicking the title bar (approximated)
        let title_bar_rect = Rectangle::new(
            settings_rect.x,
            settings_rect.y,
            settings_rect.width,
            MENU_ELEMENT_HEIGHT + MENU_GAP,
        );
        // Handle settings window dragging
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            if title_bar_rect.check_collision_point_rec(mouse_pos) {
                self.ui_state.dragging = true;
            }
        } else if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            // Stop the drag, no matter where the mouse is
            self.ui_state.dragging = false;
        }

        if self.ui_state.dragging {
            self.current_settings.anchor += mouse_delta;
        }

        // Clamp settings window to screen
        if self.current_settings.anchor.x < 0.0 {
            self.current_settings.anchor.x = 0.0;
        }
        if self.current_settings.anchor.y < 0.0 {
            self.current_settings.anchor.y = 0.0;
        }
        if self.current_settings.anchor.x + MENU_WIDTH > screen_width {
            self.current_settings.anchor.x = screen_width - MENU_WIDTH;
        }
        if self.current_settings.anchor.y + MENU_HEIGHT > screen_height {
            self.current_settings.anchor.y = screen_height - MENU_HEIGHT;
        }
    }

    /// Port of ClientUi::hideAllUi()
    pub fn hide_all_ui(&mut self) {
        self.ui_state.settings = false;
        self.ui_state.main_hud = false;
        self.ui_state.scoreboard = false;
        self.ui_state.chat = false;
        self.ui_state.fps_ping = false;
        self.ui_state.stats_bar = false;
        self.ui_state.mini_map = false;
        self.ui_state.full_map = false;
        self.ui_state.inventory = false;
        self.ui_state.loading_screen = false;
        self.ui_state.login_screen = false;
        self.ui_state.character_creation = false;
        self.ui_state.character_selection = false;
        // C++ code comments: "TODO hide modal??? no se que convendria"
        // We will leave modal state as-is, following the C++ behavior.
        // self.ui_state.modal_message = false;
    }

    /// Port of ClientUi::draw()
    /// Note: Needs &mut self to mutate state from GUI interactions.
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        if self.ui_state.main_hud {
            self.draw_main_hud(d);
        }
        if self.ui_state.scoreboard {
            self.draw_scoreboard(d);
        }
        if self.ui_state.chat {
            self.draw_chat(d);
        }
        if self.ui_state.fps_ping {
            self.draw_fps_ping(d);
        }
        if self.ui_state.stats_bar {
            self.draw_stats_bar(d);
        }
        if self.ui_state.mini_map {
            self.draw_mini_map(d);
        }
        if self.ui_state.full_map {
            self.draw_full_map(d);
        }
        if self.ui_state.inventory {
            self.draw_inventory(d);
        }
        if self.ui_state.loading_screen {
            self.draw_loading_screen(d);
        }
        if self.ui_state.login_screen {
            self.draw_login_screen(d);
        }
        if self.ui_state.character_creation {
            self.draw_character_creation_screen(d);
        }
        if self.ui_state.character_selection {
            self.draw_character_selection_screen(d);
        }
        if self.ui_state.settings {
            self.draw_settings(d);
        }
        if self.ui_state.modal_message {
            self.draw_modal_popup(d);
        }
    }

    // --- State Management Functions (ported) ---

    /// Port of ClientUi::getSettings()
    pub fn get_settings(&self) -> &SettingsState {
        &self.current_settings
    }

    /// Port of ClientUi::setSettings(SettingsState s)
    pub fn set_settings(&mut self, s: SettingsState) {
        self.current_settings = s;
    }

    /// Port of ClientUi::shouldApplySettings()
    pub fn should_apply_settings(&mut self) -> bool {
        if self.should_apply {
            self.should_apply = false;
            return true;
        }
        false
    }

    /// Port of ClientUi::shouldDiscardSettings()
    pub fn should_discard_settings(&self) -> bool {
        false // TODO
    }

    /// Port of ClientUi::shouldQuit()
    pub fn should_quit(&self) -> bool {
        false // TODO
    }

    /// Port of ClientUi::getLoginData()
    pub fn get_login_data(&self) -> &LoginScreenState {
        &self.current_login
    }

    /// Port of ClientUi::shouldAttemptLogin()
    pub fn should_attempt_login(&mut self) -> bool {
        if self.current_login.connect_button_active {
            self.current_login.connect_button_active = false;
            return true;
        }
        false
    }

    /// Port of ClientUi::toggleSettingsMenu()
    pub fn toggle_settings_menu(&mut self) {
        self.ui_state.settings = !self.ui_state.settings;
    }

    /// Port of ClientUi::setLoginFeedbackMessage(std::string newStr)
    pub fn set_login_feedback_message(&mut self, new_str: String) {
        self.current_login.connect_button_title = new_str;
    }

    /// Port of ClientUi::activateModalPopup(std::string message)
    pub fn activate_modal_popup(&mut self, message: String) {
        self.ui_state.modal_message = true;
        self.modal_message = message;
    }

    // --- Private Draw Functions (ported) ---

    /// Port of ClientUi::drawSettings()
    /// This is the correct, functional raygui implementation.
    fn draw_settings(&mut self, d: &mut RaylibDrawHandle) {
        if self.current_settings.resolution_dropdown_active {
            d.gui_lock();
        }

        let window_rect = ClientUi::move_rectangle_to(
            SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsWindow as usize],
            self.current_settings.anchor,
        );
        // GuiWindowBox returns false when the close button is pressed
        self.ui_state.settings =
            !d.gui_window_box(window_rect, &self.current_settings.settings_window_title);

        d.gui_toggle_group(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsWindowModeToggle as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.window_mode_toggle_options,
            &mut self.current_settings.window_mode_toggle_option,
        );

        d.gui_toggle_group(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsAspectRatioToggle as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.aspect_ratio_toggle_options,
            &mut self.current_settings.aspect_ratio_toggle_option,
        );

        d.gui_check_box(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsRetainCursorCheckbox as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.retain_cursor_checkbox_title,
            &mut self.current_settings.retain_cursor_checkbox,
        );

        d.gui_check_box(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsAutoAttackCheckbox as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.auto_attack_checkbox_title,
            &mut self.current_settings.autoattack_checkbox,
        );

        d.gui_group_box(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsControlsGroup as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.controls_group_title,
        );

        d.gui_label(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsWindowModeLabel as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.window_mode_title,
        );

        d.gui_label(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsResolutionLabel as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.resolution_title,
        );

        d.gui_label(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsAspectRatioLabel as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.aspect_ratio_title,
        );

        d.gui_group_box(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsInterfaceGroup as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.interface_group_title,
        );

        d.gui_check_box(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsCustomCursorCheckbox as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.custom_cursor_checkbox_title,
            &mut self.current_settings.custom_cursor_checkbox,
        );

        d.gui_check_box(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsQuickCastCheckbox as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.quick_cast_checkbox_title,
            &mut self.current_settings.quick_cast_checkbox,
        );

        d.gui_group_box(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsVideoGroup as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.video_group_title,
        );

        d.gui_toggle_group(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsIngameScaleToggle as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.ingame_interface_scale_options,
            &mut self.current_settings.in_game_scale_toggle_option,
        );

        d.gui_label(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsIngameScaleLabel as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.ingame_interface_scale_title,
        );

        d.gui_toggle_group(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsMenuScaleToggle as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.menu_scale_options,
            &mut self.current_settings.menu_scale_toggle_option,
        );

        d.gui_label(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsMenuScaleLabel as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.menu_scale_title,
        );

        if d.gui_dropdown_box(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsResolutionDropdown as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.resolution_dropdown_options,
            &mut self.current_settings.resolution_dropdown_option,
            self.current_settings.resolution_dropdown_active,
        ) {
            self.current_settings.resolution_dropdown_active =
                !self.current_settings.resolution_dropdown_active;
        }

        if d.gui_button(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsApplyButton as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.apply_button_title,
        ) {
            self.should_apply = true;
        }

        if d.gui_button(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsDiscardButton as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.discard_button_title,
        ) {
            self.should_discard = true;
        }

        if d.gui_button(
            ClientUi::move_rectangle_to(
                SETTINGS_LAYOUT_RECTANGLES[SettingsElements::SettingsQuitButton as usize],
                self.current_settings.anchor,
            ),
            &self.current_settings.quit_button_title,
        ) {
            self.should_quit = true;
        }

        d.gui_unlock();
    }

    /// Port of ClientUi::drawModalPopup()
    fn draw_modal_popup(&mut self, d: &mut RaylibDrawHandle) {
        let screen_width = d.get_screen_width() as f32;
        let screen_height = d.get_screen_height() as f32;

        let anchor = Vector2::new(
            screen_width / 2.0 - MODAL_LAYOUT_RECTANGLES[0].width / 2.0,
            screen_height / 2.0 - MODAL_LAYOUT_RECTANGLES[0].height / 2.0 - 100.0,
        );

        d.gui_panel(
            ClientUi::move_rectangle_to(MODAL_LAYOUT_RECTANGLES[0], anchor),
            "",
        );
        d.gui_label(
            ClientUi::move_rectangle_to(MODAL_LAYOUT_RECTANGLES[1], anchor),
            &self.modal_message,
        );

        if d.gui_button(
            ClientUi::move_rectangle_to(MODAL_LAYOUT_RECTANGLES[2], anchor),
            "OK",
        ) {
            self.ui_state.modal_message = false;
        }
    }

    /// Port of ClientUi::drawMainHud()
    fn draw_main_hud(&mut self, d: &mut RaylibDrawHandle) {
        let font_size = 10.0;
        let anchor = self.current_hud_state.anchor;
        let scale = self.current_hud_state.scale;

        // Background
        let r = ClientUi::scale_and_move_rectangle(
            MAIN_HUD_LAYOUT_RECTANGLES[MainHudElements::MhBackground as usize],
            scale,
            anchor,
        );
        d.draw_rectangle_rec(r, Color::RAYWHITE);

        // Helper closure to draw ability slot
        let draw_ability = |d: &mut RaylibDrawHandle, idx: usize, key: &str, cd: &mut f32| {
            let r =
                ClientUi::scale_and_move_rectangle(MAIN_HUD_LAYOUT_RECTANGLES[idx], scale, anchor);
            // GuiProgressBar draws a background, so no separate panel is needed
            d.gui_progress_bar(r, "", "", cd, 0.0, 1.0);
            d.draw_text(
                key,
                (r.x + HUD_GAP) as i32,
                (r.y + HUD_GAP) as i32,
                font_size as i32,
                Color::DARKBLUE,
            );
        };

        // Spells (QWERTY)
        draw_ability(
            d,
            MainHudElements::MhSpell1 as usize,
            "Q",
            &mut self.current_hud_state.spell1_cd,
        );
        draw_ability(
            d,
            MainHudElements::MhSpell2 as usize,
            "W",
            &mut self.current_hud_state.spell2_cd,
        );
        draw_ability(
            d,
            MainHudElements::MhSpell3 as usize,
            "E",
            &mut self.current_hud_state.spell3_cd,
        );
        draw_ability(
            d,
            MainHudElements::MhSpell4 as usize,
            "R",
            &mut self.current_hud_state.spell4_cd,
        );
        draw_ability(
            d,
            MainHudElements::MhSpell5 as usize,
            "T",
            &mut self.current_hud_state.spell5_cd,
        );
        draw_ability(
            d,
            MainHudElements::MhSpell6 as usize,
            "Y",
            &mut self.current_hud_state.spell6_cd,
        );

        // Skills (DFG)
        draw_ability(
            d,
            MainHudElements::MhSkill1 as usize,
            "D",
            &mut self.current_hud_state.skill1_cd,
        );
        draw_ability(
            d,
            MainHudElements::MhSkill2 as usize,
            "F",
            &mut self.current_hud_state.skill2_cd,
        );
        draw_ability(
            d,
            MainHudElements::MhSkill3 as usize,
            "G",
            &mut self.current_hud_state.skill3_cd,
        );

        // Passive
        draw_ability(
            d,
            MainHudElements::MhPassive as usize,
            "PAS",
            &mut self.current_hud_state.passive_cd,
        );

        // Items (1-6)
        draw_ability(
            d,
            MainHudElements::MhItem1 as usize,
            "1",
            &mut self.current_hud_state.item1_cd,
        );
        draw_ability(
            d,
            MainHudElements::MhItem2 as usize,
            "2",
            &mut self.current_hud_state.item2_cd,
        );
        draw_ability(
            d,
            MainHudElements::MhItem3 as usize,
            "3",
            &mut self.current_hud_state.item3_cd,
        );
        draw_ability(
            d,
            MainHudElements::MhItem4 as usize,
            "4",
            &mut self.current_hud_state.item4_cd,
        );
        draw_ability(
            d,
            MainHudElements::MhItem5 as usize,
            "5",
            &mut self.current_hud_state.item5_cd,
        );
        draw_ability(
            d,
            MainHudElements::MhItem6 as usize,
            "6",
            &mut self.current_hud_state.item6_cd,
        );

        // Bars
        let r_hp = ClientUi::scale_and_move_rectangle(
            MAIN_HUD_LAYOUT_RECTANGLES[MainHudElements::MhHpBar as usize],
            scale,
            anchor,
        );
        d.gui_progress_bar(
            r_hp,
            "",
            "",
            &mut self.current_hud_state.hp_percent,
            0.0,
            1.0,
        );
        d.draw_text(
            "HP",
            (r_hp.x + HUD_GAP) as i32,
            r_hp.y as i32,
            font_size as i32,
            Color::DARKBLUE,
        );

        let r_res = ClientUi::scale_and_move_rectangle(
            MAIN_HUD_LAYOUT_RECTANGLES[MainHudElements::MhResourceBar as usize],
            scale,
            anchor,
        );
        d.gui_progress_bar(
            r_res,
            "",
            "",
            &mut self.current_hud_state.resource_percent,
            0.0,
            1.0,
        );
        d.draw_text(
            "MANA",
            (r_res.x + HUD_GAP) as i32,
            r_res.y as i32,
            font_size as i32,
            Color::DARKBLUE,
        );
    }

    /// Port of ClientUi::drawLoginScreen()
    fn draw_login_screen(&mut self, d: &mut RaylibDrawHandle) {
        let screen_width = d.get_screen_width() as f32;
        let screen_height = d.get_screen_height() as f32;

        self.current_login.anchor = Vector2::new(
            (screen_width - MENU_WIDTH) / 2.0,
            (screen_height - 5.0 * MENU_ELEMENT_HEIGHT + 6.0 * MENU_GAP) / 2.0,
        );

        let username_rect = ClientUi::move_rectangle_to(
            LOGIN_LAYOUT_RECTANGLES[LoginElements::LoginUsernameBox as usize],
            self.current_login.anchor,
        );

        // gui_text_box returns bool for focus change

        if d.gui_text_box(
            username_rect,
            &mut self.current_login.username_input_text,
            self.current_login.username_input_active,
        ) {
            self.current_login.username_input_active = !self.current_login.username_input_active;
        }

        // Password Box
        let password_rect = ClientUi::move_rectangle_to(
            LOGIN_LAYOUT_RECTANGLES[LoginElements::LoginPasswordBox as usize],
            self.current_login.anchor,
        );

        if d.gui_text_box(
            password_rect,
            &mut self.current_login.password_input_text,
            self.current_login.password_input_active,
        ) {
            self.current_login.password_input_active = !self.current_login.password_input_active;
        }

        // Connect Button
        // Using the fixed layout rect that matches the C++ implementation
        let connect_rect = ClientUi::move_rectangle_to(
            LOGIN_LAYOUT_RECTANGLES[LoginElements::LoginConnectButton as usize],
            self.current_login.anchor,
        );
        if d.gui_button(connect_rect, &self.current_login.connect_button_title) {
            self.current_login.connect_button_active = true;
        }
    }

    // --- Empty Stubs (ported) ---

    fn draw_scoreboard(&self, d: &mut RaylibDrawHandle) {
        // C++ implementation is empty
    }

    fn draw_chat(&self, d: &mut RaylibDrawHandle) {
        // C++ implementation is empty
    }

    fn draw_fps_ping(&self, d: &mut RaylibDrawHandle) {
        d.draw_fps(0, 0); // Ported from C++
    }

    fn draw_stats_bar(&self, d: &mut RaylibDrawHandle) {
        // C++ implementation is empty
        let total_panel_height =
            (HUD_BAR_HEIGHT * f32::from(StatsBarElements::SbElementsCount as u16));
        d.draw_rectangle_rec(
            Self::scale_and_move_rectangle(
                Rectangle::new(0.0, 0.0, 50.0, total_panel_height),
                self.current_stats_bar.scale,
                self.current_stats_bar.anchor,
            ),
            Color::RAYWHITE,
        );
        for i in 0..StatsBarElements::SbElementsCount as u16 {
            let r = Rectangle::new(5.0, (f32::from(i) * (HUD_BAR_HEIGHT)), 40.0, HUD_BAR_HEIGHT);
            d.gui_label(
                Self::scale_and_move_rectangle(
                    r,
                    self.current_stats_bar.scale,
                    self.current_stats_bar.anchor,
                ),
                "STAT .... VAL",
            );
        }
    }

    fn draw_mini_map(&self, _d: &mut RaylibDrawHandle) {
        // C++ implementation is empty
    }

    fn draw_full_map(&self, _d: &mut RaylibDrawHandle) {
        // C++ implementation is empty
    }

    fn draw_inventory(&self, _d: &mut RaylibDrawHandle) {
        // C++ implementation is empty
    }

    fn draw_loading_screen(&self, _d: &mut RaylibDrawHandle) {
        // C++ implementation is empty
    }

    fn draw_character_creation_screen(&self, _d: &mut RaylibDrawHandle) {
        // C++ implementation is empty
    }

    fn draw_character_selection_screen(&self, _d: &mut RaylibDrawHandle) {
        // C++ implementation is empty
    }
}

impl Default for ClientUi {
    fn default() -> Self {
        Self::new()
    }
}
