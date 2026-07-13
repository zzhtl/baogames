//! 程序化 8-bit 音效系统：波形全部由代码生成，无外部音频文件。
//!
//! - [`Sfx`] 是自定义音频资产（内存 f32 单声道样本），经 [`Decodable`]
//!   接入 bevy_audio（参照官方 `Pitch` 模板），零新增依赖。
//! - 波形生成是纯函数（方波扫频 / 三角波 / 噪声爆发），噪声用固定种子
//!   xorshift，结果确定可单测。
//! - 播放走 [`PlaySfx`] 消息：任何系统 `MessageWriter<PlaySfx>` 一行触发，
//!   [`sfx_playback`] 统一消费并做同帧同类去重。

use std::sync::Arc;
use std::time::Duration;

use bevy::audio::{
    AddAudioSource, AudioPlayer, AudioSink, AudioSinkPlayback, Decodable, PlaybackSettings, Source,
    Volume,
};
use bevy::prelude::*;

pub const SFX_SAMPLE_RATE: u32 = 22_050;

#[derive(Resource, Clone, Copy)]
pub struct AudioMix {
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for AudioMix {
    fn default() -> Self {
        Self {
            music_volume: 0.7,
            sfx_volume: 0.8,
        }
    }
}

// ---------- 资产与解码器 ----------

/// 内存中的单声道 f32 音效样本。
#[derive(Asset, TypePath, Clone)]
pub struct Sfx {
    pub samples: Arc<[f32]>,
}

pub struct SfxDecoder {
    samples: Arc<[f32]>,
    pos: usize,
}

impl Iterator for SfxDecoder {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let s = self.samples.get(self.pos).copied();
        self.pos += 1;
        s
    }
}

impl Source for SfxDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        // 与 rodio SamplesBuffer 语义一致：单一 frame 直到结束
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        SFX_SAMPLE_RATE
    }
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f64(
            self.samples.len() as f64 / SFX_SAMPLE_RATE as f64,
        ))
    }
}

impl Decodable for Sfx {
    type DecoderItem = f32;
    type Decoder = SfxDecoder;
    fn decoder(&self) -> SfxDecoder {
        SfxDecoder {
            samples: self.samples.clone(),
            pos: 0,
        }
    }
}

// ---------- 波形生成（纯函数） ----------

/// 尾部 5ms 线性淡出，防止波形在非零处截断产生爆音。
fn fade_out_tail(samples: &mut [f32]) {
    let fade = (SFX_SAMPLE_RATE as f32 * 0.005) as usize;
    let n = samples.len();
    if n < fade || fade == 0 {
        return;
    }
    for i in 0..fade {
        let k = i as f32 / fade as f32;
        samples[n - 1 - i] *= k;
    }
}

/// 方波：频率从 `f0` 线性扫到 `f1`，`duty` 为占空比 (0,1)，`gain` 为振幅。
pub fn square_sweep(f0: f32, f1: f32, dur: f32, duty: f32, gain: f32) -> Vec<f32> {
    let n = (dur * SFX_SAMPLE_RATE as f32) as usize;
    let mut out = Vec::with_capacity(n);
    let mut phase = 0.0_f32;
    for i in 0..n {
        let t = i as f32 / n.max(1) as f32;
        let freq = f0 + (f1 - f0) * t;
        phase = (phase + freq / SFX_SAMPLE_RATE as f32).fract();
        out.push(if phase < duty { gain } else { -gain });
    }
    fade_out_tail(&mut out);
    out
}

/// 三角波扫频（比方波柔和，适合金币 / 加分）。
pub fn triangle_sweep(f0: f32, f1: f32, dur: f32, gain: f32) -> Vec<f32> {
    let n = (dur * SFX_SAMPLE_RATE as f32) as usize;
    let mut out = Vec::with_capacity(n);
    let mut phase = 0.0_f32;
    for i in 0..n {
        let t = i as f32 / n.max(1) as f32;
        let freq = f0 + (f1 - f0) * t;
        phase = (phase + freq / SFX_SAMPLE_RATE as f32).fract();
        // 0..1 相位映射为 -1..1..-1 三角
        let tri = if phase < 0.5 {
            phase * 4.0 - 1.0
        } else {
            3.0 - phase * 4.0
        };
        out.push(tri * gain);
    }
    fade_out_tail(&mut out);
    out
}

/// 白噪声爆发 + 一阶低通（`lp` ∈ (0,1]，越小越闷）+ 指数衰减，用于爆炸 / 受击。
pub fn noise_burst(dur: f32, gain: f32, lp: f32) -> Vec<f32> {
    let n = (dur * SFX_SAMPLE_RATE as f32) as usize;
    let mut out = Vec::with_capacity(n);
    // 固定种子 xorshift32：结果确定，便于测试与重现
    let mut state: u32 = 0x2545_F491;
    let mut filtered = 0.0_f32;
    for i in 0..n {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        let white = (state as f32 / u32::MAX as f32) * 2.0 - 1.0;
        filtered += lp * (white - filtered);
        let decay = 1.0 - i as f32 / n.max(1) as f32;
        out.push(filtered * gain * decay * decay);
    }
    fade_out_tail(&mut out);
    out
}

/// 顺序拼接多段样本（琶音 / 组合音效）。
pub fn concat(parts: &[Vec<f32>]) -> Vec<f32> {
    let mut out = Vec::with_capacity(parts.iter().map(Vec::len).sum());
    for p in parts {
        out.extend_from_slice(p);
    }
    out
}

/// 两段逐样本混合，取较长者长度。
pub fn mix(a: &[f32], b: &[f32]) -> Vec<f32> {
    let n = a.len().max(b.len());
    (0..n)
        .map(|i| {
            (a.get(i).copied().unwrap_or(0.0) + b.get(i).copied().unwrap_or(0.0)).clamp(-1.0, 1.0)
        })
        .collect()
}

// ---------- 音效种类与配方 ----------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SfxKind {
    MenuMove,
    MenuConfirm,
    Pause,
    Shoot,
    Explosion,
    ExplosionBig,
    Jump,
    Stomp,
    Coin,
    Powerup,
    Hit,
    Place,
    Flip,
    Match,
    Deny,
    Win,
    Lose,
}

impl SfxKind {
    pub const COUNT: usize = 17;
    pub const ALL: [SfxKind; Self::COUNT] = [
        SfxKind::MenuMove,
        SfxKind::MenuConfirm,
        SfxKind::Pause,
        SfxKind::Shoot,
        SfxKind::Explosion,
        SfxKind::ExplosionBig,
        SfxKind::Jump,
        SfxKind::Stomp,
        SfxKind::Coin,
        SfxKind::Powerup,
        SfxKind::Hit,
        SfxKind::Place,
        SfxKind::Flip,
        SfxKind::Match,
        SfxKind::Deny,
        SfxKind::Win,
        SfxKind::Lose,
    ];

    fn index(self) -> usize {
        Self::ALL.iter().position(|k| *k == self).unwrap_or(0)
    }
}

pub fn build_sfx(kind: SfxKind) -> Vec<f32> {
    match kind {
        SfxKind::MenuMove => square_sweep(880.0, 880.0, 0.045, 0.5, 0.25),
        SfxKind::MenuConfirm => square_sweep(660.0, 990.0, 0.09, 0.5, 0.3),
        SfxKind::Pause => square_sweep(550.0, 550.0, 0.05, 0.25, 0.3),
        SfxKind::Shoot => square_sweep(1100.0, 350.0, 0.08, 0.25, 0.22),
        SfxKind::Explosion => mix(
            &noise_burst(0.32, 0.5, 0.25),
            &square_sweep(120.0, 55.0, 0.3, 0.5, 0.3),
        ),
        SfxKind::ExplosionBig => mix(
            &noise_burst(0.6, 0.6, 0.18),
            &square_sweep(90.0, 40.0, 0.55, 0.5, 0.35),
        ),
        SfxKind::Jump => square_sweep(280.0, 660.0, 0.16, 0.5, 0.3),
        SfxKind::Stomp => square_sweep(520.0, 150.0, 0.1, 0.5, 0.35),
        SfxKind::Coin => concat(&[
            triangle_sweep(988.0, 988.0, 0.06, 0.3),
            triangle_sweep(1319.0, 1319.0, 0.14, 0.3),
        ]),
        SfxKind::Powerup => concat(&[
            square_sweep(523.0, 523.0, 0.06, 0.5, 0.28),
            square_sweep(659.0, 659.0, 0.06, 0.5, 0.28),
            square_sweep(784.0, 784.0, 0.06, 0.5, 0.28),
            square_sweep(1047.0, 1047.0, 0.09, 0.5, 0.28),
        ]),
        SfxKind::Hit => mix(
            &noise_burst(0.09, 0.4, 0.5),
            &square_sweep(200.0, 90.0, 0.08, 0.5, 0.3),
        ),
        SfxKind::Place => square_sweep(240.0, 200.0, 0.05, 0.5, 0.3),
        SfxKind::Flip => square_sweep(700.0, 900.0, 0.045, 0.5, 0.25),
        SfxKind::Match => concat(&[
            square_sweep(660.0, 660.0, 0.06, 0.5, 0.3),
            square_sweep(1320.0, 1320.0, 0.1, 0.5, 0.3),
        ]),
        SfxKind::Deny => square_sweep(220.0, 110.0, 0.16, 0.5, 0.3),
        SfxKind::Win => concat(&[
            triangle_sweep(523.0, 523.0, 0.09, 0.32),
            triangle_sweep(659.0, 659.0, 0.09, 0.32),
            triangle_sweep(784.0, 784.0, 0.09, 0.32),
            triangle_sweep(1047.0, 1047.0, 0.09, 0.32),
            triangle_sweep(1319.0, 1319.0, 0.25, 0.32),
        ]),
        SfxKind::Lose => concat(&[
            triangle_sweep(392.0, 392.0, 0.18, 0.3),
            triangle_sweep(330.0, 330.0, 0.18, 0.3),
            triangle_sweep(262.0, 262.0, 0.3, 0.3),
        ]),
    }
}

// ---------- 原创芯片音乐 ----------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MusicKind {
    Menu,
    Tank,
    BombMaze,
    SpaceShooter,
    SuperMario,
    Contra,
    BubbleShooter,
    MemoryMatch,
    Sokoban,
}

impl MusicKind {
    pub const COUNT: usize = 9;
    pub const ALL: [Self; Self::COUNT] = [
        Self::Menu,
        Self::Tank,
        Self::BombMaze,
        Self::SpaceShooter,
        Self::SuperMario,
        Self::Contra,
        Self::BubbleShooter,
        Self::MemoryMatch,
        Self::Sokoban,
    ];

    fn index(self) -> usize {
        Self::ALL.iter().position(|kind| *kind == self).unwrap_or(0)
    }
}

fn midi_frequency(note: i8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}

fn chip_note(note: i8, duration: f32, duty: f32, gain: f32, triangle: bool) -> Vec<f32> {
    let sample_count = (duration * SFX_SAMPLE_RATE as f32) as usize;
    if note < 0 {
        return vec![0.0; sample_count];
    }
    let frequency = midi_frequency(note);
    let attack = (SFX_SAMPLE_RATE as f32 * 0.004) as usize;
    let release = (SFX_SAMPLE_RATE as f32 * 0.025) as usize;
    let mut phase = 0.0_f32;
    (0..sample_count)
        .map(|index| {
            phase = (phase + frequency / SFX_SAMPLE_RATE as f32).fract();
            let wave = if triangle {
                if phase < 0.5 {
                    phase * 4.0 - 1.0
                } else {
                    3.0 - phase * 4.0
                }
            } else if phase < duty {
                1.0
            } else {
                -1.0
            };
            let attack_gain = (index as f32 / attack.max(1) as f32).min(1.0);
            let remaining = sample_count.saturating_sub(index + 1);
            let release_gain = (remaining as f32 / release.max(1) as f32).min(1.0);
            wave * gain * attack_gain * release_gain
        })
        .collect()
}

fn sequence(pattern: &[i8], step_seconds: f32, duty: f32, gain: f32, triangle: bool) -> Vec<f32> {
    let mut samples = Vec::with_capacity(
        (pattern.len() as f32 * step_seconds * SFX_SAMPLE_RATE as f32) as usize,
    );
    for &note in pattern {
        samples.extend(chip_note(note, step_seconds, duty, gain, triangle));
    }
    samples
}

/// 九首短循环均为本项目原创动机，只借用早期主机的方波/三角波音色。
pub fn build_music(kind: MusicKind) -> Vec<f32> {
    let (bpm, lead, bass): (f32, &[i8], &[i8]) = match kind {
        MusicKind::Menu => (132.0, &[72, 76, 79, 84, 79, 76, 74, 79, 77, 81, 84, 81, 79, 74, 76, -1], &[48, 48, 55, 55, 53, 53, 55, 55, 50, 50, 57, 57, 55, 55, 43, 43]),
        MusicKind::Tank => (118.0, &[55, 55, 58, 55, 62, -1, 60, 58, 55, 58, 63, 62, 58, -1, 53, 55], &[36, 36, 36, 43, 39, 39, 41, 41, 36, 36, 43, 43, 41, 41, 34, 34]),
        MusicKind::BombMaze => (148.0, &[67, 70, 74, 70, 65, 69, 72, 69, 67, 72, 75, 72, 70, 67, 65, -1], &[43, 43, 46, 46, 41, 41, 45, 45, 43, 43, 48, 48, 46, 46, 41, 41]),
        MusicKind::SpaceShooter => (156.0, &[76, 79, 83, 86, 83, 79, 78, 81, 84, 88, 84, 81, 79, 83, 86, 91], &[40, 47, 40, 47, 42, 49, 42, 49, 43, 50, 43, 50, 38, 45, 38, 45]),
        MusicKind::SuperMario => (164.0, &[72, 76, 79, 76, 74, 77, 81, 77, 76, 79, 83, 79, 77, 74, 72, 67], &[48, 55, 52, 55, 50, 57, 53, 57, 48, 55, 52, 55, 50, 57, 43, 43]),
        MusicKind::Contra => (172.0, &[64, 67, 69, 64, 72, 69, 67, 64, 65, 69, 72, 77, 72, 69, 67, 62], &[40, 40, 47, 40, 41, 41, 48, 41, 43, 43, 50, 43, 38, 38, 45, 38]),
        MusicKind::BubbleShooter => (136.0, &[77, 81, 84, 82, 79, 82, 86, 84, 81, 84, 89, 86, 84, 82, 79, 81], &[53, 60, 57, 60, 55, 62, 58, 62, 53, 60, 57, 60, 55, 62, 53, 53]),
        MusicKind::MemoryMatch => (104.0, &[72, -1, 76, 79, 74, -1, 77, 81, 76, -1, 79, 83, 74, 77, 76, -1], &[48, 48, 52, 52, 50, 50, 53, 53, 48, 48, 55, 55, 50, 50, 43, 43]),
        MusicKind::Sokoban => (112.0, &[60, 63, 67, -1, 62, 65, 69, -1, 63, 67, 70, 67, 62, 65, 60, -1], &[36, 43, 39, 43, 38, 45, 41, 45, 39, 46, 43, 46, 38, 45, 36, 36]),
    };
    let step_seconds = 30.0 / bpm;
    let melody = sequence(lead, step_seconds, 0.25, 0.13, false);
    let low = sequence(bass, step_seconds, 0.5, 0.10, true);
    mix(&melody, &low)
}

// ---------- 资源与播放 ----------

#[derive(Resource)]
pub struct SfxAssets {
    handles: Vec<Handle<Sfx>>,
}

impl SfxAssets {
    pub fn handle(&self, kind: SfxKind) -> Handle<Sfx> {
        self.handles[kind.index()].clone()
    }
}

impl FromWorld for SfxAssets {
    fn from_world(world: &mut World) -> Self {
        let mut assets = world.resource_mut::<Assets<Sfx>>();
        let handles = SfxKind::ALL
            .iter()
            .map(|&kind| {
                assets.add(Sfx {
                    samples: build_sfx(kind).into(),
                })
            })
            .collect();
        SfxAssets { handles }
    }
}

#[derive(Resource)]
pub struct MusicAssets {
    handles: Vec<Handle<Sfx>>,
}

impl MusicAssets {
    fn handle(&self, kind: MusicKind) -> Handle<Sfx> {
        self.handles[kind.index()].clone()
    }
}

impl FromWorld for MusicAssets {
    fn from_world(world: &mut World) -> Self {
        let mut assets = world.resource_mut::<Assets<Sfx>>();
        let handles = MusicKind::ALL
            .iter()
            .map(|&kind| {
                assets.add(Sfx {
                    samples: build_music(kind).into(),
                })
            })
            .collect();
        Self { handles }
    }
}

#[derive(Component)]
pub struct MusicEntity;

#[derive(Message)]
pub struct PlayMusic(pub MusicKind);

/// 播放一个音效：任何系统写入该消息即可。
#[derive(Message)]
pub struct PlaySfx(pub SfxKind);

/// 消费本帧全部 [`PlaySfx`]，同帧同类去重后生成一次性播放实体。
pub fn sfx_playback(
    mut reader: MessageReader<PlaySfx>,
    assets: Res<SfxAssets>,
    mix: Res<AudioMix>,
    mut commands: Commands,
) {
    let mut seen = [false; SfxKind::COUNT];
    for PlaySfx(kind) in reader.read() {
        let i = kind.index();
        if seen[i] {
            continue;
        }
        seen[i] = true;
        commands.spawn((
            AudioPlayer::<Sfx>(assets.handle(*kind)),
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(mix.sfx_volume)),
        ));
    }
}

pub fn music_playback(
    mut reader: MessageReader<PlayMusic>,
    assets: Res<MusicAssets>,
    mix: Res<AudioMix>,
    existing: Query<Entity, With<MusicEntity>>,
    mut commands: Commands,
) {
    let Some(PlayMusic(kind)) = reader.read().last() else {
        return;
    };
    for entity in &existing {
        commands.entity(entity).despawn();
    }
    commands.spawn((
        AudioPlayer::<Sfx>(assets.handle(*kind)),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(mix.music_volume)),
        MusicEntity,
    ));
}

fn music_volume_sync(
    mix: Res<AudioMix>,
    mut sinks: Query<&mut AudioSink, With<MusicEntity>>,
) {
    if !mix.is_changed() {
        return;
    }
    for mut sink in &mut sinks {
        sink.set_volume(Volume::Linear(mix.music_volume));
    }
}

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<Sfx>()
            .add_message::<PlaySfx>()
            .add_message::<PlayMusic>()
            .init_resource::<AudioMix>()
            .init_resource::<SfxAssets>()
            .init_resource::<MusicAssets>()
            .add_systems(Update, (sfx_playback, music_playback, music_volume_sync));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waveform_length_matches_duration() {
        let s = square_sweep(440.0, 440.0, 0.5, 0.5, 0.3);
        assert_eq!(s.len(), (0.5 * SFX_SAMPLE_RATE as f32) as usize);
        let t = triangle_sweep(440.0, 880.0, 0.25, 0.3);
        assert_eq!(t.len(), (0.25 * SFX_SAMPLE_RATE as f32) as usize);
    }

    #[test]
    fn all_sfx_nonempty_and_within_amplitude() {
        for kind in SfxKind::ALL {
            let samples = build_sfx(kind);
            assert!(!samples.is_empty(), "{kind:?} 无样本");
            let peak = samples.iter().fold(0.0_f32, |m, s| m.max(s.abs()));
            assert!(peak <= 1.0, "{kind:?} 峰值超限 {peak}");
            assert!(peak > 0.01, "{kind:?} 能量为零");
        }
    }

    #[test]
    fn noise_is_deterministic() {
        assert_eq!(noise_burst(0.1, 0.5, 0.3), noise_burst(0.1, 0.5, 0.3));
    }

    #[test]
    fn kind_index_is_unique_and_total() {
        // ALL 必须收录全部变体且顺序即索引：漏收会让新音效静默退化为 index 0
        for (i, kind) in SfxKind::ALL.iter().enumerate() {
            assert_eq!(kind.index(), i, "{kind:?} 在 ALL 中的位置与 index() 不一致");
        }
        assert_eq!(SfxKind::ALL.len(), SfxKind::COUNT);
    }

    #[test]
    fn tail_fades_to_silence() {
        let s = square_sweep(440.0, 440.0, 0.2, 0.5, 0.3);
        assert!(s.last().copied().unwrap_or(1.0).abs() < 0.01);
    }

    #[test]
    fn decoder_yields_all_samples_and_reports_duration() {
        let sfx = Sfx {
            samples: build_sfx(SfxKind::Coin).into(),
        };
        let n = sfx.samples.len();
        let decoder = sfx.decoder();
        let dur = decoder.total_duration().unwrap();
        assert_eq!(decoder.count(), n);
        let expect = n as f64 / SFX_SAMPLE_RATE as f64;
        assert!((dur.as_secs_f64() - expect).abs() < 1e-6);
    }

    #[test]
    fn mix_takes_longer_length_and_clamps() {
        let a = vec![0.9_f32; 10];
        let b = vec![0.9_f32; 20];
        let m = mix(&a, &b);
        assert_eq!(m.len(), 20);
        assert!(m[0] <= 1.0);
        assert_eq!(m[15], 0.9);
    }

    #[test]
    fn every_music_loop_is_nonempty_and_bounded() {
        for kind in MusicKind::ALL {
            let samples = build_music(kind);
            assert!(samples.len() > SFX_SAMPLE_RATE as usize * 2, "{kind:?} 循环过短");
            let peak = samples.iter().fold(0.0_f32, |max, sample| max.max(sample.abs()));
            assert!(peak <= 1.0, "{kind:?} 峰值超限 {peak}");
            assert!(peak > 0.05, "{kind:?} 能量过低");
            assert!(samples.last().copied().unwrap_or(1.0).abs() < 0.01);
        }
    }
}
