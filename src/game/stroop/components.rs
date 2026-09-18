use bevy::prelude::*;

/// 中央的题面汉字。
#[derive(Component)]
pub struct StroopWord;

/// 顶部的规则提示。
#[derive(Component)]
pub struct StroopPrompt;

/// 色块的选中框。
#[derive(Component)]
pub struct StroopCursor;

/// 顶栏右侧：关卡 / 剩余时间 / 进度。
#[derive(Component)]
pub struct StroopHud;

/// 底栏左侧：分数 / 纪录 / 连对。
#[derive(Component)]
pub struct StroopScoreHud;

/// 底栏右侧：本题倒计时与瞬时提示。
#[derive(Component)]
pub struct StroopMessage;
