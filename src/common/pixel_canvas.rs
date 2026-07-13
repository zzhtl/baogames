//! 低分辨率像素画布。
//!
//! 游戏世界先渲染到 240×180（4:3）或 320×180（16:9）的纹理，再由外层
//! 相机按整数倍最近邻放大到窗口。逻辑世界始终保持 540 单位高，画布上的一个
//! 像素对应三个世界单位。

use bevy::camera::RenderTarget;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};

use super::constants::ARENA_H;
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

fn create_canvas(mode: DisplayMode) -> Image {
    let (width, height) = mode.canvas_size();
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
) {
    let image_handle = images.add(create_canvas(config.display_mode));
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
        RenderTarget::Image(image_handle.clone().into()),
        Msaa::Off,
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
    mut game_projection: Query<&mut Projection, With<InGameCamera>>,
    mut canvas_sprite: Query<&mut Sprite, With<Canvas>>,
    mut scanlines: Query<(&mut Sprite, &mut Visibility), With<CrtScanline>>,
) {
    if !config.is_changed() {
        return;
    }
    let (width, height) = config.display_mode.canvas_size();
    if let Some(image) = images.get_mut(&canvas_image.0) {
        let size = Extent3d {
            width,
            height,
            ..default()
        };
        image.texture_descriptor.size = size;
        image.resize(size);
    }
    if let Ok(mut projection) = game_projection.single_mut()
        && let Projection::Orthographic(projection) = &mut *projection
    {
        projection.scaling_mode = bevy::camera::ScalingMode::Fixed {
            width: config.display_mode.world_width(),
            height: ARENA_H,
        };
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
    transform.translation.y = (wave * 4.0 * strength).round();
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
    fn canvas_uses_three_world_units_per_pixel() {
        for mode in [DisplayMode::Classic4x3, DisplayMode::Widescreen16x9] {
            let (width, height) = mode.canvas_size();
            assert_eq!(height as f32 * 3.0, ARENA_H);
            assert_eq!(width as f32 * 3.0, mode.world_width());
        }
    }
}
