//! 保持像素游戏布局、按窗口实际像素渲染的画布。
//!
//! 240×180 / 320×180 只用于布局与整数倍适配。纹理分辨率匹配窗口上的实际
//! 显示区域（含 DPI），文字与小图形不再经过低分辨率降采样。逻辑世界仍高 540。

use bevy::camera::{ImageRenderTarget, RenderTarget};
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};

use super::constants::ARENA_H;
use super::px::snap;
use super::audio::{PlaySfx, SfxKind};
use super::settings::DisplayMode;

const GAME_LAYERS: RenderLayers = RenderLayers::layer(0);
const PRESENT_LAYERS: RenderLayers = RenderLayers::layer(1);

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelCanvasConfig {
    pub display_mode: DisplayMode,
    pub crt_enabled: bool,
    pub shake_enabled: bool,
}

impl Default for PixelCanvasConfig {
    fn default() -> Self {
        Self {
            display_mode: DisplayMode::Classic4x3,
            crt_enabled: false,
            shake_enabled: false,
        }
    }
}

#[derive(Component)]
pub struct InGameCamera;

#[derive(Component)]
struct OuterCamera;

#[derive(Component)]
struct Canvas;

#[derive(Component)]
struct CrtScanline;

#[derive(Resource, Default)]
struct ScreenShake {
    remaining: f32,
    duration: f32,
}

#[derive(Resource)]
struct CanvasImage(Handle<Image>);

fn render_size(mode: DisplayMode, logical_size: Vec2, dpi: f32) -> (u32, u32) {
    let (width, height) = mode.canvas_size();
    let scale = (logical_size.x / width as f32).min(logical_size.y / height as f32).floor().max(1.0) * dpi;
    ((width as f32 * scale).round() as u32, (height as f32 * scale).round() as u32)
}

/// 窗口坐标转静止关卡坐标；黑边区域不参与点击。
pub fn cursor_world(window: &Window, config: &PixelCanvasConfig, cursor: Vec2) -> Option<Vec2> {
    let (w, h) = config.display_mode.canvas_size();
    let scale = (window.width() / w as f32).min(window.height() / h as f32).floor().max(1.0);
    let pos = (cursor - Vec2::new(window.width(), window.height()) * 0.5)
        * Vec2::new(1.0, -1.0) * (super::px::WORLD_PER_PX / scale);
    (pos.x.abs() <= config.display_mode.world_width() * 0.5 && pos.y.abs() <= ARENA_H * 0.5).then_some(pos)
}

fn create_canvas((width, height): (u32, u32)) -> Image {
    let size = Extent3d {
        width,
        height,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("baogames pixel canvas"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);
    image
}

fn setup_pixel_canvas(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    config: Res<PixelCanvasConfig>,
    window: Single<&Window>,
) {
    let size = render_size(config.display_mode, Vec2::new(window.width(), window.height()), window.scale_factor());
    let image_handle = images.add(create_canvas(size));
    commands.insert_resource(CanvasImage(image_handle.clone()));

    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.02, 0.02, 0.025)),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::Fixed {
                width: config.display_mode.world_width(),
                height: ARENA_H,
            },
            ..OrthographicProjection::default_2d()
        }),
        // 字形直接按最终屏幕密度光栅化，避免先缩小再放大造成细笔画丢失。
        RenderTarget::Image(ImageRenderTarget {
            handle: image_handle.clone(),
            scale_factor: size.1 as f32 / ARENA_H,
        }),
        Msaa::Sample4,
        InGameCamera,
        GAME_LAYERS,
    ));

    let (width, height) = config.display_mode.canvas_size();
    commands.spawn((
        Sprite {
            image: image_handle,
            custom_size: Some(Vec2::new(width as f32, height as f32)),
            ..default()
        },
        Canvas,
        PRESENT_LAYERS,
    ));

    let visibility = if config.crt_enabled {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    for row in 0..height / 2 {
        let y = -(height as f32) * 0.5 + row as f32 * 2.0 + 0.5;
        commands.spawn((
            Sprite::from_color(
                Color::srgba(0.0, 0.0, 0.02, 0.16),
                Vec2::new(width as f32, 1.0),
            ),
            Transform::from_xyz(0.0, y, 1.0),
            visibility,
            CrtScanline,
            PRESENT_LAYERS,
        ));
    }

    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Msaa::Off,
        OuterCamera,
        PRESENT_LAYERS,
    ));
}

fn sync_canvas_mode(
    config: Res<PixelCanvasConfig>,
    canvas_image: Res<CanvasImage>,
    mut images: ResMut<Assets<Image>>,
    mut game_camera: Query<(&mut Projection, &mut RenderTarget), With<InGameCamera>>,
    window: Single<Ref<Window>>,
    // 两个查询都要 &mut Sprite，Bevy 无法自己证明 With<Canvas> 与 With<CrtScanline>
    // 不相交，必须显式 Without 拆开——否则系统初始化时直接 panic(B0001)。
    mut canvas_sprite: Query<&mut Sprite, (With<Canvas>, Without<CrtScanline>)>,
    mut scanlines: Query<(&mut Sprite, &mut Visibility), With<CrtScanline>>,
) {
    if !config.is_changed() && !window.is_changed() {
        return;
    }
    let (width, height) = config.display_mode.canvas_size();
    let native = render_size(config.display_mode, Vec2::new(window.width(), window.height()), window.scale_factor());
    let size = Extent3d { width: native.0, height: native.1, ..default() };
    if images.get(&canvas_image.0).is_some_and(|image| image.texture_descriptor.size != size)
        && let Some(image) = images.get_mut(&canvas_image.0)
    {
        image.resize(size);
    }
    if let Ok((mut projection, mut target)) = game_camera.single_mut() {
        if let RenderTarget::Image(target) = &mut *target {
            target.scale_factor = native.1 as f32 / ARENA_H;
        }
        if let Projection::Orthographic(projection) = &mut *projection {
            projection.scaling_mode = bevy::camera::ScalingMode::Fixed {
                width: config.display_mode.world_width(),
                height: ARENA_H,
            };
        }
    }
    if let Ok(mut sprite) = canvas_sprite.single_mut() {
        sprite.custom_size = Some(Vec2::new(width as f32, height as f32));
    }
    for (mut sprite, mut visibility) in &mut scanlines {
        sprite.custom_size = Some(Vec2::new(width as f32, 1.0));
        *visibility = if config.crt_enabled {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn shake_from_sfx(
    config: Res<PixelCanvasConfig>,
    mut events: MessageReader<PlaySfx>,
    mut shake: ResMut<ScreenShake>,
) {
    if !config.shake_enabled {
        for _ in events.read() {}
        return;
    }
    for PlaySfx(kind) in events.read() {
        let duration: f32 = match kind {
            SfxKind::ExplosionBig => 0.22,
            SfxKind::Explosion => 0.14,
            SfxKind::Hit | SfxKind::Stomp => 0.08,
            _ => 0.0,
        };
        if duration > 0.0 {
            shake.duration = shake.duration.max(duration);
            shake.remaining = shake.remaining.max(duration);
        }
    }
}

fn apply_camera_shake(
    time: Res<Time>,
    config: Res<PixelCanvasConfig>,
    mut shake: ResMut<ScreenShake>,
    mut camera: Query<&mut Transform, With<InGameCamera>>,
) {
    let Ok(mut transform) = camera.single_mut() else {
        return;
    };
    if !config.shake_enabled || shake.remaining <= 0.0 {
        shake.remaining = 0.0;
        transform.translation.y = 0.0;
        return;
    }
    shake.remaining = (shake.remaining - time.delta_secs()).max(0.0);
    let strength = if shake.duration > 0.0 {
        shake.remaining / shake.duration
    } else {
        0.0
    };
    let wave = (time.elapsed_secs() * 73.0).sin() + (time.elapsed_secs() * 41.0).cos() * 0.5;
    // 必须吸附到画布像素网格：舍到世界单位会留下 1/3 像素的偏移，整屏 shimmer
    transform.translation.y = snap(wave * 4.0 * strength);
}

fn fit_canvas(
    config: Res<PixelCanvasConfig>,
    window: Single<&Window>,
    mut projection: Single<&mut Projection, With<OuterCamera>>,
) {
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };
    let (width, height) = config.display_mode.canvas_size();
    let horizontal = window.width() / width as f32;
    let vertical = window.height() / height as f32;
    let integer_scale = horizontal.min(vertical).floor().max(1.0);
    projection.scale = 1.0 / integer_scale;
}

pub struct PixelCanvasPlugin;

impl Plugin for PixelCanvasPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PixelCanvasConfig>()
            .init_resource::<ScreenShake>()
            .add_systems(Startup, setup_pixel_canvas)
            .add_systems(Update, (sync_canvas_mode, fit_canvas, shake_from_sfx).chain())
            .add_systems(PostUpdate, apply_camera_shake);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_target_matches_native_pixels_and_dpi() {
        let size = Vec2::new(1280.0, 720.0);
        assert_eq!(render_size(DisplayMode::Classic4x3, size, 1.0), (960, 720));
        assert_eq!(render_size(DisplayMode::Widescreen16x9, size, 1.0), (1280, 720));
        assert_eq!(render_size(DisplayMode::Classic4x3, size, 2.0), (1920, 1440));
        assert_eq!(render_size(DisplayMode::Classic4x3, size, 1.25), (1200, 900));
    }

    #[test]
    fn pointer_mapping_excludes_black_bars_and_tracks_dpi() {
        let mut window = Window { resolution: bevy::window::WindowResolution::new(1280, 720), ..default() };
        let config = PixelCanvasConfig::default();
        assert_eq!(cursor_world(&window, &config, Vec2::new(640.0, 360.0)), Some(Vec2::ZERO));
        assert_eq!(cursor_world(&window, &config, Vec2::new(160.0, 360.0)), Some(Vec2::new(-360.0, 0.0)));
        assert!(cursor_world(&window, &config, Vec2::new(80.0, 360.0)).is_none());
        window.resolution.set_scale_factor_override(Some(2.0));
        assert_eq!(cursor_world(&window, &config, Vec2::new(320.0, 180.0)), Some(Vec2::ZERO));
        assert_eq!(cursor_world(&window, &config, Vec2::new(80.0, 180.0)), Some(Vec2::new(-360.0, 0.0)));
    }

    #[test]
    fn canvas_uses_three_world_units_per_pixel() {
        for mode in [DisplayMode::Classic4x3, DisplayMode::Widescreen16x9] {
            let (width, height) = mode.canvas_size();
            assert_eq!(height as f32 * 3.0, ARENA_H);
            assert_eq!(width as f32 * 3.0, mode.world_width());
        }
    }
}
