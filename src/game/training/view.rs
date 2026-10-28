use super::{
    TrainingStage, instructions,
    logic::{self, Action, Challenge, Feature},
};
use crate::common::render::{UiFont, pixel_font};
use crate::common::settings::InputAction;
use crate::game::model::{GameEntity, GameKind, GameSession, SaveData};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::{FontHinting, TextBounds};

const BG: Color = Color::srgb(0.035, 0.055, 0.08);
const CARD: Color = Color::srgb(0.075, 0.11, 0.15);
const EDGE: Color = Color::srgb(0.26, 0.34, 0.42);
const INK: Color = Color::srgb(0.95, 0.97, 0.98);
const MUTED: Color = Color::srgb(0.71, 0.79, 0.83);
const MINT: Color = Color::srgb(0.48, 0.92, 0.76);
const GOLD: Color = Color::srgb(1.0, 0.81, 0.35);
const COLORS: [Color; 4] = [
    Color::srgb(1.0, 0.38, 0.38),
    Color::srgb(0.32, 0.73, 1.0),
    Color::srgb(0.42, 0.91, 0.60),
    Color::srgb(1.0, 0.80, 0.28),
];

#[derive(Resource)]
pub(in crate::game) struct TrainingArt {
    shapes: [Handle<Mesh>; 4],
    colors: [Handle<ColorMaterial>; 4],
}

impl FromWorld for TrainingArt {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let shapes = [
            meshes.add(Circle::new(1.0)),
            meshes.add(Rectangle::new(2.0, 2.0)),
            meshes.add(RegularPolygon::new(1.0, 3)),
            meshes.add(Rectangle::new(1.4, 1.4)),
        ];
        let colors = COLORS.map(|color| world.resource_mut::<Assets<ColorMaterial>>().add(color));
        Self { shapes, colors }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum UiAction {
    Game(Action),
    Restart,
    Back,
}

#[derive(Component)]
pub(in crate::game) struct GameButton {
    pub(super) bounds: Rect,
    pub(super) action: UiAction,
}

#[derive(Component)]
pub(in crate::game) struct TrainingVisual;

struct Draw<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    font: &'a UiFont,
    art: &'a TrainingArt,
    depth: f32,
}

impl Draw<'_, '_, '_> {
    fn rect(&mut self, pos: Vec2, size: Vec2, color: Color, z: f32) -> Entity {
        self.commands
            .spawn((
                Sprite::from_color(color, size),
                Transform::from_translation(pos.extend(z)),
                GameEntity,
                TrainingVisual,
            ))
            .id()
    }

    fn panel(&mut self, pos: Vec2, size: Vec2, fill: Color, border: Color) -> Entity {
        self.depth += 0.02;
        self.rect(pos, size, border, self.depth);
        self.rect(pos, size - Vec2::splat(2.0), fill, self.depth + 0.01)
    }

    fn text(&mut self, value: &str, pos: Vec2, size: f32, color: Color, anchor: Anchor) {
        self.commands.spawn((
            Text2d::new(value),
            pixel_font(self.font, size),
            FontHinting::Enabled,
            TextColor(color),
            anchor,
            Transform::from_translation(pos.extend(10.0)),
            GameEntity,
            TrainingVisual,
        ));
    }

    fn paragraph(&mut self, value: &str, pos: Vec2, width: f32, size: f32) {
        self.commands.spawn((
            Text2d::new(value),
            pixel_font(self.font, size),
            FontHinting::Enabled,
            TextColor(MUTED),
            Anchor::TOP_LEFT,
            TextBounds {
                width: Some(width),
                height: None,
            },
            Transform::from_translation(pos.extend(10.0)),
            GameEntity,
            TrainingVisual,
        ));
    }

    fn button(&mut self, value: &str, pos: Vec2, size: Vec2, action: UiAction, selected: bool) {
        let entity = self.panel(
            pos,
            size,
            if selected {
                Color::srgb(0.12, 0.25, 0.24)
            } else {
                CARD
            },
            if selected { MINT } else { EDGE },
        );
        self.commands.entity(entity).insert(GameButton {
            bounds: Rect::from_center_size(pos, size),
            action,
        });
        self.text(
            value,
            pos,
            18.0,
            if selected { MINT } else { INK },
            Anchor::CENTER,
        );
    }

    fn feature(&mut self, feature: Feature, pos: Vec2, size: f32) {
        self.commands.spawn((
            Mesh2d(self.art.shapes[feature.shape as usize % 4].clone()),
            MeshMaterial2d(self.art.colors[feature.color as usize % 4].clone()),
            Transform::from_translation(pos.extend(3.0))
                .with_scale(Vec3::splat(size * 0.5))
                .with_rotation(Quat::from_rotation_z(if feature.shape == 3 {
                    std::f32::consts::FRAC_PI_4
                } else {
                    0.0
                })),
            GameEntity,
            TrainingVisual,
        ));
    }

    fn bit_grid(
        &mut self,
        board: &[bool],
        n: usize,
        center: Vec2,
        cell: f32,
        cursor: Option<usize>,
    ) {
        for (i, &on) in board.iter().enumerate() {
            let pos = grid_pos(i, n, n, center, cell);
            let entity = self.panel(
                pos,
                Vec2::splat(cell - 5.0),
                if on {
                    Color::srgb(0.16, 0.48, 0.41)
                } else {
                    CARD
                },
                if cursor == Some(i) { GOLD } else { EDGE },
            );
            if cursor.is_some() {
                self.commands.entity(entity).insert(GameButton {
                    bounds: Rect::from_center_size(pos, Vec2::splat(cell - 5.0)),
                    action: UiAction::Game(Action::Pick(i)),
                });
            }
            if on {
                self.rect(pos, Vec2::splat(cell * 0.45), MINT, 3.0);
            }
        }
    }
}

fn grid_pos(index: usize, cols: usize, rows: usize, center: Vec2, cell: f32) -> Vec2 {
    center
        + Vec2::new(
            (index % cols) as f32 - (cols - 1) as f32 * 0.5,
            (rows - 1) as f32 * 0.5 - (index / cols) as f32,
        ) * cell
}

fn controls(kind: GameKind) -> (&'static str, &'static str) {
    match kind {
        GameKind::Pattern | GameKind::Detective => ("选中答案", ""),
        GameKind::FlashGrid | GameKind::Mirror | GameKind::Balance => ("选择 / 取消", "提交检查"),
        GameKind::GoNoGo => ("响应目标", ""),
        GameKind::Hanoi => ("拿起 / 放下", "撤销一步"),
        GameKind::LightsOut => ("翻转灯光", "撤销一步"),
        GameKind::Coding => ("运行指令", "撤销指令"),
        GameKind::Rotation => ("提交检查", ""),
        _ => ("确认", "取消"),
    }
}

fn board(draw: &mut Draw, stage: &TrainingStage, session: &GameSession) {
    match &stage.challenge {
        Challenge::Pattern(g) => {
            for i in 0..6 {
                let pos = Vec2::new(-235.0 + i as f32 * 94.0, 58.0);
                draw.panel(pos, Vec2::splat(78.0), CARD, EDGE);
                if let Some(&value) = g.sequence.get(i) {
                    if g.numeric {
                        draw.text(&value.to_string(), pos, 34.0, INK, Anchor::CENTER);
                    } else {
                        draw.feature(
                            Feature {
                                shape: value,
                                color: value,
                            },
                            pos,
                            42.0,
                        );
                    }
                } else {
                    draw.text("?", pos, 40.0, GOLD, Anchor::CENTER);
                }
            }
            draw.text(
                "接下来是哪一个？",
                Vec2::new(0.0, -12.0),
                20.0,
                MUTED,
                Anchor::CENTER,
            );
            for (i, &value) in g.options.iter().enumerate() {
                let pos = Vec2::new(-210.0 + i as f32 * 140.0, -91.0);
                draw.button(
                    if g.numeric { "" } else { " " },
                    pos,
                    Vec2::new(120.0, 90.0),
                    UiAction::Game(Action::Pick(i)),
                    i == stage.cursor,
                );
                if g.numeric {
                    draw.text(&value.to_string(), pos, 34.0, INK, Anchor::CENTER);
                } else {
                    draw.feature(
                        Feature {
                            shape: value,
                            color: value,
                        },
                        pos,
                        42.0,
                    );
                }
            }
        }
        Challenge::Flash(g) => {
            let showing = g.preview > 0.0;
            draw.text(
                &if showing {
                    format!("记住这些位置 · {} 秒后隐藏", g.preview.ceil() as u32)
                } else {
                    "凭记忆选格子，再按提交检查".to_string()
                },
                Vec2::new(0.0, 131.0),
                18.0,
                GOLD,
                Anchor::CENTER,
            );
            draw.bit_grid(
                if showing { &g.target } else { &g.marked },
                g.n,
                Vec2::new(0.0, -22.0),
                272.0 / g.n as f32,
                if showing { None } else { Some(stage.cursor) },
            );
        }
        Challenge::Mirror(g) => {
            draw.text(
                "原图",
                Vec2::new(-170.0, 121.0),
                19.0,
                MUTED,
                Anchor::CENTER,
            );
            draw.text(
                "你的镜像",
                Vec2::new(170.0, 121.0),
                19.0,
                MINT,
                Anchor::CENTER,
            );
            let cell = 240.0 / g.n as f32;
            draw.bit_grid(&g.source, g.n, Vec2::new(-170.0, -22.0), cell, None);
            draw.bit_grid(
                &g.marked,
                g.n,
                Vec2::new(170.0, -22.0),
                cell,
                Some(stage.cursor),
            );
            for y in (-140..110).step_by(20) {
                draw.rect(Vec2::new(0.0, y as f32), Vec2::new(2.0, 10.0), GOLD, 3.0);
            }
        }
        Challenge::Reaction(g) => {
            draw.text(
                if g.shape_rule {
                    "只点绿色圆形；其他颜色或形状都等待"
                } else {
                    "绿色就按下，红色等它自己消失"
                },
                Vec2::new(0.0, 125.0),
                19.0,
                GOLD,
                Anchor::CENTER,
            );
            let entity = draw.panel(Vec2::new(0.0, -12.0), Vec2::new(270.0, 230.0), CARD, EDGE);
            draw.commands.entity(entity).insert(GameButton {
                bounds: Rect::from_center_size(Vec2::new(0.0, -12.0), Vec2::new(270.0, 230.0)),
                action: UiAction::Game(Action::Primary),
            });
            if stage.feedback > 0.0 {
                draw.text(
                    "准备下一次",
                    Vec2::new(0.0, 4.0),
                    24.0,
                    MUTED,
                    Anchor::CENTER,
                );
            } else {
                draw.feature(
                    Feature {
                        shape: if !g.shape_rule || g.round_shape { 0 } else { 2 },
                        color: if g.green { 2 } else { 0 },
                    },
                    Vec2::new(0.0, 4.0),
                    130.0,
                );
                draw.text(
                    &format!("{} 秒", g.remaining.ceil() as u32),
                    Vec2::new(0.0, -95.0),
                    23.0,
                    INK,
                    Anchor::CENTER,
                );
            }
            draw.text(
                &format!("剩余机会 {}", session.lives),
                Vec2::new(0.0, -150.0),
                19.0,
                MUTED,
                Anchor::CENTER,
            );
        }
        Challenge::Balance(g) => {
            for (x, label, value, color) in [
                (-158.0, "目标重量", g.target, GOLD),
                (158.0, "已选总和", g.sum(), MINT),
            ] {
                draw.text(label, Vec2::new(x, 126.0), 18.0, MUTED, Anchor::CENTER);
                draw.panel(Vec2::new(x, 62.0), Vec2::new(235.0, 86.0), CARD, EDGE);
                draw.text(
                    &value.to_string(),
                    Vec2::new(x, 62.0),
                    42.0,
                    color,
                    Anchor::CENTER,
                );
            }
            draw.text("=", Vec2::new(0.0, 62.0), 35.0, MUTED, Anchor::CENTER);
            let cols = g.weights.len() / 2;
            for (i, value) in g.weights.iter().enumerate() {
                let pos = grid_pos(i, cols, 2, Vec2::new(0.0, -85.0), 100.0);
                let pos = Vec2::new(pos.x, if i < cols { -48.0 } else { -125.0 });
                draw.button(
                    &format!("{}{}", if g.selected[i] { "✓ " } else { "" }, value),
                    pos,
                    Vec2::new(84.0, 60.0),
                    UiAction::Game(Action::Pick(i)),
                    i == stage.cursor || g.selected[i],
                );
            }
        }
        Challenge::Hanoi(g) => {
            for i in 0..3 {
                let x = -210.0 + i as f32 * 210.0;
                let pos = Vec2::new(x, -20.0);
                draw.button(
                    "",
                    pos,
                    Vec2::new(192.0, 238.0),
                    UiAction::Game(Action::Pick(i)),
                    stage.cursor == i,
                );
                draw.text(
                    ["起点", "借用", "终点"][i],
                    Vec2::new(x, 119.0),
                    19.0,
                    if i == 2 { MINT } else { MUTED },
                    Anchor::CENTER,
                );
                draw.rect(Vec2::new(x, -18.0), Vec2::new(5.0, 170.0), EDGE, 2.0);
                draw.rect(Vec2::new(x, -114.0), Vec2::new(175.0, 8.0), MUTED, 2.0);
                for (j, &disk) in g.towers[i].iter().enumerate() {
                    let pos = Vec2::new(x, -96.0 + j as f32 * 29.0);
                    draw.rect(
                        pos,
                        Vec2::new(32.0 + disk as f32 * 24.0, 24.0),
                        COLORS[(disk - 1) % 4],
                        3.0,
                    );
                    draw.text(&disk.to_string(), pos, 16.0, BG, Anchor::CENTER);
                }
                if g.held == Some(i) {
                    draw.text("已拿起", Vec2::new(x, 82.0), 18.0, GOLD, Anchor::CENTER);
                }
            }
            draw.text(
                &format!("已移动 {} 步 · 可以撤销", g.history.len()),
                Vec2::new(0.0, -158.0),
                17.0,
                MUTED,
                Anchor::CENTER,
            );
        }
        Challenge::Lights(g) => {
            draw.text(
                &format!("还亮着 {} 盏灯", g.on.iter().filter(|&&on| on).count()),
                Vec2::new(0.0, 131.0),
                19.0,
                GOLD,
                Anchor::CENTER,
            );
            for (i, &on) in g.on.iter().enumerate() {
                let cell = 272.0 / g.n as f32;
                let pos = grid_pos(i, g.n, g.n, Vec2::new(0.0, -22.0), cell);
                draw.button(
                    "",
                    pos,
                    Vec2::splat(cell - 6.0),
                    UiAction::Game(Action::Pick(i)),
                    stage.cursor == i,
                );
                if on {
                    draw.feature(Feature { shape: 0, color: 3 }, pos, cell * 0.48);
                } else {
                    draw.rect(pos, Vec2::splat(8.0), EDGE, 3.0);
                }
            }
        }
        Challenge::Coding(g) => {
            draw.text(
                "地图",
                Vec2::new(-165.0, 131.0),
                18.0,
                MUTED,
                Anchor::CENTER,
            );
            draw.text(
                &format!("指令 {}/{}", g.program.len(), g.n * 2 + 4),
                Vec2::new(160.0, 131.0),
                18.0,
                MINT,
                Anchor::CENTER,
            );
            for (i, &blocked) in g.blocked.iter().enumerate() {
                let cell = 260.0 / g.n as f32;
                let pos = grid_pos(i, g.n, g.n, Vec2::new(-165.0, -22.0), cell);
                draw.panel(
                    pos,
                    Vec2::splat(cell - 3.0),
                    if blocked { EDGE } else { CARD },
                    BG,
                );
                if blocked {
                    draw.text("×", pos, 22.0, MUTED, Anchor::CENTER);
                }
                if i == g.n * g.n - 1 {
                    draw.text("★", pos, 29.0, GOLD, Anchor::CENTER);
                }
                if i == g.robot {
                    draw.rect(pos, Vec2::splat(cell * 0.65), MINT, 3.0);
                    for dx in [-5.0, 5.0] {
                        draw.rect(pos + Vec2::new(dx, 3.0), Vec2::splat(3.0), BG, 4.0);
                    }
                }
            }
            for (i, offset) in [
                Vec2::new(0.0, 43.0),
                Vec2::new(48.0, 0.0),
                Vec2::ZERO,
                Vec2::new(-48.0, 0.0),
            ]
            .into_iter()
            .enumerate()
            {
                draw.button(
                    logic::Direction::ALL[i].label(),
                    Vec2::new(159.0, 40.0) + offset,
                    Vec2::splat(40.0),
                    UiAction::Game(Action::Pick(i)),
                    false,
                );
            }
            for i in 0..g.n * 2 + 4 {
                let pos = Vec2::new(64.0 + (i % 6) as f32 * 36.0, -45.0 - (i / 6) as f32 * 36.0);
                draw.panel(
                    pos,
                    Vec2::splat(32.0),
                    CARD,
                    if g.running && i == g.step { MINT } else { EDGE },
                );
                if let Some(dir) = g.program.get(i) {
                    draw.text(dir.label(), pos, 23.0, INK, Anchor::CENTER);
                }
            }
        }
        Challenge::Rotation(g) => {
            draw.text(
                "目标图形",
                Vec2::new(-170.0, 121.0),
                18.0,
                MUTED,
                Anchor::CENTER,
            );
            draw.text(
                "旋转这里",
                Vec2::new(170.0, 121.0),
                18.0,
                MINT,
                Anchor::CENTER,
            );
            draw.bit_grid(
                &g.target,
                g.n,
                Vec2::new(-170.0, 0.0),
                190.0 / g.n as f32,
                None,
            );
            draw.bit_grid(
                &g.current,
                g.n,
                Vec2::new(170.0, 0.0),
                190.0 / g.n as f32,
                None,
            );
            draw.button(
                "向左旋转",
                Vec2::new(99.0, -133.0),
                Vec2::new(132.0, 38.0),
                UiAction::Game(Action::Pick(0)),
                false,
            );
            draw.button(
                "向右旋转",
                Vec2::new(241.0, -133.0),
                Vec2::new(132.0, 38.0),
                UiAction::Game(Action::Pick(1)),
                false,
            );
        }
        Challenge::Detective(g) => {
            draw.text(
                "寻找",
                Vec2::new(-208.0, 131.0),
                18.0,
                MUTED,
                Anchor::CENTER,
            );
            draw.feature(g.target, Vec2::new(-148.0, 131.0), 25.0);
            let total = g.items.iter().filter(|&&item| item == g.target).count();
            draw.text(
                &format!(
                    "已找到 {} / {}",
                    g.found.iter().filter(|&&found| found).count(),
                    total
                ),
                Vec2::new(117.0, 131.0),
                18.0,
                MINT,
                Anchor::CENTER,
            );
            let cell = 280.0 / g.rows as f32;
            for (i, &item) in g.items.iter().enumerate() {
                let pos = grid_pos(i, g.cols, g.rows, Vec2::new(0.0, -28.0), cell);
                draw.button(
                    "",
                    pos,
                    Vec2::splat(cell - 7.0),
                    UiAction::Game(Action::Pick(i)),
                    stage.cursor == i || g.found[i],
                );
                if g.found[i] {
                    draw.text("✓", pos, 32.0, MINT, Anchor::CENTER);
                } else {
                    draw.feature(item, pos, cell * 0.47);
                }
            }
        }
    }
}

pub(in crate::game) fn render(
    mut commands: Commands,
    font: Res<UiFont>,
    art: Res<TrainingArt>,
    save: Res<SaveData>,
    mut stage: ResMut<TrainingStage>,
    session: Res<GameSession>,
    existing: Query<Entity, With<TrainingVisual>>,
) {
    if !stage.dirty {
        return;
    }
    stage.dirty = false;
    for entity in &existing {
        commands.entity(entity).try_despawn();
    }
    let mut draw = Draw {
        commands: &mut commands,
        font: &font,
        art: &art,
        depth: 1.0,
    };
    draw.rect(Vec2::ZERO, Vec2::new(960.0, 540.0), BG, 0.5);
    draw.text(
        stage.kind.short_title(),
        Vec2::new(-334.0, 241.0),
        27.0,
        INK,
        Anchor::CENTER_LEFT,
    );
    draw.text(
        &format!(
            "第 {} 关 · {} · 进度 {}/{}",
            stage.level,
            stage.age.label(),
            stage.completed,
            logic::rounds(stage.kind, stage.level)
        ),
        Vec2::new(-334.0, 206.0),
        16.0,
        MUTED,
        Anchor::CENTER_LEFT,
    );
    let binding = |action| save.settings.bindings[0].binding(action).keyboard.label();
    draw.button(
        &format!("{} 重来", binding(InputAction::Reset)),
        Vec2::new(282.0, 240.0),
        Vec2::new(112.0, 34.0),
        UiAction::Restart,
        false,
    );
    let (skill, instruction, help) = instructions(stage.kind);
    draw.text(
        instruction,
        Vec2::new(0.0, 171.0),
        21.0,
        INK,
        Anchor::CENTER,
    );
    if !stage.started {
        draw.panel(Vec2::new(0.0, -20.0), Vec2::new(650.0, 286.0), CARD, EDGE);
        draw.text(skill, Vec2::new(0.0, 73.0), 29.0, MINT, Anchor::CENTER);
        draw.paragraph(help, Vec2::new(-284.0, 27.0), 568.0, 22.0);
        draw.text(
            "准备好再开始，按开始键可以暂停",
            Vec2::new(0.0, -67.0),
            18.0,
            MUTED,
            Anchor::CENTER,
        );
        draw.button(
            "开始练习",
            Vec2::new(0.0, -120.0),
            Vec2::new(230.0, 42.0),
            UiAction::Game(Action::Primary),
            true,
        );
    } else {
        board(&mut draw, &stage, &session);
        if !stage.message.is_empty() {
            draw.text(
                &stage.message,
                Vec2::new(0.0, -186.0),
                17.0,
                GOLD,
                Anchor::CENTER,
            );
        }
    }
    let (primary, secondary) = controls(stage.kind);
    draw.button(
        &format!(
            "{}  {}",
            binding(InputAction::Primary),
            if stage.started {
                primary
            } else {
                "开始练习"
            }
        ),
        Vec2::new(-224.0, -239.0),
        Vec2::new(210.0, 38.0),
        UiAction::Game(Action::Primary),
        true,
    );
    if stage.started && !secondary.is_empty() {
        draw.button(
            &format!("{}  {}", binding(InputAction::Secondary), secondary),
            Vec2::new(0.0, -239.0),
            Vec2::new(210.0, 38.0),
            UiAction::Game(Action::Secondary),
            false,
        );
    } else {
        draw.text(
            "鼠标 / 方向键操作",
            Vec2::new(0.0, -239.0),
            16.0,
            MUTED,
            Anchor::CENTER,
        );
    }
    draw.button(
        &format!("{}  返回大厅", binding(InputAction::Back)),
        Vec2::new(224.0, -239.0),
        Vec2::new(210.0, 38.0),
        UiAction::Back,
        false,
    );
}
