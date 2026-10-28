//! 大厅独立渲染到窗口；游戏仍使用原来的低分辨率像素画布。
use bevy::camera::{ScalingMode, visibility::RenderLayers};
use bevy::ecs::system::SystemParam;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::Anchor;
use bevy::text::TextBounds;
use bevy::window::{CursorIcon, SystemCursorIcon};

use super::super::model::{GameCategory, GameKind, MenuEntity, SaveData};
use super::{CaptureDevice, LibraryFocus, MenuPage, MenuUiState};
use crate::common::settings::{InputAction, PlayerSlot};

const BG: Color = Color::srgb(0.035, 0.055, 0.075);
const PANEL: Color = Color::srgb(0.065, 0.092, 0.12);
const RAISED: Color = Color::srgb(0.09, 0.13, 0.16);
const LINE: Color = Color::srgb(0.15, 0.20, 0.24);
const INK: Color = Color::srgb(0.92, 0.95, 0.95);
const MUTED: Color = Color::srgb(0.55, 0.65, 0.69);
const MINT: Color = Color::srgb(0.49, 0.91, 0.76);
const GOLD: Color = Color::srgb(0.98, 0.77, 0.43);

pub(super) const CARD_SIZE: Vec2 = Vec2::new(364.0, 82.0);
pub(super) fn card_origin(slot: usize) -> Vec2 {
    Vec2::new(
        48.0 + (slot % 2) as f32 * 380.0,
        252.0 + (slot / 2) as f32 * 96.0,
    )
}

#[derive(Component)]
pub(in crate::game) struct MenuCamera;

#[derive(Resource)]
pub(in crate::game) struct MenuArt {
    font: Handle<Font>,
    rounded: Handle<Image>,
}

impl FromWorld for MenuArt {
    fn from_world(world: &mut World) -> Self {
        let font = world.resource::<crate::common::render::UiFont>().0.clone();
        // 一张可复用的圆角白底，九宫格缩放后保持 12px 圆角。
        let mut image = Image::new_fill(
            Extent3d {
                width: 32,
                height: 32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[255, 255, 255, 0],
            TextureFormat::Rgba8UnormSrgb,
            default(),
        );
        if let Some(data) = image.data.as_mut() {
            for y in 0..32 {
                for x in 0..32 {
                    let delta = (Vec2::new(x as f32 + 0.5, y as f32 + 0.5) - Vec2::splat(16.0))
                        .abs()
                        - Vec2::splat(4.0);
                    let alpha = (12.5 - delta.max(Vec2::ZERO).length()).clamp(0.0, 1.0);
                    data[(y * 32 + x) * 4 + 3] = (alpha * 255.0) as u8;
                }
            }
        }
        image.sampler = ImageSampler::linear();
        MenuArt {
            font,
            rounded: world.resource_mut::<Assets<Image>>().add(image),
        }
    }
}

pub(super) fn setup_camera(commands: &mut Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 2,
            clear_color: ClearColorConfig::Custom(BG),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: 1280.0,
                min_height: 720.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        Msaa::Off,
        RenderLayers::layer(2),
        MenuCamera,
    ));
}

pub(in crate::game) fn reset_cursor(mut commands: Commands, windows: Query<Entity, With<Window>>) {
    for entity in &windows {
        commands
            .entity(entity)
            .insert(CursorIcon::System(SystemCursorIcon::Default));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MenuTarget {
    Game(GameKind),
    Category(GameCategory),
    Settings,
    Launch,
    Stage(i8),
    Age,
    Setting(usize, i8),
    Back,
    Binding(usize, CaptureDevice),
    Player,
    Page(i8),
}

#[derive(Component)]
struct HitTarget {
    target: MenuTarget,
    bounds: Rect,
    base: Color,
}

#[derive(Resource, Default)]
pub(in crate::game) struct MenuPointer {
    pub(super) clicked: Option<MenuTarget>,
}

#[derive(SystemParam)]
pub(in crate::game) struct PointerInput<'w, 's> {
    buttons: Res<'w, ButtonInput<MouseButton>>,
    windows: Query<'w, 's, (Entity, &'static Window)>,
    cameras: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<MenuCamera>>,
    targets: Query<'w, 's, (&'static HitTarget, &'static mut Sprite)>,
}

pub(in crate::game) fn menu_pointer(
    mut commands: Commands,
    mut input: PointerInput,
    mut pointer: ResMut<MenuPointer>,
    time: Res<Time>,
    ui: Res<MenuUiState>,
) {
    pointer.clicked = None;
    let Ok((entity, window)) = input.windows.single() else {
        return;
    };
    let position = window.cursor_position().and_then(|cursor| {
        let (camera, transform) = input.cameras.single().ok()?;
        let world = camera.viewport_to_world_2d(transform, cursor).ok()?;
        Some(Vec2::new(world.x + 640.0, 360.0 - world.y))
    });
    let mut hovered = false;
    let mut hit_area = f32::INFINITY;
    for (target, mut sprite) in &mut input.targets {
        let active = ui.capture.is_none() || target.target == MenuTarget::Back;
        let over = active && position.is_some_and(|p| target.bounds.contains(p));
        let color = if over {
            target.base.mix(
                &Color::WHITE,
                if input.buttons.pressed(MouseButton::Left) {
                    0.06
                } else {
                    0.15
                },
            )
        } else {
            target.base
        };
        sprite.color = sprite
            .color
            .mix(&color, (time.delta_secs() * 18.0).min(1.0));
        if over {
            hovered = true;
            // 行背景和行内按钮可以重叠；优先命中面积更小的按钮。
            let area = target.bounds.width() * target.bounds.height();
            if input.buttons.just_pressed(MouseButton::Left) && area < hit_area {
                pointer.clicked = Some(target.target);
                hit_area = area;
            }
        }
    }
    commands
        .entity(entity)
        .insert(CursorIcon::System(if hovered {
            SystemCursorIcon::Pointer
        } else {
            SystemCursorIcon::Default
        }));
}

pub(super) struct View<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    art: &'a MenuArt,
    surface_depth: f32,
}

impl View<'_, '_, '_> {
    pub(super) fn block(&mut self, center: Vec2, size: Vec2, color: Color, z: f32) {
        self.commands.spawn((
            Sprite::from_color(color, size),
            Transform::from_xyz(center.x - 640.0, 360.0 - center.y, z),
            RenderLayers::layer(2),
            MenuEntity,
        ));
    }

    fn rounded(&mut self, origin: Vec2, size: Vec2, color: Color, z: f32) -> Entity {
        self.commands
            .spawn((
                Sprite {
                    image: self.art.rounded.clone(),
                    color,
                    custom_size: Some(size),
                    image_mode: SpriteImageMode::Sliced(TextureSlicer {
                        border: BorderRect::all(12.0),
                        ..default()
                    }),
                    ..default()
                },
                Transform::from_xyz(
                    origin.x + size.x * 0.5 - 640.0,
                    360.0 - origin.y - size.y * 0.5,
                    z,
                ),
                RenderLayers::layer(2),
                MenuEntity,
            ))
            .id()
    }

    fn surface(
        &mut self,
        origin: Vec2,
        size: Vec2,
        fill: Color,
        border: Color,
        target: Option<MenuTarget>,
    ) {
        // 子面板按绘制顺序叠放，避免同一深度的背景遮住按钮。
        self.surface_depth += 0.02;
        let z = self.surface_depth;
        self.rounded(origin, size, border, z);
        let entity = self.rounded(origin + Vec2::ONE, size - Vec2::splat(2.0), fill, z + 0.01);
        if let Some(target) = target {
            self.commands.entity(entity).insert(HitTarget {
                target,
                bounds: Rect::from_corners(origin, origin + size),
                base: fill,
            });
        }
    }

    fn label(&mut self, value: &str, x: f32, y: f32, size: f32, color: Color) {
        self.commands.spawn((
            Text2d::new(value),
            TextFont::from_font_size(size).with_font(self.art.font.clone()),
            TextColor(color),
            Anchor::CENTER_LEFT,
            Transform::from_xyz(x - 640.0, 360.0 - y, 10.0),
            RenderLayers::layer(2),
            MenuEntity,
        ));
    }

    fn paragraph(&mut self, value: &str, x: f32, y: f32, width: f32) {
        self.commands.spawn((
            Text2d::new(value),
            TextFont::from_font_size(17.0).with_font(self.art.font.clone()),
            TextColor(MUTED),
            TextBounds {
                width: Some(width),
                height: None,
            },
            Anchor::TOP_LEFT,
            Transform::from_xyz(x - 640.0, 360.0 - y, 10.0),
            RenderLayers::layer(2),
            MenuEntity,
        ));
    }

    fn button(&mut self, label: &str, origin: Vec2, size: Vec2, target: MenuTarget, primary: bool) {
        self.surface(
            origin,
            size,
            if primary { MINT } else { RAISED },
            if primary { MINT } else { LINE },
            Some(target),
        );
        self.label(
            label,
            origin.x + 18.0,
            origin.y + size.y * 0.5,
            18.0,
            if primary { BG } else { INK },
        );
    }
}

fn key(save: &SaveData, action: InputAction) -> &'static str {
    save.settings.bindings[PlayerSlot::One.index()]
        .binding(action)
        .keyboard
        .label()
}

fn start_key(save: &SaveData) -> String {
    let bound = key(save, InputAction::Start);
    if bound == "Enter" {
        bound.to_string()
    } else {
        format!("{bound} / Enter")
    }
}

fn build_library(view: &mut View, save: &SaveData, selected: GameKind, ui: &MenuUiState) {
    view.label("今天，玩点什么？", 48.0, 130.0, 32.0, INK);
    view.label("留一点时间，给简单的快乐。", 48.0, 171.0, 17.0, MUTED);
    for (i, category) in GameCategory::ALL.into_iter().enumerate() {
        let on = ui.category == category;
        let origin = Vec2::new(48.0 + i as f32 * 140.0, 200.0);
        view.surface(
            origin,
            Vec2::new(128.0, 36.0),
            if on { RAISED } else { BG },
            if on && ui.library_focus == LibraryFocus::Tabs {
                MINT
            } else if on {
                LINE
            } else {
                BG
            },
            Some(MenuTarget::Category(category)),
        );
        view.label(
            &format!("{}  /  {:02}", category.label(), category.games().len()),
            origin.x + 16.0,
            218.0,
            17.0,
            if on { MINT } else { MUTED },
        );
    }
    view.label("Tab 切换分类", 650.0, 218.0, 14.0, MUTED);
    let games = ui.category.games();
    let page = games.iter().position(|kind| *kind == selected).unwrap_or(0) / super::GAMES_PER_PAGE;
    let pages = games.len().div_ceil(super::GAMES_PER_PAGE);
    if pages > 1 {
        view.button("‹", Vec2::new(410.0, 200.0), Vec2::new(44.0, 36.0), MenuTarget::Page(-1), false);
        view.label(&format!("{} / {} 页", page + 1, pages), 476.0, 218.0, 16.0, MINT);
        view.button("›", Vec2::new(566.0, 200.0), Vec2::new(44.0, 36.0), MenuTarget::Page(1), false);
    }
    for (slot, kind) in games.iter().copied().skip(page * super::GAMES_PER_PAGE).take(super::GAMES_PER_PAGE).enumerate() {
        let pos = card_origin(slot);
        let on = kind == selected;
        let index = kind.index();
        let unlocked = save.unlocked_levels[index].min(kind.max_level());
        view.surface(
            pos,
            CARD_SIZE,
            if on {
                Color::srgb(0.10, 0.19, 0.19)
            } else {
                PANEL
            },
            if on { MINT } else { LINE },
            Some(MenuTarget::Game(kind)),
        );
        view.rounded(pos + Vec2::new(14.0, 13.0), Vec2::splat(56.0), BG, 3.0);
        super::icons::cover(view, kind, pos + Vec2::new(42.0, 41.0), 2.6);
        view.label(
            kind.short_title(),
            pos.x + 86.0,
            pos.y + 27.0,
            21.0,
            if on { MINT } else { INK },
        );
        view.label(
            &format!("已解锁 {unlocked}/{} 关", kind.max_level()),
            pos.x + 86.0,
            pos.y + 54.0,
            14.0,
            MUTED,
        );
        view.label(
            &format!("{:02}", page * super::GAMES_PER_PAGE + slot + 1),
            pos.x + 326.0,
            pos.y + 24.0,
            13.0,
            MUTED,
        );
        view.block(
            pos + Vec2::new(213.0, 70.0),
            Vec2::new(254.0, 2.0),
            LINE,
            3.0,
        );
        let width = 254.0 * unlocked as f32 / kind.max_level() as f32;
        view.block(
            pos + Vec2::new(86.0 + width * 0.5, 70.0),
            Vec2::new(width, 2.0),
            if on { MINT } else { kind.accent() },
            4.0,
        );
    }

    let index = selected.index();
    let unlocked = save.unlocked_levels[index].min(selected.max_level());
    let level = save.selected_levels[index].clamp(1, unlocked);
    view.surface(
        Vec2::new(824.0, 112.0),
        Vec2::new(408.0, 510.0),
        PANEL,
        LINE,
        None,
    );
    view.rounded(Vec2::new(844.0, 132.0), Vec2::new(368.0, 124.0), BG, 3.0);
    for x in 0..15 {
        for y in 0..4 {
            view.block(
                Vec2::new(860.0 + x as f32 * 24.0, 150.0 + y as f32 * 28.0),
                Vec2::splat(2.0),
                LINE,
                3.5,
            );
        }
    }
    super::icons::cover(view, selected, Vec2::new(1028.0, 192.0), 5.5);
    view.label("READY TO PLAY", 860.0, 150.0, 10.0, MINT);
    view.label(selected.short_title(), 848.0, 283.0, 28.0, INK);
    view.paragraph(selected.goal_text(), 848.0, 311.0, 360.0);
    view.label("最高得分", 848.0, 377.0, 14.0, MUTED);
    view.label(
        &format!("{:06}", save.high_scores[index]),
        932.0,
        376.0,
        23.0,
        GOLD,
    );
    view.block(Vec2::new(1028.0, 401.0), Vec2::new(360.0, 1.0), LINE, 3.0);
    let stage_on = ui.library_focus == LibraryFocus::Stage && ui.stage_row == 0;
    view.label(
        "选择关卡",
        848.0,
        438.0,
        16.0,
        if stage_on { MINT } else { MUTED },
    );
    for (x, delta, enabled, label) in [
        (980.0, -1, level > 1, "−"),
        (1164.0, 1, level < unlocked, "+"),
    ] {
        view.surface(
            Vec2::new(x, 416.0),
            Vec2::new(44.0, 44.0),
            RAISED,
            if enabled { LINE } else { PANEL },
            enabled.then_some(MenuTarget::Stage(delta)),
        );
        view.label(
            label,
            x + 14.0,
            438.0,
            22.0,
            if enabled { INK } else { MUTED.with_alpha(0.4) },
        );
    }
    view.label(
        &format!("第 {level:02} 关"),
        1050.0,
        438.0,
        20.0,
        if stage_on { MINT } else { INK },
    );
    if ui.category == GameCategory::Puzzle {
        view.label("年龄档", 848.0, 494.0, 16.0, MUTED);
        view.surface(
            Vec2::new(980.0, 474.0),
            Vec2::new(228.0, 40.0),
            RAISED,
            if ui.library_focus == LibraryFocus::Stage && ui.stage_row == 1 {
                MINT
            } else {
                LINE
            },
            Some(MenuTarget::Age),
        );
        view.label(
            &format!("{}     切换", save.age_tier.label()),
            1000.0,
            494.0,
            17.0,
            INK,
        );
    } else {
        view.label("通关后自动解锁下一关", 848.0, 494.0, 15.0, MUTED);
    }
    view.button(
        "开始游戏",
        Vec2::new(848.0, 542.0),
        Vec2::new(360.0, 52.0),
        MenuTarget::Launch,
        true,
    );
    view.label("Enter  →", 1100.0, 568.0, 15.0, BG);
    let hint = match ui.library_focus {
        LibraryFocus::Stage => "左右选关 · 上下切换选项 · 确认开始",
        LibraryFocus::Tabs => "左右切换分类 · 下键进入游戏列表",
        LibraryFocus::Games => "点击卡片选游戏，准备好就出发。",
    };
    view.label(hint, 848.0, 609.0, 12.0, MUTED);
    view.label(
        &format!(
            "方向键 / {}{}{}{}  选择",
            key(save, InputAction::Up),
            key(save, InputAction::Left),
            key(save, InputAction::Down),
            key(save, InputAction::Right)
        ),
        48.0,
        683.0,
        15.0,
        MUTED,
    );
    view.label(
        &format!(
            "{}  选关     {}  开始     {}  设置",
            key(save, InputAction::Primary),
            start_key(save),
            key(save, InputAction::Secondary)
        ),
        374.0,
        683.0,
        15.0,
        MUTED,
    );
    view.label(&format!("{} 款游戏  ·  随时开玩", GameKind::ALL.len()), 1054.0, 683.0, 14.0, MINT);
}

fn build_settings(view: &mut View, save: &SaveData, ui: &MenuUiState) {
    view.label("调到你的节奏", 48.0, 142.0, 34.0, INK);
    view.label("画面、声音和操作，都由你决定。", 48.0, 187.0, 18.0, MUTED);
    view.surface(
        Vec2::new(48.0, 240.0),
        Vec2::new(336.0, 284.0),
        PANEL,
        LINE,
        None,
    );
    view.label("小提示", 72.0, 275.0, 20.0, MINT);
    let tips = [
        "画面比例用于游戏内画布。\n大厅始终保持清晰、完整的布局。",
        "辅助模式放宽跳跃时机，\n适合刚开始玩平台跳跃的朋友。",
        "开启扫描线，找回老电视的感觉。\n关闭后，游戏画面更干净。",
        "震屏会为爆炸和命中增加反馈。\n偏爱平稳画面时可以关闭。",
        "音乐与音效可以分别调节。\n点击 − / +，每次调整 10%。",
        "音乐与音效可以分别调节。\n点击 − / +，每次调整 10%。",
        "分别设置 P1 和 P2 的键盘、手柄。\n同一玩家的冲突键位会自动交换。",
        "回到大厅，选一款游戏继续玩。",
    ];
    view.paragraph(tips[ui.settings_cursor], 72.0, 311.0, 288.0);
    view.label("设置修改后自动保存", 72.0, 487.0, 15.0, GOLD);
    let settings = &save.settings;
    let rows = [
        ("画面比例", settings.display_mode.label().to_string()),
        ("玩法模式", settings.gameplay_profile.label().to_string()),
        (
            "CRT 扫描线",
            if settings.crt_enabled {
                "开启"
            } else {
                "关闭"
            }
            .to_string(),
        ),
        (
            "轻微震屏",
            if settings.screen_shake {
                "开启"
            } else {
                "关闭"
            }
            .to_string(),
        ),
        (
            "音乐音量",
            format!("{}%", (settings.music_volume * 100.0).round() as i32),
        ),
        (
            "音效音量",
            format!("{}%", (settings.sfx_volume * 100.0).round() as i32),
        ),
        ("按键配置", "P1 / P2  →".to_string()),
        ("返回游戏大厅", "→".to_string()),
    ];
    for (i, (label, value)) in rows.iter().enumerate() {
        let y = 116.0 + i as f32 * 64.0;
        let on = i == ui.settings_cursor;
        view.surface(
            Vec2::new(428.0, y),
            Vec2::new(804.0, 54.0),
            if on { RAISED } else { PANEL },
            if on { MINT } else { LINE },
            Some(MenuTarget::Setting(i, 0)),
        );
        view.label(label, 452.0, y + 27.0, 19.0, if on { MINT } else { INK });
        view.label(value, 984.0, y + 27.0, 18.0, INK);
        if i < 6 {
            view.button(
                "−",
                Vec2::new(916.0, y + 6.0),
                Vec2::new(44.0, 42.0),
                MenuTarget::Setting(i, -1),
                false,
            );
            view.button(
                "+",
                Vec2::new(1174.0, y + 6.0),
                Vec2::new(44.0, 42.0),
                MenuTarget::Setting(i, 1),
                false,
            );
        }
        if i == 4 || i == 5 {
            let volume = if i == 4 {
                settings.music_volume
            } else {
                settings.sfx_volume
            };
            for segment in 0..10 {
                view.block(
                    Vec2::new(690.0 + segment as f32 * 17.0, y + 27.0),
                    Vec2::new(12.0, 16.0),
                    if (segment as f32) < (volume * 10.0).round() {
                        MINT
                    } else {
                        LINE
                    },
                    4.0,
                );
            }
        }
    }
    view.label(
        &format!(
            "↑ ↓  选择     ← →  调整     {} / Esc  返回",
            key(save, InputAction::Secondary)
        ),
        48.0,
        683.0,
        15.0,
        MUTED,
    );
}

fn build_controls(view: &mut View, save: &SaveData, ui: &MenuUiState) {
    view.label("让操作更顺手", 48.0, 142.0, 34.0, INK);
    view.paragraph(
        "点击右侧的键位开始修改，\n再按下新的键盘按键或手柄按钮。",
        48.0,
        180.0,
        330.0,
    );
    let player = if ui.control_player == PlayerSlot::One {
        "P1"
    } else {
        "P2"
    };
    view.button(
        &format!("当前玩家 {player}    切换"),
        Vec2::new(48.0, 274.0),
        Vec2::new(330.0, 52.0),
        MenuTarget::Player,
        false,
    );
    view.paragraph(
        "已有键位发生冲突时会自动交换。\n修改后立即生效，并自动保存。",
        48.0,
        356.0,
        330.0,
    );
    if let Some((_, action, device)) = ui.capture {
        view.surface(
            Vec2::new(48.0, 455.0),
            Vec2::new(330.0, 150.0),
            RAISED,
            MINT,
            None,
        );
        let device = if device == CaptureDevice::Keyboard {
            "键盘"
        } else {
            "手柄"
        };
        view.label(&format!("等待{device}输入…"), 68.0, 490.0, 22.0, MINT);
        view.label(
            &format!("正在修改：{}", action.label()),
            68.0,
            531.0,
            17.0,
            INK,
        );
        view.label("Esc 或右上角按钮取消", 68.0, 574.0, 15.0, MUTED);
    }
    view.label("操作", 452.0, 132.0, 15.0, MUTED);
    view.label("键盘", 762.0, 132.0, 15.0, MUTED);
    view.label("手柄", 1022.0, 132.0, 15.0, MUTED);
    let bindings = save.settings.bindings[ui.control_player.index()];
    for (i, action) in InputAction::ALL.into_iter().enumerate() {
        let y = 155.0 + i as f32 * 52.0;
        let on = i == ui.control_cursor;
        view.surface(
            Vec2::new(428.0, y),
            Vec2::new(804.0, 46.0),
            PANEL,
            LINE,
            None,
        );
        view.label(
            action.label(),
            452.0,
            y + 23.0,
            18.0,
            if on { MINT } else { INK },
        );
        let binding = bindings.binding(action);
        for (x, device, label) in [
            (708.0, CaptureDevice::Keyboard, binding.keyboard.label()),
            (968.0, CaptureDevice::Gamepad, binding.gamepad.label()),
        ] {
            let active = on && ui.control_device == device;
            view.surface(
                Vec2::new(x, y + 4.0),
                Vec2::new(246.0, 38.0),
                RAISED,
                if active { MINT } else { LINE },
                Some(MenuTarget::Binding(i, device)),
            );
            view.label(
                if active && ui.capture.is_some() {
                    "请按键…"
                } else {
                    label
                },
                x + 18.0,
                y + 23.0,
                17.0,
                if active { MINT } else { INK },
            );
        }
    }
    view.label(
        &format!(
            "↑ ↓  选择操作     ← →  选择设备     {}  修改     {}  切换玩家     Esc  返回",
            key(save, InputAction::Primary),
            key(save, InputAction::Reset)
        ),
        48.0,
        683.0,
        15.0,
        MUTED,
    );
}

pub(super) fn build_menu(
    commands: &mut Commands,
    art: &MenuArt,
    save: &SaveData,
    selected: GameKind,
    ui: &MenuUiState,
) {
    let mut view = View { commands, art, surface_depth: 1.0 };
    view.block(Vec2::new(640.0, 44.0), Vec2::new(1280.0, 88.0), PANEL, 0.0);
    for (x, y, color) in [
        (0.0, 0.0, MINT),
        (14.0, 0.0, GOLD),
        (0.0, 14.0, INK),
        (14.0, 14.0, MINT),
    ] {
        view.block(Vec2::new(54.0 + x, 37.0 + y), Vec2::splat(10.0), color, 3.0);
    }
    view.label("BaoGames", 94.0, 43.0, 28.0, INK);
    view.label("你的迷你游乐场", 278.0, 46.0, 15.0, MUTED);
    let label = match (ui.page, ui.capture.is_some()) {
        (MenuPage::Library, _) => "系统设置  ↗",
        (_, true) => "取消修改  ×",
        _ => "返回  ←",
    };
    view.button(
        label,
        Vec2::new(1072.0, 24.0),
        Vec2::new(160.0, 40.0),
        if ui.page == MenuPage::Library {
            MenuTarget::Settings
        } else {
            MenuTarget::Back
        },
        false,
    );
    view.block(Vec2::new(640.0, 650.0), Vec2::new(1184.0, 1.0), LINE, 0.0);
    match ui.page {
        MenuPage::Library => build_library(&mut view, save, selected, ui),
        MenuPage::Settings => build_settings(&mut view, save, ui),
        MenuPage::Controls => build_controls(&mut view, save, ui),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cards_fit_between_tabs_footer_and_detail_panel() {
        for slot in 0..8 {
            let origin = card_origin(slot);
            assert!(origin.x >= 48.0 && origin.x + CARD_SIZE.x < 824.0);
            assert!(origin.y >= 252.0 && origin.y + CARD_SIZE.y < 650.0);
            for other in slot + 1..8 {
                let delta = (origin - card_origin(other)).abs();
                assert!(delta.x > CARD_SIZE.x || delta.y > CARD_SIZE.y);
            }
        }
    }
}
