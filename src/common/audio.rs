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

use bevy::audio::{AddAudioSource, AudioPlayer, Decodable, PlaybackSettings, Source};
use bevy::prelude::*;

pub const SFX_SAMPLE_RATE: u32 = 22_050;

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

/// 播放一个音效：任何系统写入该消息即可。
#[derive(Message)]
pub struct PlaySfx(pub SfxKind);

/// 消费本帧全部 [`PlaySfx`]，同帧同类去重后生成一次性播放实体。
pub fn sfx_playback(
    mut reader: MessageReader<PlaySfx>,
    assets: Res<SfxAssets>,
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
            PlaybackSettings::DESPAWN,
        ));
    }
}

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<Sfx>()
            .add_message::<PlaySfx>()
            .init_resource::<SfxAssets>()
            .add_systems(Update, sfx_playback);
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
}
