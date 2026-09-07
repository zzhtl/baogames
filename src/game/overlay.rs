//! 统一的暂停 / 通关 / 失败覆盖层。
//!
//! 每帧对账 `GameSession` 的状态与覆盖层实体的存在性：状态变化时销毁重建，
//! 状态不变时零开销。实体挂在相机子节点下，魂斗罗 / 超级玛丽滚屏时依然居中。
//! 八个游戏无需任何接入代码，各自只负责把结果细节写进 `session.status`。

use bevy::prelude::*;

use crate::common::constants::{ARENA_H, ARENA_W, FONT_BODY, FONT_TITLE, Z_OVERLAY, Z_OVERLAY_PANEL, Z_OVERLAY_TEXT};
use crate::common::pixel_canvas::InGameCamera;
use crate::common::px::px;
use crate::common::theme::{ACCENT, BORDER, DANGER, SCRIM, SURFACE, TEXT_DIM, TEXT_MUTED};
use bevy::text::FontHinting;

use crate::common::render::{UiFont, pixel_font};

use super::model::GameSession;

#[derive(Component)]
pub(super) struct OverlayEntity;

/// 覆盖层应显示的内容；`None` 表示不显示。
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(super) enum OverlayKind {
    Paused,
    Won,
    Lost,
}

pub(super) fn overlay_kind(session: &GameSession) -> Option<OverlayKind> {
    if session.finished {
        Some(if session.won {
            OverlayKind::Won
        } else {
            OverlayKind::Lost
        })
    } else if session.paused {
        Some(OverlayKind::Paused)
    } else {
        None
    }
}

/// 标题 / 副标题（游戏自己的结果细节）/ 按键提示，文案全中文统一。
pub(super) fn overlay_texts(
    kind: OverlayKind,
    status: &str,
    has_next_level: bool,
) -> (&'static str, String, &'static str) {
    match kind {
        OverlayKind::Paused => ("已暂停", String::new(), "开始键 继续 · 返回键 卡带墙"),
        OverlayKind::Won if has_next_level => (
            "关卡完成",
            status.to_string(),
            "开始键 下一关 · 返回键 卡带墙",
        ),
        OverlayKind::Won => ("通关", status.to_string(), "开始键 再来一次 · 返回键 卡带墙"),
        OverlayKind::Lost => ("游戏结束", status.to_string(), "开始键 重试 · 返回键 卡带墙"),
    }
}

fn accent(kind: OverlayKind) -> Color {
    match kind {
        OverlayKind::Paused => Color::srgb(0.45, 0.62, 0.92),
        OverlayKind::Won => ACCENT,
        OverlayKind::Lost => DANGER,
    }
}

pub(super) fn overlay_sync(
    mut commands: Commands,
    session: Option<Res<GameSession>>,
    font: Res<UiFont>,
    camera_q: Query<Entity, With<InGameCamera>>,
    existing: Query<Entity, With<OverlayEntity>>,
    mut shown: Local<Option<OverlayKind>>,
) {
    let target = session.as_deref().and_then(overlay_kind);
    // Local 会跨对局残留（OnExit 只清实体不清 Local）：
    // 目标状态与记录一致且实体确实在位才能跳过，否则重建自愈。
    if target == *shown && (target.is_none() || !existing.is_empty()) {
        return;
    }
    *shown = target;
    for e in &existing {
        commands.entity(e).despawn();
    }
    let (Some(kind), Some(session)) = (target, session) else {
        return;
    };
    let Ok(camera) = camera_q.single() else {
        return;
    };

    let spawn = |commands: &mut Commands, bundle: (Sprite, Transform)| {
        commands
            .spawn((bundle.0, bundle.1, OverlayEntity, ChildOf(camera)))
            .id()
    };

    // 半透明全屏遮罩：放大 1.6 倍防止 AutoMin 适配后露边
    // 遮罩铺满：16:9 世界比 4:3 宽，用 1.6 倍保证两种比例下都盖满
    spawn(
        &mut commands,
        (
            Sprite::from_color(SCRIM, Vec2::new(ARENA_W * 1.6, ARENA_H * 1.6)),
            Transform::from_xyz(0.0, 0.0, Z_OVERLAY),
        ),
    );
    // 面板：accent 描边 + 深色填充。尺寸按画布像素给，描边正好 1 像素。
    let panel_size = Vec2::new(px(204.0), px(64.0));
    spawn(
        &mut commands,
        (
            Sprite::from_color(accent(kind), panel_size),
            Transform::from_xyz(0.0, 0.0, Z_OVERLAY_PANEL),
        ),
    );
    spawn(
        &mut commands,
        (
            Sprite::from_color(SURFACE, panel_size - Vec2::splat(BORDER * 2.0)),
            Transform::from_xyz(0.0, 0.0, Z_OVERLAY_PANEL + 0.1),
        ),
    );

    let has_next_level = session.won && session.level < session.kind.max_level();
    let (title, detail, hint) = overlay_texts(kind, &session.status, has_next_level);
    let mut spawn_text = |value: &str, y: f32, size: f32, color: Color| {
        commands.spawn((
            Text2d::new(value),
            pixel_font(&font, size),
            FontHinting::Enabled,
            TextColor(color),
            Transform::from_xyz(0.0, y, Z_OVERLAY_TEXT),
            OverlayEntity,
            ChildOf(camera),
        ));
    };
    spawn_text(title, px(15.0), FONT_TITLE, accent(kind));
    if !detail.is_empty() {
        spawn_text(&detail, px(-5.0), FONT_BODY, TEXT_MUTED);
    }
    spawn_text(hint, px(-21.0), FONT_BODY, TEXT_DIM);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::model::GameKind;

    fn session(paused: bool, finished: bool, won: bool) -> GameSession {
        GameSession {
            kind: GameKind::Tank,
            level: 1,
            score: 0,
            lives: 3,
            paused,
            finished,
            won,
            status: "细节".to_string(),
        }
    }

    #[test]
    fn kind_none_when_running() {
        assert_eq!(overlay_kind(&session(false, false, false)), None);
    }

    #[test]
    fn kind_paused() {
        assert_eq!(
            overlay_kind(&session(true, false, false)),
            Some(OverlayKind::Paused)
        );
    }

    #[test]
    fn finished_beats_paused() {
        // 结束状态优先于暂停：结束后开始键不再是“继续”
        assert_eq!(
            overlay_kind(&session(true, true, true)),
            Some(OverlayKind::Won)
        );
        assert_eq!(
            overlay_kind(&session(false, true, false)),
            Some(OverlayKind::Lost)
        );
    }

    #[test]
    fn texts_carry_status_detail() {
        let (title, detail, hint) = overlay_texts(OverlayKind::Won, "得分 123", true);
        assert_eq!(title, "关卡完成");
        assert_eq!(detail, "得分 123");
        assert!(hint.contains("下一关"));

        let (title, detail, _) = overlay_texts(OverlayKind::Paused, "得分 123", false);
        assert_eq!(title, "已暂停");
        assert!(detail.is_empty(), "暂停不显示结果细节");
    }
}
