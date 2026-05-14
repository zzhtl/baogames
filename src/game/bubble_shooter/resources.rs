use bevy::asset::RenderAssetUsages;
use bevy::image::Image;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

#[derive(Resource)]
pub struct BubbleStage {
    pub grid: Vec<Vec<Option<u8>>>,
    pub descend: usize,
    pub aim: f32,
    pub current: u8,
    pub next: u8,
    pub shot_active: bool,
    pub shots_left_for_descend: i32,
    pub max_shots_per_descend: i32,
    #[allow(dead_code)]
    pub colors_count: u8,
    pub message: String,
    pub message_clock: f32,
    pub flash_clock: f32,
}

/// 圆形泡泡的预绘制贴图：球体 + 高光。颜色通过 `Sprite.color` 染色。
#[derive(Resource, Clone)]
pub struct BubbleAssets {
    pub ball: Handle<Image>,
    pub highlight: Handle<Image>,
}

impl FromWorld for BubbleAssets {
    fn from_world(world: &mut World) -> Self {
        let ball = build_ball_image();
        let highlight = build_highlight_image();
        let mut images = world.resource_mut::<Assets<Image>>();
        BubbleAssets {
            ball: images.add(ball),
            highlight: images.add(highlight),
        }
    }
}

fn build_ball_image() -> Image {
    // 64x64 的圆球，渐变明暗模拟立体感。Alpha 在边缘抗锯齿。
    const SIZE: u32 = 64;
    let r = SIZE as f32 / 2.0;
    let mut data = vec![0u8; (SIZE * SIZE * 4) as usize];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as f32 + 0.5 - r;
            let dy = y as f32 + 0.5 - r;
            let dist = (dx * dx + dy * dy).sqrt();
            let i = ((y * SIZE + x) * 4) as usize;
            let edge_outer = r - 0.5;
            let edge_inner = r - 2.0;
            let alpha = if dist <= edge_inner {
                1.0
            } else if dist <= edge_outer {
                ((edge_outer - dist) / (edge_outer - edge_inner)).clamp(0.0, 1.0)
            } else {
                0.0
            };
            // 立体着色：左上偏亮，右下偏暗（球面 Lambert 简化）
            let nx = dx / r;
            let ny = dy / r;
            let z2 = (1.0 - nx * nx - ny * ny).max(0.0);
            let z = z2.sqrt();
            let light = (-0.45 * nx - 0.45 * ny + 0.75 * z).clamp(0.0, 1.0);
            let value = (0.55 + 0.45 * light).clamp(0.0, 1.0);
            let v8 = (value * 255.0) as u8;
            data[i] = v8;
            data[i + 1] = v8;
            data[i + 2] = v8;
            data[i + 3] = (alpha * 255.0) as u8;
        }
    }
    Image::new(
        Extent3d {
            width: SIZE,
            height: SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

fn build_highlight_image() -> Image {
    // 32x32 的柔和白色高光圆点（径向衰减），叠加在球的左上方。
    const SIZE: u32 = 32;
    let r = SIZE as f32 / 2.0;
    let mut data = vec![0u8; (SIZE * SIZE * 4) as usize];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as f32 + 0.5 - r;
            let dy = y as f32 + 0.5 - r;
            let d = (dx * dx + dy * dy).sqrt() / r;
            let i = ((y * SIZE + x) * 4) as usize;
            let alpha = (1.0 - d.clamp(0.0, 1.0)).powf(2.2);
            data[i] = 255;
            data[i + 1] = 255;
            data[i + 2] = 255;
            data[i + 3] = (alpha * 255.0) as u8;
        }
    }
    Image::new(
        Extent3d {
            width: SIZE,
            height: SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}
