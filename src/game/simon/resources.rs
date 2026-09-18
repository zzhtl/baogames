use bevy::prelude::*;
use rand::Rng;

/// 一轮的进行阶段。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SimonPhase {
    /// 播放前的准备。
    Ready,
    /// 正在放序列，玩家的按键不算数。
    Playback,
    /// 轮到玩家按。
    Input,
    /// 一轮结束后的定格（对错都有）。
    Feedback,
}

#[derive(Resource)]
pub struct SimonStage {
    pub sequence: Vec<u8>,
    pub phase: SimonPhase,
    pub clock: f32,
    /// 播放到序列的第几个。
    pub play_index: usize,
    /// 玩家已经按对几个。
    pub input_index: usize,
    /// 当前亮着的音板与剩余亮灯时间。
    pub lit: Option<usize>,
    pub lit_clock: f32,
    pub round: u32,
    pub rounds_total: u32,
    pub reverse: bool,
    pub step_time: f32,
    pub input_time: f32,
    /// 本轮是答错了在重来（命还有）。重来时序列长度必须保持不变。
    pub retry: bool,
    /// 命已经扣光，等定格结束就判负。
    pub failed: bool,
    pub message: String,
}

impl SimonStage {
    /// 本轮玩家该按的第 `input_index` 个音板。倒序模式从尾巴往回数。
    pub fn expected(&self) -> Option<u8> {
        let len = self.sequence.len();
        if self.input_index >= len {
            return None;
        }
        let at = if self.reverse {
            len - 1 - self.input_index
        } else {
            self.input_index
        };
        self.sequence.get(at).copied()
    }

    pub fn is_cleared(&self) -> bool {
        self.round > self.rounds_total
    }

    /// 音符之间的间隔。
    pub fn gap_time(&self) -> f32 {
        self.step_time * super::constants::GAP_RATIO
    }

    /// 定格结束，开始下一轮。
    ///
    /// 答错重来时序列**保持原长**原样重放；只有答对了才加长一个。
    /// 抽成方法是为了让这条规则有地方可测 —— 写在 system 里就只能靠手玩验证。
    pub fn begin_next_round<R: Rng + ?Sized>(&mut self, pads: usize, rng: &mut R) {
        if std::mem::take(&mut self.retry) {
            // 原样重放，不加长
        } else {
            extend_sequence(&mut self.sequence, pads, rng);
        }
        self.phase = SimonPhase::Ready;
        self.clock = super::constants::READY_TIME;
        self.input_index = 0;
        self.play_index = 0;
        self.lit = None;
    }
}

/// 按 `len` 生成一段序列。相邻两个不重复 —— 连着两下同一个板听起来像一下。
pub fn build_sequence<R: Rng + ?Sized>(len: usize, pads: usize, rng: &mut R) -> Vec<u8> {
    let pads = pads.max(2);
    let mut out: Vec<u8> = Vec::with_capacity(len);
    for _ in 0..len {
        let next = match out.last() {
            Some(&last) => {
                let offset = rng.gen_range(1..pads) as u8;
                (last + offset) % pads as u8
            }
            None => rng.gen_range(0..pads) as u8,
        };
        out.push(next);
    }
    out
}

/// 往序列尾部追加一个（同样避免和上一个重复）。
pub fn extend_sequence<R: Rng + ?Sized>(sequence: &mut Vec<u8>, pads: usize, rng: &mut R) {
    let tail = build_sequence(1, pads, rng);
    let next = match sequence.last() {
        Some(&last) => {
            let offset = rng.gen_range(1..pads.max(2)) as u8;
            (last + offset) % pads as u8
        }
        None => tail[0],
    };
    sequence.push(next);
}
