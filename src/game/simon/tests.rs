use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::game::model::AgeTier;

use super::constants::*;
use super::palette;
use super::resources::{SimonStage, SimonPhase, build_sequence, extend_sequence};
use super::setup::PAD_OFFSETS;
use super::systems::pad_of;

const PADS: usize = 4;

fn stage_with(sequence: Vec<u8>, reverse: bool) -> SimonStage {
    SimonStage {
        sequence,
        phase: SimonPhase::Input,
        clock: 1.0,
        play_index: 0,
        input_index: 0,
        lit: None,
        lit_clock: 0.0,
        round: 1,
        rounds_total: 3,
        reverse,
        step_time: 0.5,
        input_time: 3.0,
        retry: false,
        failed: false,
        message: String::new(),
    }
}

/// 四个音板必须正好对上四个方向键，这是这款游戏"零学习成本"的全部依据。
#[test]
fn every_direction_maps_to_exactly_one_pad() {
    let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut pads: Vec<usize> = dirs.iter().map(|&d| pad_of(d).expect("方向必须有音板")).collect();
    pads.sort_unstable();
    assert_eq!(pads, vec![0, 1, 2, 3]);
    assert_eq!(pad_of((1, 1)), None, "斜向不该映射到音板");
    assert_eq!(PAD_OFFSETS.len(), PADS);
    assert_eq!(palette::PAD_DIM.len(), PADS);
    assert_eq!(palette::PAD_LIT.len(), PADS);
}

/// 亮起的颜色必须明显比暗色亮，12 像素的小方块只靠色相分不出"亮没亮"。
#[test]
fn lit_pads_are_clearly_brighter_than_dim_ones() {
    for pad in 0..PADS {
        let dim = palette::PAD_DIM[pad].to_linear();
        let lit = palette::PAD_LIT[pad].to_linear();
        let dim_lum = dim.red * 0.2126 + dim.green * 0.7152 + dim.blue * 0.0722;
        let lit_lum = lit.red * 0.2126 + lit.green * 0.7152 + lit.blue * 0.0722;
        assert!(lit_lum > dim_lum * 3.0, "第 {pad} 个音板亮暗差不够：{dim_lum} → {lit_lum}");
    }
}

#[test]
fn sequences_never_repeat_the_same_pad_twice_in_a_row() {
    let mut rng = StdRng::seed_from_u64(307);
    for len in 2..=20 {
        let sequence = build_sequence(len, PADS, &mut rng);
        assert_eq!(sequence.len(), len);
        assert!(sequence.iter().all(|&pad| (pad as usize) < PADS));
        for pair in sequence.windows(2) {
            assert_ne!(pair[0], pair[1], "连着两下同一个板听起来像一下：{sequence:?}");
        }
    }
}

#[test]
fn extending_keeps_the_no_repeat_rule() {
    let mut rng = StdRng::seed_from_u64(311);
    let mut sequence = build_sequence(3, PADS, &mut rng);
    for _ in 0..30 {
        extend_sequence(&mut sequence, PADS, &mut rng);
        for pair in sequence.windows(2) {
            assert_ne!(pair[0], pair[1], "追加后出现了连续重复：{sequence:?}");
        }
    }
    assert_eq!(sequence.len(), 33);
}

#[test]
fn forward_mode_expects_the_sequence_head_first() {
    let mut stage = stage_with(vec![2, 0, 3], false);
    for expected in [2u8, 0, 3] {
        assert_eq!(stage.expected(), Some(expected));
        stage.input_index += 1;
    }
    assert_eq!(stage.expected(), None, "按完就没有下一个了");
}

#[test]
fn reverse_mode_expects_the_sequence_tail_first() {
    let mut stage = stage_with(vec![2, 0, 3], true);
    for expected in [3u8, 0, 2] {
        assert_eq!(stage.expected(), Some(expected));
        stage.input_index += 1;
    }
    assert_eq!(stage.expected(), None);
}

#[test]
fn a_round_is_cleared_only_after_every_round_is_done() {
    let mut stage = stage_with(vec![1, 2], false);
    stage.rounds_total = 3;
    for round in 1..=3 {
        stage.round = round;
        assert!(!stage.is_cleared(), "第 {round} 轮不该算通关");
    }
    stage.round = 4;
    assert!(stage.is_cleared());
}

#[test]
fn junior_tier_is_slower_shorter_and_never_reversed() {
    for level in 1..=10u8 {
        assert!(start_len_for(level, AgeTier::Junior) <= JUNIOR_MAX_START_LEN);
        assert!(
            step_time_for(level, AgeTier::Junior) > step_time_for(level, AgeTier::Senior),
            "第 {level} 关低龄档播放没有变慢"
        );
        assert!(!reverse_for(level, AgeTier::Junior), "低龄档不该考倒序");
    }
    assert!(!reverse_for(7, AgeTier::Senior));
    assert!(reverse_for(8, AgeTier::Senior));
    assert!(input_time_for(AgeTier::Junior) > input_time_for(AgeTier::Senior));
}

/// 播放速度必须逐关变快，否则"第 10 关"只是序列更长而已。
#[test]
fn playback_gets_faster_every_level() {
    let times: Vec<f32> = (1..=10u8).map(|l| step_time_for(l, AgeTier::Senior)).collect();
    for pair in times.windows(2) {
        assert!(pair[1] <= pair[0], "播放速度出现回落：{times:?}");
    }
    assert!(times[9] < times[0] * 0.6, "最后一关没有明显更快：{times:?}");
}

#[test]
fn round_score_grows_with_sequence_length() {
    assert_eq!(round_score(3), 30);
    assert_eq!(round_score(10), 100);
    assert!(round_score(8) > round_score(5));
}

/// 答错重来时序列不能变长 —— 否则罚得太狠，一错就雪崩。
#[test]
fn a_retry_replays_the_same_sequence_while_a_win_extends_it() {
    let mut rng = StdRng::seed_from_u64(313);
    let mut stage = stage_with(vec![0, 2, 1], false);
    let before = stage.sequence.len();
    let original = stage.sequence.clone();

    stage.retry = true;
    stage.begin_next_round(PADS, &mut rng);
    assert_eq!(stage.sequence, original, "重来一轮必须原样重放");
    assert!(!stage.retry, "重放一次之后要把标记清掉");
    assert_eq!(stage.phase, SimonPhase::Ready);
    assert_eq!(stage.input_index, 0);
    assert_eq!(stage.play_index, 0);

    stage.begin_next_round(PADS, &mut rng);
    assert_eq!(stage.sequence.len(), before + 1, "答对一轮应该加长一个");
    assert_eq!(&stage.sequence[..before], &original[..], "加长不能动前面已有的序列");
}
