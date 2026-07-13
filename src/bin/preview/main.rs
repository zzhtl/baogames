//! 离线精灵预览工具：把游戏在用的 `SpriteDef` 直接光栅化成 PNG，无需运行 Bevy。
//!
//! 用法：
//! ```text
//! cargo run --bin preview                      # 渲染全部 contra 精灵 + 场景
//! cargo run --bin preview -- contra player     # 单个精灵：x8 放大图 + 实机尺寸图
//! cargo run --bin preview -- contra scene      # 实战场景拼图（贴游戏背景色）
//! cargo run --bin preview -- compare a.png b.png out.png   # 与参考图并排
//! ```
//! 产物写到 `preview_out/`。`*_x8.png` 看清像素；`*_real.png` 按游戏实际缩放(1.333×)
//! 采样再放大，暴露"细腿/碎高光在小尺寸糊成一坨"的问题。

mod raster;

use std::fs;

use bevy::color::Color;
use bevy::math::Vec2;

use baogames::common::constants::{ARENA_W, WINDOW_W};
use baogames::common::sprite_def::SpriteDef;
use baogames::game::contra::palette as cp;
use baogames::game::contra::sprites as cs;
use baogames::game::bomb_maze::sprites as bms;
use baogames::game::memory_match::sprites as ms;
use baogames::game::sokoban::sprites as sk;
use baogames::game::space_shooter::sprites as ss;
use baogames::game::tank::sprites as ts;

use raster::{Canvas, View, sprite_bbox};

const OUT_DIR: &str = "preview_out";
/// 逻辑单位 → 屏幕像素的实机缩放比（相机 AutoMin 等比放大）。
const REAL_SCALE: f32 = WINDOW_W as f32 / ARENA_W as f32;
/// 单精灵预览的放大倍率。
const X8: f32 = 8.0;
/// 实机图采样后为肉眼可见再做的整数放大。
const REAL_ZOOM: u32 = 8;

fn main() {
    fs::create_dir_all(OUT_DIR).expect("创建 preview_out 失败");
    let args: Vec<String> = std::env::args().skip(1).collect();
    let argv: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match argv.as_slice() {
        [] => {
            render_all_contra();
            render_all_tank();
            render_all_space();
            render_all_bomb();
            render_all_memory();
            render_all_soko();
        }
        ["contra", "all"] => render_all_contra(),
        ["contra", "scene"] => render_contra_scene(),
        ["contra", name] => match contra_sprite(name) {
            Some(def) => render_sprite(&format!("contra_{name}"), def, scene_bg()),
            None => eprintln!("未知 contra 精灵: {name}（player/prone/flip/soldier/sniper/jumper/heavy/gunner）"),
        },
        ["tank", "all"] => render_all_tank(),
        ["tank", name] => match tank_sprite(name) {
            Some(def) => render_sprite(&format!("tank_{name}"), def, Color::srgb(0.10, 0.10, 0.13)),
            None => eprintln!("未知 tank 精灵: {name}（p1/p2/basic/fast/power/armor）"),
        },
        ["space", "all"] => render_all_space(),
        ["space", name] => match space_sprite(name) {
            Some(def) => render_sprite(&format!("space_{name}"), def, Color::srgb(0.03, 0.05, 0.10)),
            None => eprintln!("未知 space 精灵: {name}（player/scout/sniper/bomber/carrier/boss/powerup）"),
        },
        ["bomb", "all"] => render_all_bomb(),
        ["bomb", name] => match bomb_sprite(name) {
            Some(def) => render_sprite(&format!("bomb_{name}"), def, Color::srgb(0.16, 0.28, 0.16)),
            None => eprintln!("未知 bomb 精灵: {name}（player1/player2/balloom/oneal/doll/kondoria/bomb/hard/soft/flame/exit）"),
        },
        ["memory", "all"] => render_all_memory(),
        ["memory", name] => match memory_sprite(name) {
            Some(def) => render_sprite(&format!("memory_{name}"), def, Color::srgb(0.07, 0.09, 0.15)),
            None => eprintln!("未知 memory 精灵: {name}（back/face/matched）"),
        },
        ["soko", "all"] => render_all_soko(),
        ["soko", name] => match soko_sprite(name) {
            Some(def) => render_sprite(&format!("soko_{name}"), def, Color::srgb(0.10, 0.13, 0.19)),
            None => eprintln!("未知 soko 精灵: {name}（wall/floor/goal/box/boxdone/player）"),
        },
        ["compare", a, b, out] => compare(a, b, out),
        _ => eprintln!("参数无法识别，见文件头用法说明"),
    }
}

fn contra_sprite(name: &str) -> Option<&'static SpriteDef> {
    Some(match name {
        "player" | "bill" => &cs::PLAYER_BILL,
        "prone" => &cs::PLAYER_PRONE,
        "flip" => &cs::PLAYER_FLIP,
        "soldier" => &cs::ENEMY_SOLDIER,
        "sniper" => &cs::ENEMY_SNIPER,
        "jumper" => &cs::ENEMY_JUMPER,
        "heavy" => &cs::ENEMY_HEAVY,
        "gunner" => &cs::ENEMY_GUNNER,
        "falcon" => &cs::FALCON,
        "boss" => &cs::BOSS_BODY,
        _ => return None,
    })
}

fn tank_sprite(name: &str) -> Option<&'static SpriteDef> {
    Some(match name {
        "p1" | "player1" => &ts::TANK_P1,
        "p2" | "player2" => &ts::TANK_P2,
        "basic" => &ts::TANK_ENEMY_BASIC,
        "fast" => &ts::TANK_ENEMY_FAST,
        "power" => &ts::TANK_ENEMY_POWER,
        "armor" => &ts::TANK_ENEMY_ARMOR,
        "brick" => &ts::BRICK_SUBTILE,
        "steel" => &ts::STEEL_TILE,
        "water" => &ts::WATER_TILE,
        "bush" => &ts::BUSH_TILE,
        "ice" => &ts::ICE_TILE,
        "base" => &ts::BASE_EAGLE,
        "star" => &ts::PU_STAR,
        "grenade" => &ts::PU_GRENADE,
        "helmet" => &ts::PU_HELMET,
        "onelife" => &ts::PU_TANK,
        "clock" => &ts::PU_CLOCK,
        "shovel" => &ts::PU_SHOVEL,
        _ => return None,
    })
}

fn render_all_tank() {
    for name in [
        "p1", "p2", "basic", "fast", "power", "armor", "brick", "steel", "water", "bush", "ice",
        "base", "star", "grenade", "helmet", "onelife", "clock", "shovel",
    ] {
        if let Some(def) = tank_sprite(name) {
            render_sprite(&format!("tank_{name}"), def, Color::srgb(0.10, 0.10, 0.13));
        }
    }
}

fn space_sprite(name: &str) -> Option<&'static SpriteDef> {
    Some(match name {
        "player" | "ship" => &ss::SHIP_PLAYER,
        "scout" => &ss::ENEMY_SCOUT,
        "sniper" => &ss::ENEMY_SNIPER,
        "bomber" => &ss::ENEMY_BOMBER,
        "carrier" => &ss::ENEMY_CARRIER,
        "boss" => &ss::ENEMY_BOSS,
        "powerup" => &ss::POWERUP_P,
        _ => return None,
    })
}

fn render_all_space() {
    for name in ["player", "scout", "sniper", "bomber", "carrier", "boss", "powerup"] {
        if let Some(def) = space_sprite(name) {
            render_sprite(&format!("space_{name}"), def, Color::srgb(0.03, 0.05, 0.10));
        }
    }
}

fn bomb_sprite(name: &str) -> Option<&'static SpriteDef> {
    Some(match name {
        "player1" | "p1" => &bms::BM_PLAYER1,
        "player2" | "p2" => &bms::BM_PLAYER2,
        "balloom" => &bms::ENEMY_BALLOOM,
        "oneal" => &bms::ENEMY_ONEAL,
        "doll" => &bms::ENEMY_DOLL,
        "kondoria" => &bms::ENEMY_KONDORIA,
        "bomb" => &bms::BOMB_DEF,
        "hard" => &bms::HARD_WALL,
        "soft" => &bms::SOFT_WALL,
        "flame" => &bms::FLAME_DEF,
        "exit" => &bms::EXIT_DEF,
        _ => return None,
    })
}

fn render_all_bomb() {
    for name in [
        "player1", "player2", "balloom", "oneal", "doll", "kondoria", "bomb", "hard", "soft",
        "flame", "exit",
    ] {
        if let Some(def) = bomb_sprite(name) {
            render_sprite(&format!("bomb_{name}"), def, Color::srgb(0.16, 0.28, 0.16));
        }
    }
}

fn memory_sprite(name: &str) -> Option<&'static SpriteDef> {
    Some(match name {
        "back" => &ms::CARD_BACK,
        "face" => &ms::CARD_FACE,
        "matched" | "done" => &ms::CARD_FACE_MATCHED,
        _ => return None,
    })
}

fn render_all_memory() {
    for name in ["back", "face", "matched"] {
        if let Some(def) = memory_sprite(name) {
            render_sprite(&format!("memory_{name}"), def, Color::srgb(0.07, 0.09, 0.15));
        }
    }
}

fn soko_sprite(name: &str) -> Option<&'static SpriteDef> {
    Some(match name {
        "wall" => &sk::WALL_TILE,
        "floor" => &sk::FLOOR_TILE,
        "goal" => &sk::GOAL_TILE,
        "box" => &sk::BOX_WOOD,
        "boxdone" | "done" => &sk::BOX_DONE,
        "player" => &sk::SOKO_PLAYER,
        _ => return None,
    })
}

fn render_all_soko() {
    for name in ["wall", "floor", "goal", "box", "boxdone", "player"] {
        if let Some(def) = soko_sprite(name) {
            render_sprite(&format!("soko_{name}"), def, Color::srgb(0.10, 0.13, 0.19));
        }
    }
}

fn scene_bg() -> Color {
    cp::COLOR_SKY_HI
}

fn render_all_contra() {
    for name in [
        "player", "prone", "flip", "soldier", "sniper", "jumper", "heavy", "gunner", "falcon", "boss",
    ] {
        if let Some(def) = contra_sprite(name) {
            render_sprite(&format!("contra_{name}"), def, scene_bg());
        }
    }
    render_contra_scene();
}

/// 单精灵：x8 放大图 + 实机尺寸图（按 1.333× 采样后整数放大）。
fn render_sprite(stem: &str, def: &SpriteDef, bg: Color) {
    let pad = 3.0;
    let bb = sprite_bbox(def);
    let view = View {
        min_x: bb.min_x - pad,
        min_y: bb.min_y - pad,
        max_x: bb.max_x + pad,
        max_y: bb.max_y + pad,
    };

    let mut big = Canvas::new(view, X8, bg);
    big.draw_sprite(def, Vec2::ZERO);
    big.save(&format!("{OUT_DIR}/{stem}_x8.png"), 1);

    let mut real = Canvas::new(view, REAL_SCALE, bg);
    real.draw_sprite(def, Vec2::ZERO);
    real.save(&format!("{OUT_DIR}/{stem}_real.png"), REAL_ZOOM);

    println!("✓ {stem}: 包络 {:.0}×{:.0}，实机约 {:.0}px 宽", def.size.x, def.size.y, def.size.x * REAL_SCALE);
}

/// 实战场景：Bill 与三种敌兵脚底对齐地面线，贴游戏夜空背景，检查互相可读与等高关系。
fn render_contra_scene() {
    let roster: [(&SpriteDef, &str); 4] = [
        (&cs::PLAYER_BILL, "Bill"),
        (&cs::ENEMY_SOLDIER, "Soldier"),
        (&cs::ENEMY_SNIPER, "Sniper"),
        (&cs::ENEMY_JUMPER, "Jumper"),
    ];
    let ground_y = 0.0;
    let spacing = 40.0;
    let view = View {
        min_x: -22.0,
        min_y: -12.0,
        max_x: spacing * roster.len() as f32 - 6.0,
        max_y: 38.0,
    };
    let mut canvas = Canvas::new(view, X8, cp::COLOR_SKY_HI);
    // 草皮 + 岩壁地面带
    canvas.fill_band(ground_y - 3.0, 6.0, cp::COLOR_GRASS_MID);
    canvas.fill_band(ground_y - 9.0, 8.0, cp::COLOR_ROCK_MID);
    for (i, (def, _)) in roster.iter().enumerate() {
        let x = i as f32 * spacing;
        // 脚底（包络底 = origin.y - size.y/2）落在 ground_y
        let oy = ground_y + def.size.y * 0.5;
        canvas.draw_sprite(def, Vec2::new(x, oy));
    }
    canvas.save(&format!("{OUT_DIR}/contra_scene.png"), 1);
    // 同时出一张实机尺寸的场景，检查小尺寸可读性
    let mut real = Canvas::new(view, REAL_SCALE, cp::COLOR_SKY_HI);
    real.fill_band(ground_y - 3.0, 6.0, cp::COLOR_GRASS_MID);
    real.fill_band(ground_y - 9.0, 8.0, cp::COLOR_ROCK_MID);
    for (i, (def, _)) in roster.iter().enumerate() {
        let x = i as f32 * spacing;
        let oy = ground_y + def.size.y * 0.5;
        real.draw_sprite(def, Vec2::new(x, oy));
    }
    real.save(&format!("{OUT_DIR}/contra_scene_real.png"), REAL_ZOOM);
    println!("✓ contra_scene: {} 个角色并列", roster.len());
}

/// 把"我的精灵图"与参考截图并排（参考图等比缩到等高），便于校准配色/姿势。
fn compare(mine_path: &str, ref_path: &str, out_path: &str) {
    let mine = image::open(mine_path)
        .unwrap_or_else(|e| panic!("打不开 {mine_path}: {e}"))
        .to_rgba8();
    let reference = image::open(ref_path)
        .unwrap_or_else(|e| panic!("打不开 {ref_path}: {e}"))
        .to_rgba8();
    let h = mine.height().max(reference.height());
    let scale_to_h = |img: &image::RgbaImage| -> image::RgbaImage {
        if img.height() == h {
            img.clone()
        } else {
            let nw = (img.width() as f32 * h as f32 / img.height() as f32).round() as u32;
            image::imageops::resize(img, nw.max(1), h, image::imageops::FilterType::Nearest)
        }
    };
    let a = scale_to_h(&mine);
    let b = scale_to_h(&reference);
    let gap = 8u32;
    let out_w = a.width() + gap + b.width();
    let mut out = image::RgbaImage::from_pixel(out_w, h, image::Rgba([24, 24, 28, 255]));
    image::imageops::overlay(&mut out, &a, 0, 0);
    image::imageops::overlay(&mut out, &b, (a.width() + gap) as i64, 0);
    out.save(out_path)
        .unwrap_or_else(|e| panic!("写 {out_path} 失败: {e}"));
    println!("✓ 并排图 {out_path}（左=我的 {}px，右=参考 {}px）", a.width(), b.width());
}
