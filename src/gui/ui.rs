use log::LevelFilter;
use notan::egui::{self, *};
use notan::math::{Mat3, Vec2};
use notan::prelude::*;
use notan::{draw::*, extra};
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime};
use device_query::{DeviceQuery, DeviceState, Keycode};

use winapi::um::winuser::{
    SetWindowLongW, GWL_EXSTYLE, GWL_STYLE, WS_BORDER, WS_CAPTION, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_EX_ACCEPTFILES,
    WS_EX_LAYERED, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_EX_WINDOWEDGE, WS_MINIMIZEBOX, WS_SYSMENU, WS_VISIBLE,
};

use crate::gui::draw_item_log::clear_item_log;
use crate::memory::instance_manager::{get_process_pid_and_window_handle, WindowInfo};
use crate::types::buffs::check_buff_timers;
use crate::gui::draw_map::draw_map;
use crate::gui::Fonts;
use crate::mapgeneration::blacha::is_blacha_ok;
use crate::memory::gamedata;
use crate::memory::process::D2RWindowArea;
use crate::settings::MapPosition;
use crate::types::item_filter::ItemFilters;
use crate::LOCALISATION;
use crate::{
    mapgeneration::{self, jsondata::SeedData},
    memory::{gamedata::GameData, process::D2RInstance},
    settings::Settings,
};
use winapi::shared::windef::{HWND, POINT};

use super::draw_buff_bar::draw_buff_bar;
use super::draw_item_log::draw_item_log;
use super::draw_item_tooltip::draw_item_tooltip;
use super::draw_lines::draw_lines;
use super::draw_objects::draw_objects;
use super::draw_party_info::draw_party_info;
use super::draw_path::draw_pathfinding;
use super::draw_presets::draw_presets;
use super::draw_units::draw_units;
use super::egui::{create_egui_panel, create_language_select_ui};
use super::images;
use super::util::get_attached_levels;

#[notan_main]
pub fn start_ui() -> Result<(), String> {
    // load config
    let mut settings: Settings = match Settings::new() {
        Ok(settings) => settings,
        Err(err) => panic!("Error reading from settings file {}", err),
    };
    settings.detect_locale();

    let win_config = WindowConfig::default()
        .set_size(10, 10)
        .set_always_on_top(settings.general.overlay_mode)
        .set_decorations(!settings.general.overlay_mode)
        .set_mouse_passthrough(settings.general.overlay_mode)
        .set_transparent(settings.general.overlay_mode)
        .set_resizable(!settings.general.overlay_mode)
        .set_multisampling(settings.general.multisampling)
        .set_window_icon(Some("primemh.png".into()))
        .set_taskbar_icon(Some("primemh.png".into()))
        .set_title("PrimeMH")
        .set_high_dpi(settings.general.high_dpi)
        .set_vsync(settings.general.vsync);

    let result = notan::init_with(init)
        .add_config(win_config)
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .add_plugin(extra::FpsLimit::new(settings.general.fps_limit))
        .update(update)
        .draw(draw)
        .build();

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            settings.general.multisampling = 0;
            settings.save();
            log::error!("Initialization with multisampling failed\nTry launching again\n{}", e);
            Err(e)
        }
    }
}

fn init(gfx: &mut Graphics) -> State {
    // load config
    let settings: Settings = match Settings::new() {
        Ok(settings) => settings,
        Err(err) => {
            let localisation = LOCALISATION.lock().unwrap();
            panic!("{}\n{}", localisation.get_primemh("error5"), err)
        }
    };
    log::info!("Loaded settings file");

    {
        let mut localisation = LOCALISATION.lock().unwrap();
        localisation.update_locale(settings.general.language.clone());
    }
    log::info!("Loaded localisation for {:?}", &settings.general.language);

    let _blacha_result = is_blacha_ok(&settings).unwrap();
    log::info!("D2LoD test passed, map generation ok");

    let images = images::load_images(gfx);
    log::info!("Loaded images");

    let item_filters = match ItemFilters::load() {
        Some(item_filters) => item_filters,
        None => {
            let localisation = LOCALISATION.lock().unwrap();
            panic!("{}", localisation.get_primemh("error6"))
        }
    };

    let windows: Vec<WindowInfo> = get_process_pid_and_window_handle();
    log::info!("Found {} D2R windows {:?}", windows.len(), windows);
    if windows.len() == 0 {
        let localisation = LOCALISATION.lock().unwrap();
        log::error!("{}", localisation.get_primemh("error12"))
    }
    let d2rinstances: Vec<D2RInstance> = windows.iter().map(|window| D2RInstance::new(&window)).collect();

    let exocet_font = gfx.create_font(include_bytes!("./fonts/exocet.otf")).expect("Could not load exocet font!");
    let formal_font = gfx.create_font(include_bytes!("./fonts/formal.otf")).expect("Could not load formal font!");
    let korean_font = gfx
        .create_font(include_bytes!("./fonts/NotoSansCJKkr-Regular.otf"))
        .expect("Could not load korean_font font!");
    let taiwan_font = gfx
        .create_font(include_bytes!("./fonts/NotoSansCJKtc-Regular.otf"))
        .expect("Could not load taiwan_font font!");
    let blizzard_font = gfx
        .create_font(include_bytes!("./fonts/blizzardglobaltcunicode.ttf"))
        .expect("Could not load blizzard_font font!");

    let fonts = Fonts {
        exocet_font,
        formal_font,
        korean_font,
        taiwan_font,
        blizzard_font,
    };

    log::info!("Loaded fonts");

    let seed_data = SeedData::default();

    log::info!("Started UI successfully");

    let texture = gfx
            .create_texture()
            .from_image(include_bytes!("images/translate.png"))
            .with_premultiplied_alpha()
            .build()
            .unwrap();

    let language_icon = gfx.egui_register_texture(&texture);

    let texture = gfx
            .create_texture()
            .from_image(include_bytes!("images/unlocked.png"))
            .with_premultiplied_alpha()
            .build()
            .unwrap();
    let unlocked_icon = gfx.egui_register_texture(&texture);

    let texture = gfx
            .create_texture()
            .from_image(include_bytes!("images/locked.png"))
            .with_premultiplied_alpha()
            .build()
            .unwrap();
    let locked_icon = gfx.egui_register_texture(&texture);
    let last_map_opacity = settings.visual.map_opacity.clone();

    State {
        d2rinstances,
        settings,
        seed_data,
        last_seed: 0,
        game_data: None,
        fonts,
        images,
        item_filters,
        item_frame: 0,
        egui_rect: Rect {
            min: Pos2::ZERO,
            max: Pos2::ZERO,
        },
        egui_hovering: false,
        relative_mouse_pos: (0, 0),
        launch_time: SystemTime::now(),
        ui_panel_visible: false,
        ui_panel_toggle: false,
        map_overlay_visible: true,
        map_overlay_toggle: false,
        language_selector_visible: false,
        language_icon,
        unlocked_icon,
        locked_icon,
        last_map_opacity,
        checked: false,
        instance_locked: None,

    }
}

#[derive(AppState)]
pub(crate) struct State {
    pub d2rinstances: Vec<D2RInstance>,
    pub settings: Settings,
    pub seed_data: SeedData,
    pub last_seed: u32,
    pub game_data: Option<GameData>,
    pub fonts: Fonts,
    pub images: HashMap<String, Texture>,
    pub item_filters: ItemFilters,
    pub item_frame: i32,
    pub egui_rect: Rect,
    pub egui_hovering: bool,
    pub relative_mouse_pos: (i32, i32),
    pub launch_time: SystemTime,
    pub ui_panel_visible: bool,
    pub ui_panel_toggle: bool,
    pub map_overlay_visible: bool,
    pub map_overlay_toggle: bool,
    pub language_selector_visible: bool,
    pub language_icon: egui::SizedTexture,
    pub unlocked_icon: egui::SizedTexture,
    pub locked_icon: egui::SizedTexture,
    pub last_map_opacity: f32,
    pub checked: bool,
    pub instance_locked: Option<HWND>,
}

fn update(app: &mut App, state: &mut State) {

    let instance_locked = state.instance_locked.clone();
    let d2rprocess = state.d2rinstances.iter_mut().find(|instance| instance.is_window_active(app.window().id(), instance_locked));

    if state.settings.general.disable_log && log::max_level() == LevelFilter::Debug {
        log::set_max_level(LevelFilter::Off);
    } else if log::max_level() == LevelFilter::Off {
        log::set_max_level(LevelFilter::Debug);
    }
    match d2rprocess {
        Some(d2rprocess) => {
            if d2rprocess.is_window_active(app.window().id(), instance_locked) {
                let device_state: DeviceState = DeviceState::new();
                let keys: Vec<Keycode> = device_state.get_keys();

                // uses Home key to toggle egui panel visibility
                if state.settings.hotkeys.hotkey_toggle_menu.clone().pressed(&keys) {

                    if !state.ui_panel_toggle {
                        state.ui_panel_visible = !state.ui_panel_visible;
                        state.ui_panel_toggle = true
                    }
                } else {
                    state.ui_panel_toggle = false
                }

                //uses PageUp key to toggle map overlay visibility
                if state.settings.hotkeys.hotkey_toggle_map.clone().pressed(&keys) {
                    if !state.map_overlay_toggle {
                        state.map_overlay_visible = !state.map_overlay_visible;
                        state.map_overlay_toggle = true
                    }
                } else {
                    state.map_overlay_toggle = false
                }

                if state.settings.hotkeys.hotkey_exit.clone().pressed(&keys) {
                    std::process::exit(0);
                }
            }

            if let Some(game_data) = GameData::read_game_memory(d2rprocess) {
                check_buff_timers(&game_data, &mut d2rprocess.buff_instance.buff_timers);
                // if new seed
                if game_data.seed_values.map_seed != state.last_seed {
                    // generate new seed data using blachas' tool and parse the JSON into seed_data
                    log::info!(
                        "New game detected, generating data for map seed {} {:?} {}",
                        game_data.seed_values.map_seed,
                        game_data.seed_values.difficulty,
                        game_data.seed_values.level
                    );
                    log::info!("Using D2LoD path '{}'", &state.settings.general.d2lodpath.as_os_str().to_string_lossy());
                    state.seed_data = mapgeneration::seeddata::generate_seed_data(&game_data.seed_values, &state.settings);
                }
                state.last_seed = game_data.seed_values.map_seed;
                state.game_data = Some(game_data);
            } else {
                if state.game_data.is_some() {
                    log::debug!("Game data not found, in menu");
                }
                clear_item_log();
                state.game_data = None;
            }

            if state.settings.general.overlay_mode {
                let d2r_window: D2RWindowArea = d2rprocess.get_window_info();
                app.window().set_size(d2r_window.width as u32, d2r_window.height as u32);
                app.window().set_position(d2r_window.x, d2r_window.y);
                let relative_mouse_pos = get_relative_mouse_pos(&d2r_window);
                if mouse_hovering_egui(relative_mouse_pos, state.egui_rect, app.window().dpi()) {
                    if !state.egui_hovering {
                        unsafe {
                            let hwnd = app.window().id() as isize as HWND;
                            let mut style =
                                WS_CAPTION | WS_MINIMIZEBOX | WS_BORDER | WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_SYSMENU;
                            let mut style_ex = WS_EX_WINDOWEDGE | WS_EX_ACCEPTFILES;
                            style |= WS_VISIBLE;
                            style_ex |= WS_EX_TOPMOST;
                            SetWindowLongW(hwnd, GWL_STYLE, style as i32);
                            SetWindowLongW(hwnd, GWL_EXSTYLE, style_ex as i32);
                        }
                    }
                    state.egui_hovering = true;
                } else {
                    if state.egui_hovering {
                        unsafe {
                            let hwnd = app.window().id() as isize as HWND;
                            let mut style =
                                WS_CAPTION | WS_MINIMIZEBOX | WS_BORDER | WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_SYSMENU;
                            let mut style_ex = WS_EX_WINDOWEDGE | WS_EX_ACCEPTFILES;
                            style |= WS_VISIBLE;
                            style_ex |= WS_EX_TOPMOST;
                            style_ex |= WS_EX_TRANSPARENT | WS_EX_LAYERED;
                            SetWindowLongW(hwnd, GWL_STYLE, style as i32);
                            SetWindowLongW(hwnd, GWL_EXSTYLE, style_ex as i32);
                        }
                    }
                    state.egui_hovering = false;
                }
                state.relative_mouse_pos = relative_mouse_pos;
            } else {
                if app.window().size().0 == 10 && app.window().size().1 == 10 {
                    app.window().set_size(800, 600);
                    app.window().set_position(100, 100);
                }
            }
        },
        None => return,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {

    let instance_locked = state.instance_locked.clone();
    let d2rprocess = state.d2rinstances.iter_mut().find(|instance| instance.is_window_active(app.window().id(), instance_locked));

    if let Some(d2rprocess) = d2rprocess {
        if d2rprocess.is_window_active(app.window().id(), instance_locked) || !state.settings.general.overlay_mode {
            let width: f32;
            let height: f32;
            let mut mask = gfx.create_draw();

            match state.settings.general.map_position {
                MapPosition::Center => {
                    width = app.window().width() as f32;
                    height = app.window().height() as f32;
                    mask.rect((0.0, 0.0), (app.window().width() as f32, app.window().height() as f32));
                }
                MapPosition::TopLeft => {
                    width = app.window().width() as f32 * 0.33;
                    height = app.window().height() as f32 / 2.45;
                    mask.rect((0.0, 0.0), (app.window().width() as f32 / 3.0, app.window().height() as f32 / 3.0));
                }
                MapPosition::TopRight => {
                    width = app.window().width() as f32 * 1.67;
                    height = app.window().height() as f32 / 2.44;
                    mask.rect(
                        (app.window().width() as f32 - (app.window().width() as f32 / 3.0), 0.0),
                        (app.window().width() as f32 / 3.0, app.window().height() as f32 / 3.0),
                    );
                }
            }

            let mut draw = gfx.create_draw();
            draw.mask(Some(&mask));

            // toggle map with "Page Up" button
            if state.map_overlay_visible {
                // in game
                if let Some(game_data) = &state.game_data {
                    if (game_data.menus.automap_visible || state.settings.visual.always_show_map)
                        && !(state.settings.visual.hide_map_menus_open && game_data.menus.is_panel_open())
                    {
                        let stitched_levels = get_attached_levels(&game_data.seed_values.level);
                        stitched_levels.iter().for_each(|level_id| {
                            if let Some(this_level) = state.seed_data.levels.iter_mut().find(|l| l.id == *level_id) {
                                let scale: f32 = state.settings.visual.scale;
                                // render map image here
                                if this_level.level_image.map_image.is_none() || state.last_map_opacity != state.settings.visual.map_opacity {
                                    log::info!(
                                        "Rendering map image, seed: {}, difficulty: {:?}, level: {:?}",
                                        &game_data.seed_values.map_seed,
                                        &game_data.seed_values.difficulty,
                                        &this_level.name
                                    );
                                    this_level.level_image.map_image = Some(draw_map(gfx, this_level, &state.settings));
                                }

                                if let Some(map_image) = &mut this_level.level_image.map_image {
                                    let render_scale = state.settings.general.render_scale;
                                    let window_center_x = width as f32 * 0.5 / scale * render_scale;
                                    let window_center_y = height as f32 * 0.5 / (scale / 2.0 / render_scale);

                                    let map_position_x = ((this_level.offset.x as f32 - game_data.player.pos_x)
                                        * render_scale)
                                        + window_center_x;
                                    let map_position_y = ((this_level.offset.y as f32 - game_data.player.pos_y)
                                        * render_scale)
                                        + window_center_y;

                                    let player_pos_x = (game_data.player.pos_x - this_level.offset.x as f32) * render_scale;
                                    let player_pos_y = (game_data.player.pos_y - this_level.offset.y as f32) * render_scale;
                                    let scale_matrix =
                                        Mat3::from_scale(Vec2::from([scale / render_scale, scale / 2.0 / render_scale]));
                                    draw.transform().push(scale_matrix);
                                    draw.image(map_image)
                                        .translate(map_position_x, map_position_y)
                                        .rotate_degrees_from(
                                            (map_position_x + player_pos_x, map_position_y + player_pos_y),
                                            45.0,
                                        );
                                    if &game_data.seed_values.level == &this_level.id {
                                        draw_pathfinding(&mut draw, game_data, &state.settings, this_level, map_position_x, map_position_y, player_pos_x, player_pos_y);
                                    }
                                    draw.transform().pop();
                                    draw_presets(
                                        &mut draw,
                                        this_level,
                                        &state.fonts,
                                        game_data,
                                        &state.settings,
                                        &state.images,
                                        &width,
                                        &height,
                                    );
                                    draw_lines(&mut draw, this_level, game_data, &state.settings, &width, &height);
                                }
                            }
                        });
                        state.last_map_opacity = state.settings.visual.map_opacity.clone();

                        draw_units(
                            &mut draw,
                            game_data,
                            &state.settings,
                            &width,
                            &height,
                            &state.fonts,
                        );
                        draw_objects(&mut draw, game_data, &state.settings, &width, &height, &state.images, &state.fonts);
                        draw.mask(None);

                        draw_item_log(
                            &mut draw,
                            game_data,
                            &state.settings,
                            &width,
                            &height,
                            &state.fonts.exocet_font,
                            state.item_frame,
                            &state.item_filters,
                        );

                        draw_item_tooltip(&mut draw, game_data, &state.settings, &state.fonts.exocet_font, &state.settings.visual.scale, state.relative_mouse_pos);
                        draw_buff_bar(&mut draw, game_data, &state.settings, &state.fonts, &mut d2rprocess.buff_instance, game_data.menus.skill_popover_visible, &app.window().width(), &app.window().height(), &state.images);
                        draw_party_info(&mut draw, game_data, &state.fonts.formal_font, game_data.menus.party_portaits, &state.settings.party_info, &app.window().width(), &app.window().height());

                        state.item_frame += 1;
                        if state.item_frame > 20 {
                            state.item_frame = 0;
                        }
                    }

                    if !game_data.menus.pause_menu_visible {
                        let addr_base = d2rprocess.offsets.hover + d2rprocess.base_address as u64;
                        let val_base: u32 = d2rprocess.read_mem(addr_base);

                        if val_base == 1 {
                            let val_plus_4: u32 = d2rprocess.read_mem(addr_base + 4);

                            if val_plus_4 == 1 || val_plus_4 == 4 {
                                let val_plus_8: u32 = d2rprocess.read_mem(addr_base + 8);

								if val_plus_4 == 1 {
									use crate::memory::structs::{Unit, StatsList, StatValueStruct};

									let bucket_index = (val_plus_8 & 0x7F) as usize;

									let base_addr_1ecaa48: u64 = d2rprocess.read_mem(d2rprocess.base_address as u64 + 0x1ECAA48);
									let mut current_unit_ptr: u64 = if base_addr_1ecaa48 != 0 {
										let bucket_addr = base_addr_1ecaa48 + 0x2630 + (bucket_index as u64 * 8);
										d2rprocess.read_mem(bucket_addr)
									} else {
										0
									};

									let mut found_unit: Option<Unit> = None;
									let mut found_unit_ptr: u64 = 0;

									while current_unit_ptr != 0 && current_unit_ptr != 1 {
										let current_unit: Unit = d2rprocess.read_mem(current_unit_ptr);
										if current_unit.unit_id == val_plus_8 {
											found_unit = Some(current_unit);
											break;
										}
										current_unit_ptr = current_unit.p_next;
									}

									if let Some(unit) = found_unit {
										if unit.p_stats_list_ex != 0 {
											let stats_list: StatsList = d2rprocess.read_mem(unit.p_stats_list_ex);

											let current_unit_hex_str = format!("0x{:X}", current_unit_ptr);
											let mut monster_lv: Option<u32> = None;
											let mut cur_hp_raw: Option<u32> = None;
											let mut max_hp_raw: Option<u32> = None;

											let mut res_phys: (i32, bool) = (0, false);
											let mut res_mag: (i32, bool) = (0, false);
											let mut res_fire: (i32, bool) = (0, false);
											let mut res_cold: (i32, bool) = (0, false);
											let mut res_light: (i32, bool) = (0, false);
											let mut res_pois: (i32, bool) = (0, false);

											let stride = 8;

											if stats_list.stat_ptr != 0 && stats_list.stat_count > 0 {
												for i in 0..stats_list.stat_count {
													let item_addr = stats_list.stat_ptr + (i as u64 * stride);
													let stat_item: StatValueStruct = d2rprocess.read_mem(item_addr);

													let full_value = (stat_item.value as u32 & 0xFFFF) | ((stat_item.value2 as u32 & 0xFFFF) << 16);

													match stat_item.stat {
														0x6 => cur_hp_raw = Some(full_value >> 8),
														0x7 => max_hp_raw = Some(full_value >> 8),
														0xC => monster_lv = Some(full_value),
														_ => {}
													}
												}
											}

											if stats_list.stat_ex_ptr != 0 && stats_list.stat_ex_count > 0 {
												for i in 0..stats_list.stat_ex_count {
													let item_addr = stats_list.stat_ex_ptr + (i as u64 * stride);
													let stat_item: StatValueStruct = d2rprocess.read_mem(item_addr);

													match stat_item.stat {
														36  => res_phys = (stat_item.value as i32, true),
														37  => res_mag = (stat_item.value as i32, true),
														39  => res_fire = (stat_item.value as i32, true),
														41  => res_light = (stat_item.value as i32, true),
														43  => res_cold = (stat_item.value as i32, true),
														45  => res_pois = (stat_item.value as i32, true),
														_ => {}
													}
												}
											}

											let mut text_segments: Vec<(String, u32)> = Vec::new();

											if let Some(lv) = monster_lv {
												text_segments.push((format!("Lv: {}", lv), 0xC6B276FF));
											}

											// text_segments.push((format!("Ptr: {}  ", current_unit_hex_str), 0x95A5A6FF));

											if let (Some(cur_hp), Some(max_hp)) = (cur_hp_raw, max_hp_raw) {
												let pct = if max_hp > 0 { (cur_hp as u64 * 100 / max_hp as u64) as u32 } else { 0 };
												text_segments.push((format!("HP: {}/{} ({}%)", cur_hp, max_hp, pct), 0xFFFFFFFF));
											}

											let push_res = |segments: &mut Vec<(String, u32)>, prefix: &str, res_data: (i32, bool), color: u32| {
												if res_data.1 {
													let text = format!("{}:{}", prefix, res_data.0);
													segments.push((text, color));
												}
											};

											push_res(&mut text_segments, "物", res_phys, 0xC6B276FF);
											push_res(&mut text_segments, "魔", res_mag, 0xE67E22FF);
											push_res(&mut text_segments, "火", res_fire, 0xE74C3CFF);
											push_res(&mut text_segments, "冰", res_cold, 0x5DADE2FF);
											push_res(&mut text_segments, "电", res_light, 0xF4D03FFF);
											push_res(&mut text_segments, "毒", res_pois, 0x2ECC71FF);

											if !text_segments.is_empty() {
												let screen_w = app.window().width() as f32;
												let screen_h = app.window().height() as f32;
												let font_size = 28.0;
												let draw_y = screen_h * 0.12;

												let mut total_line_width = 0.0;
												for (txt, _) in &text_segments {
													for c in txt.chars() {
														if c.is_ascii() {
															total_line_width += 14.0;
														} else {
															total_line_width += 28.0;
														}
													}
												}

												let mut current_x = (screen_w * 0.5) - (total_line_width * 0.5);

												for (txt, color_hex) in text_segments {
													let mut text_draw = draw.text(&state.fonts.blizzard_font, &txt);
													text_draw
														.position(current_x, draw_y)
														.size(font_size)
														.color(Color::from_hex(color_hex))
														.h_align_left()
														.v_align_top();

													let mut segment_width = 0.0;
													for c in txt.chars() {
														if c.is_ascii() {
															segment_width += 13.0;
														} else {
															segment_width += 28.0;
														}
													}

													current_x += segment_width;
												}
											}
										}
									}
								}
                            }
                        }

						use crate::memory::structs::{Unit, StatsList, StatesList};

						let base_addr_1ecaa48: u64 = d2rprocess.read_mem(d2rprocess.base_address as u64 + 0x1ECAA48);
						if base_addr_1ecaa48 != 0 {
							let main_unit_ptr: u64 = d2rprocess.read_mem(base_addr_1ecaa48 + 0x2238);
							if main_unit_ptr != 0 {
								let main_unit: Unit = d2rprocess.read_mem(main_unit_ptr);

								if main_unit.p_stats_list_ex != 0 {
									let stats_list: StatsList = d2rprocess.read_mem(main_unit.p_stats_list_ex);
									let mut current_states_ptr = stats_list.state_unit_ptr;
									let mut state_name_map: HashMap<u32, &str> = HashMap::new();
									state_name_map.insert(0x01, "冻结状态");
									state_name_map.insert(0x02, "中毒状态");
									state_name_map.insert(0x09, "伤害加深");
									state_name_map.insert(0x0B, "冰寒状态");
									state_name_map.insert(0x1C, "审判光环");
									state_name_map.insert(0x2C, "冰冻光环");
									state_name_map.insert(0x80, "护甲神殿");
									state_name_map.insert(0x82, "抗电神殿");
									state_name_map.insert(0x83, "抗火神殿");
									state_name_map.insert(0x84, "抗寒神殿");
									state_name_map.insert(0x85, "抗毒神殿");
									state_name_map.insert(0x86, "技能神殿");
									state_name_map.insert(0x89, "经验神殿");
									state_name_map.insert(0xB2, "解毒药水");
									state_name_map.insert(0xB3, "融冰药水");
									state_name_map.insert(0xB4, "精力药水");
									state_name_map.insert(0xCD, "咒印状态");
									state_name_map.insert(0xD0, "吞噬状态");

									let mut state_blacklist: HashSet<u32> = HashSet::new();
									state_blacklist.insert(0x00);
									state_blacklist.insert(0x64);	//治疗药水
									state_blacklist.insert(0x66);	//进门CD
									state_blacklist.insert(0x6A);	//法力药水
									state_blacklist.insert(0xB9);	//公共CD
									state_blacklist.insert(0xE6);	//瘴气锁链

									let screen_w = app.window().width() as f32;
									let screen_h = app.window().height() as f32;
									let font_size_buff = 28.0;
									let line_height_buff = 32.0;
									let mut draw_y_buff = screen_h * 0.75;

									while current_states_ptr != 0 {
										let states_list: StatesList = d2rprocess.read_mem(current_states_ptr);

										if state_blacklist.contains(&states_list.state_id) {
											current_states_ptr = states_list.p_next_state;
											continue;
										}

										if states_list.unit_type != 4 && states_list.duration_end_frame_low > 0 {
											let state_name = match state_name_map.get(&states_list.state_id) {
												Some(name) => name.to_string(),
												None => format!("{:X}", states_list.state_id),
											};

											// let end_frame_bits = (states_list.duration_end_frame_low as u64) | ((states_list.duration_end_frame_high as u64) << 32);
											let current_frame: u32 = d2rprocess.read_mem(base_addr_1ecaa48 + 0x170);
											let duration_end_frame: f32 = f32::from_bits(states_list.duration_end_frame_low);
											let total_seconds = (duration_end_frame as u32 - current_frame) / 25;

											let hours = total_seconds / 3600;
											let minutes = (total_seconds % 3600) / 60;
											let seconds = total_seconds % 60;

											let time_string = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
											let text_buff = format!("{}：{}", state_name, time_string);

											let mut total_buff_line_width = 0.0;
											for c_buff in text_buff.chars() {
												if c_buff.is_ascii() {
													total_buff_line_width += 14.0;
												} else {
													total_buff_line_width += 28.0;
												}
											}

											let draw_x_buff = screen_w - total_buff_line_width - 10.0;
											let mut text_draw_buff = draw.text(&state.fonts.blizzard_font, &text_buff);
											text_draw_buff
												.position(draw_x_buff, draw_y_buff)
												.size(font_size_buff)
												.color(Color::from_hex(0xC6B276FF))
												.h_align_left()
												.v_align_top();
											draw_y_buff += line_height_buff;
										}

										current_states_ptr = states_list.p_next_state;
									}

								}
							}
						}
                    }
                } else {

                    // in game menus
                    let last_game_name = gamedata::get_last_game_name(&d2rprocess);

                    if last_game_name.len() > 0 {
                        let last_game = format!("Last Game: {}", last_game_name);
                        draw.text(&state.fonts.exocet_font, &last_game)
                            .position(app.window().width() as f32 * 0.75, 10.0)
                            .size(16.0)
                            .color(Color::from_hex(0xC6B276FF))
                            .h_align_center()
                            .v_align_top();
                    }
                }
            }

            let mut output;
            if state.language_selector_visible {
                output = plugins.egui(|ctx| {
                    if state.ui_panel_visible{
                        create_language_select_ui(app, ctx, state);
                    }
                    state.egui_rect = ctx.used_rect();
                });
            } else {
                let hwnd = d2rprocess.window.hwnd.clone();
                output = plugins.egui(|ctx| {
                    if state.ui_panel_visible{
                        create_egui_panel(app, ctx, state, hwnd);
                    }
                    state.egui_rect = ctx.used_rect();
                });
            }

            output.clear_color(Color::TRANSPARENT);

            gfx.render(&output);
            gfx.render(&draw);

        } else {
            let mut draw = gfx.create_draw();
            draw.clear(Color::TRANSPARENT);
            gfx.render(&draw);
        }
    } else {
        let mut draw = gfx.create_draw();
        draw.clear(Color::TRANSPARENT);
        gfx.render(&draw);
    }
}

fn get_relative_mouse_pos(d2r_window: &D2RWindowArea) -> (i32, i32) {
    let mut point = POINT { x: 0, y: 0 };
    unsafe { ::winapi::um::winuser::GetCursorPos(&mut point as *mut POINT) };
    (point.x - d2r_window.x, point.y - d2r_window.y)
}

fn mouse_hovering_egui(relative_mouse_pos: (i32, i32), egui_rect: Rect, dpi: f64) -> bool {
    relative_mouse_pos.0 > egui_rect.left() as i32
        && relative_mouse_pos.0 < (egui_rect.right() * dpi as f32) as i32
        && relative_mouse_pos.1 > egui_rect.top() as i32
        && relative_mouse_pos.1 < (egui_rect.bottom() * dpi as f32) as i32
}
