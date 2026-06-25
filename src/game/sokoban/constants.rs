use bevy::prelude::*;

// 10 关推箱子地图。字符含义：
//   '#' 墙   ' ' 地板   '.' 目标点
//   '$' 箱子 '*' 箱子已在目标点上
//   '@' 玩家 '+' 玩家站在目标点上
pub const LEVELS: [&[&str]; 10] = [
    // L1：1 个箱子，水平一推
    &[
        "#######",
        "#     #",
        "#@$  .#",
        "#     #",
        "#######",
    ],
    // L2：1 个箱子，向上两推
    &[
        "#######",
        "#  .  #",
        "#     #",
        "#  $  #",
        "#  @  #",
        "#######",
    ],
    // L3：2 个箱子，分别推到右侧目标
    &[
        "########",
        "#      #",
        "#@$   .#",
        "#      #",
        "# $   .#",
        "#      #",
        "########",
    ],
    // L4：箱子需绕过墙体推入凹槽
    &[
        "########",
        "#      #",
        "#  ##  #",
        "#  .   #",
        "#      #",
        "#  $   #",
        "#  @   #",
        "#      #",
        "########",
    ],
    // L5：3 个箱子并排向上
    &[
        "##########",
        "#        #",
        "#  ...   #",
        "#        #",
        "#  $$$   #",
        "#  @     #",
        "#        #",
        "##########",
    ],
    // L6：箱子需要先向左再向上拐角
    &[
        "########",
        "#      #",
        "#.     #",
        "# $    #",
        "#  @   #",
        "#      #",
        "########",
    ],
    // L7：两个箱子各推一个角落
    &[
        "##########",
        "#.      .#",
        "#        #",
        "#   ##   #",
        "# $ ## $ #",
        "#   ##   #",
        "#   @    #",
        "#        #",
        "##########",
    ],
    // L8：5 个箱子并排
    &[
        "###########",
        "#         #",
        "#  .....  #",
        "#         #",
        "#  $$$$$  #",
        "#         #",
        "#    @    #",
        "###########",
    ],
    // L9：两组通道，箱子先平推再向上
    &[
        "##########",
        "#.      .#",
        "#        #",
        "#  #  #  #",
        "#  $  $  #",
        "#  # @#  #",
        "#        #",
        "##########",
    ],
    // L10：十字形阵，四箱推向四方
    &[
        "###########",
        "#    .    #",
        "#         #",
        "#    $    #",
        "#.  $@$  .#",
        "#    $    #",
        "#         #",
        "#    .    #",
        "###########",
    ],
];

// 每关限时（秒），逐关放宽以容纳更长解法。
pub const LEVEL_TIME: [f32; 10] = [
    60.0, 75.0, 100.0, 110.0, 140.0, 110.0, 180.0, 220.0, 200.0, 260.0,
];

// 棋盘可用区域，中心居中。
pub const BOARD_W: f32 = 880.0;
pub const BOARD_H: f32 = 380.0;
pub const BOARD_CENTER_Y: f32 = -20.0;

// 长按移动节奏：首次响应立即；之后每过 MOVE_COOLDOWN 秒触发一次。
pub const MOVE_COOLDOWN: f32 = 0.13;

// 颜色
pub const COLOR_WALL_BORDER: Color = Color::srgb(0.30, 0.20, 0.14);
pub const COLOR_WALL_INNER: Color = Color::srgb(0.56, 0.38, 0.24);
pub const COLOR_WALL_HI: Color = Color::srgb(0.72, 0.52, 0.34);
pub const COLOR_WALL_MORTAR: Color = Color::srgb(0.34, 0.22, 0.14);
pub const COLOR_FLOOR: Color = Color::srgb(0.16, 0.20, 0.28);
pub const COLOR_FLOOR_EDGE: Color = Color::srgb(0.12, 0.15, 0.22);
pub const COLOR_GOAL_BORDER: Color = Color::srgb(0.95, 0.52, 0.28);
pub const COLOR_GOAL_INNER: Color = Color::srgb(0.36, 0.20, 0.16);
pub const COLOR_GOAL_GLOW: Color = Color::srgb(1.0, 0.80, 0.42);

pub const COLOR_BOX_BORDER: Color = Color::srgb(0.74, 0.52, 0.22);
pub const COLOR_BOX_INNER: Color = Color::srgb(0.92, 0.72, 0.34);
pub const COLOR_BOX_DONE_BORDER: Color = Color::srgb(0.36, 0.78, 0.42);
pub const COLOR_BOX_DONE_INNER: Color = Color::srgb(0.56, 0.92, 0.46);

pub const COLOR_PLAYER_BORDER: Color = Color::srgb(0.20, 0.42, 0.72);
pub const COLOR_PLAYER_INNER: Color = Color::srgb(0.42, 0.72, 0.98);
pub const COLOR_PLAYER_FACE: Color = Color::srgb(1.0, 0.94, 0.66);
