/*
 * Copyright (C) 2020 Zixiao Han
 */

use crate::{
    def,
    state::State,
    util::{get_lowest_index, get_highest_index}
};

pub static MATE_VAL: i32 = 20000;
pub static TERM_VAL: i32 = 10000;
pub static LOSING_EXCHANGE_VAL: i32 = -250;

pub static DELTA_MARGIN: i32 = 190;
pub static DELTA_MAX_MARGIN: i32 = 1190;

pub static FUTILITY_MARGIN: i32 = 515;

static Q_VAL: i32 = 1000;
static R_VAL: i32 = 525;
static B_VAL: i32 = 350;
static N_VAL: i32 = 345;
static P_VAL: i32 = 100;

static KING_PROTECTED_BASE_VAL: i32 = 10;
static KING_EXPOSED_BASE_PEN: i32 = -30;
static KING_SEMI_EXPOSED_BASE_PEN: i32 = -15;
static KING_MIDGAME_SQR_VAL: i32 = 50;
static KING_ENDGAME_SQR_VAL: i32 = 30;
static KING_ENDGAME_AVOID_SQR_PEN: i32 = -20;

static PASS_PAWN_VAL: i32 = 20;
static DUP_PAWN_PEN: i32 = -10;
static ISOLATE_PAWN_PEN: i32 = -10;
static OPEN_ISOLATE_PAWN_PEN: i32 = -20;

static ROOK_SEMI_OPEN_LINE_VAL: i32 = 15;
static ROOK_OPEN_LINE_VAL: i32 = 20;

static QUEEN_OPEN_LINE_VAL: i32 = 10;

static GUARDED_PIECE_VAL: i32 = 10;

static CENTER_CONTROL_VAL: i32 = 15;
static THREAT_VAL: i32 = 15;
static INVASION_VAL: i32 = 5;
static TRAPPED_PEN: i32 = -10;

static ROOK_MIDGAME_MOB_BASE_VAL: i32 = 2;
static BISHOP_MIDGAME_MOB_BASE_VAL: i32 = 2;
static KNIGHT_MIDGAME_MOB_BASE_VAL: i32 = 2;

static ROOK_ENDGAME_MOB_BASE_VAL: i32 = 5;
static BISHOP_ENDGMAE_MOB_BASE_VAL: i32 = 5;
static KNIGHT_ENDGAME_MOB_BASE_VAL: i32 = 5;

static TOTAL_PHASE: i32 = 128;
static Q_PHASE_WEIGHT: i32 = 32;
static R_PHASE_WEIGHT: i32 = 8;
static B_PHASE_WEIGHT: i32 = 4;
static N_PHASE_WEIGHT: i32 = 4;

static TEMPO_VAL: i32 = 10;

static CENTER_CONTROL_MASK: u64 = 0b00000000_00000000_00000000_00011000_00011000_00000000_00000000_00000000;

static ALL_TRAP_MASK: u64 = 0b00000000_10000001_10000001_10000001_10000001_10000001_10000001_00000000;
static N_TRAP_MASK: u64 = 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
static B_TRAP_MASK: u64 = 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
static R_TRAP_MASK: u64 = 0b00000000_00000000_00000000_11111111_11111111_00000000_00000000_00000000;

static W_INVASION_MASK: u64 = 0b01111110_01111110_01111110_00111100_00000000_00000000_00000000_00000000;
static B_INVASION_MASK: u64 = 0b00000000_00000000_00000000_00000000_00111100_01111110_01111110_01111110;

static WP_THREAT_MASK: u64 = 0b00000000_01111110_00011000_00000000_00000000_00000000_00000000_00000000;
static BP_THREAT_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00011000_01111110_00000000;

static WR_THREAT_MASK: u64 = 0b00000000_01111110_00000000_00000000_00000000_00000000_00000000_00000000;
static BR_THREAT_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_01111110_00000000;

static WK_MIDGAME_SAFE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11000011_11000111;
static BK_MIDGAME_SAFE_MASK: u64 = 0b11000111_11000011_00000000_00000000_00000000_00000000_00000000_00000000;

static K_ENDGAME_PREF_MASK: u64 = 0b00000000_00000000_00111100_00111100_00111100_00111100_00000000_00000000;
static K_ENDGAME_AVOID_MASK: u64 = 0b11100111_11000011_10000001_00000000_00000000_10000001_11000011_11100111;

#[derive(PartialEq, Debug)]
pub struct FeatureMap {
    pawn_count: i32,
    queen_count: i32,
    rook_count: i32,
    bishop_count: i32,
    knight_count: i32,

    dup_pawn_count: i32,
    isolate_pawn_count: i32,
    open_isolate_pawn_count: i32,
    passed_pawn_count: i32,

    knight_mobility: i32,
    bishop_mobility: i32,
    rook_mobility: i32,

    semi_open_rook_count: i32,
    open_rook_count: i32,
    open_queen_count: i32,

    center_count: i32,
    invasion_count: i32,
    threat_count: i32,
    trapped_count: i32,

    guarded_piece_count: i32,

    king_expose_count: i32,
    king_semi_expose_count: i32,
    king_protector_count: i32,
    king_midgame_safe_sqr_count: i32,
    king_endgame_pref_sqr_count: i32,
    king_endgame_avoid_sqr_count: i32,
}

impl FeatureMap {
    pub fn empty() -> Self {
        FeatureMap {
            pawn_count: 0,
            queen_count: 0,
            rook_count: 0,
            bishop_count: 0,
            knight_count: 0,

            dup_pawn_count: 0,
            isolate_pawn_count: 0,
            open_isolate_pawn_count: 0,
            passed_pawn_count: 0,

            knight_mobility: 0,
            bishop_mobility: 0,
            rook_mobility: 0,

            semi_open_rook_count: 0,
            open_rook_count: 0,
            open_queen_count: 0,

            center_count: 0,
            invasion_count: 0,
            threat_count: 0,
            trapped_count: 0,

            guarded_piece_count: 0,

            king_expose_count: 0,
            king_semi_expose_count: 0,
            king_protector_count: 0,
            king_midgame_safe_sqr_count: 0,
            king_endgame_pref_sqr_count: 0,
            king_endgame_avoid_sqr_count: 0,
        }
    }
}

pub fn val_of(piece: u8) -> i32 {
    match piece {
        0 => 0,
        def::WK => MATE_VAL,
        def::WQ => Q_VAL,
        def::WR => R_VAL,
        def::WB => B_VAL,
        def::WN => N_VAL,
        def::WP => P_VAL,

        def::BK => MATE_VAL,
        def::BQ => Q_VAL,
        def::BR => R_VAL,
        def::BB => B_VAL,
        def::BN => N_VAL,
        def::BP => P_VAL,

        _ => 0,
    }
}

pub fn is_term_val(val: i32) -> bool {
    val < -TERM_VAL || val > TERM_VAL
}

pub fn eval_materials(state: &State) -> i32 {
    let bitboard = state.bitboard;

    let score_sign = if state.player == def::PLAYER_W {
        1
    } else {
        -1
    };

    (bitboard.w_queen.count_ones() as i32 * Q_VAL
    + bitboard.w_rook.count_ones() as i32 * R_VAL
    + bitboard.w_bishop.count_ones() as i32 * B_VAL
    + bitboard.w_knight.count_ones() as i32 * N_VAL
    + bitboard.w_pawn.count_ones() as i32 * P_VAL
    - bitboard.b_queen.count_ones() as i32 * Q_VAL
    - bitboard.b_rook.count_ones() as i32 * R_VAL
    - bitboard.b_bishop.count_ones() as i32 * B_VAL
    - bitboard.b_knight.count_ones() as i32 * N_VAL
    - bitboard.b_pawn.count_ones() as i32 * P_VAL) * score_sign
}

pub fn eval_state(state: &State, material_score: i32) -> i32 {
    let bitboard = state.bitboard;
    if bitboard.w_pawn | bitboard.b_pawn | bitboard.w_rook | bitboard.b_rook | bitboard.w_queen | bitboard.b_queen == 0 {
        if ((bitboard.w_bishop | bitboard.w_knight).count_ones() as i32 - (bitboard.b_bishop | bitboard.b_knight).count_ones() as i32).abs() < 2 {
            return 0
        }
    }

    let score_sign = if state.player == def::PLAYER_W {
        1
    } else {
        -1
    };

    let (w_features_map, b_features_map) = extract_features(state);

    let midgame_extra_score =
        w_features_map.isolate_pawn_count * ISOLATE_PAWN_PEN
        + w_features_map.open_isolate_pawn_count * OPEN_ISOLATE_PAWN_PEN
        + w_features_map.semi_open_rook_count * ROOK_SEMI_OPEN_LINE_VAL
        + w_features_map.open_rook_count * ROOK_OPEN_LINE_VAL
        + w_features_map.open_queen_count * QUEEN_OPEN_LINE_VAL
        + w_features_map.rook_mobility * ROOK_MIDGAME_MOB_BASE_VAL
        + w_features_map.bishop_mobility * BISHOP_MIDGAME_MOB_BASE_VAL
        + w_features_map.knight_mobility * KNIGHT_MIDGAME_MOB_BASE_VAL
        + w_features_map.king_protector_count * KING_PROTECTED_BASE_VAL
        + w_features_map.king_midgame_safe_sqr_count * KING_MIDGAME_SQR_VAL
        + w_features_map.king_expose_count * KING_EXPOSED_BASE_PEN
        + w_features_map.king_semi_expose_count * KING_SEMI_EXPOSED_BASE_PEN
        + w_features_map.center_count * CENTER_CONTROL_VAL
        + w_features_map.invasion_count * INVASION_VAL
        + w_features_map.threat_count * THREAT_VAL
        + w_features_map.trapped_count * TRAPPED_PEN
        + w_features_map.guarded_piece_count * GUARDED_PIECE_VAL
        - b_features_map.isolate_pawn_count * ISOLATE_PAWN_PEN
        - b_features_map.open_isolate_pawn_count * OPEN_ISOLATE_PAWN_PEN
        - b_features_map.semi_open_rook_count * ROOK_SEMI_OPEN_LINE_VAL
        - b_features_map.open_rook_count * ROOK_OPEN_LINE_VAL
        - b_features_map.open_queen_count * QUEEN_OPEN_LINE_VAL
        - b_features_map.rook_mobility * ROOK_MIDGAME_MOB_BASE_VAL
        - b_features_map.bishop_mobility * BISHOP_MIDGAME_MOB_BASE_VAL
        - b_features_map.knight_mobility * KNIGHT_MIDGAME_MOB_BASE_VAL
        - b_features_map.king_protector_count * KING_PROTECTED_BASE_VAL
        - b_features_map.king_midgame_safe_sqr_count * KING_MIDGAME_SQR_VAL
        - b_features_map.king_expose_count * KING_EXPOSED_BASE_PEN
        - b_features_map.king_semi_expose_count * KING_SEMI_EXPOSED_BASE_PEN
        - b_features_map.center_count * CENTER_CONTROL_VAL
        - b_features_map.invasion_count * INVASION_VAL
        - b_features_map.threat_count * THREAT_VAL
        - b_features_map.trapped_count * TRAPPED_PEN
        - b_features_map.guarded_piece_count - GUARDED_PIECE_VAL;

    let endgame_extra_score =
        w_features_map.passed_pawn_count * PASS_PAWN_VAL
        + w_features_map.dup_pawn_count * DUP_PAWN_PEN
        + w_features_map.king_endgame_pref_sqr_count * KING_ENDGAME_SQR_VAL
        + w_features_map.king_endgame_avoid_sqr_count * KING_ENDGAME_AVOID_SQR_PEN
        + w_features_map.rook_mobility * ROOK_ENDGAME_MOB_BASE_VAL
        + w_features_map.bishop_mobility * BISHOP_ENDGMAE_MOB_BASE_VAL
        + w_features_map.knight_mobility * KNIGHT_ENDGAME_MOB_BASE_VAL
        - b_features_map.passed_pawn_count * PASS_PAWN_VAL
        - b_features_map.dup_pawn_count * DUP_PAWN_PEN
        - b_features_map.king_endgame_pref_sqr_count * KING_ENDGAME_SQR_VAL
        - b_features_map.king_endgame_avoid_sqr_count * KING_ENDGAME_AVOID_SQR_PEN
        - b_features_map.rook_mobility * ROOK_ENDGAME_MOB_BASE_VAL
        - b_features_map.bishop_mobility * BISHOP_ENDGMAE_MOB_BASE_VAL
        - b_features_map.knight_mobility * KNIGHT_ENDGAME_MOB_BASE_VAL;

    let phase = w_features_map.queen_count * Q_PHASE_WEIGHT
    + w_features_map.rook_count * R_PHASE_WEIGHT
    + w_features_map.bishop_count * B_PHASE_WEIGHT
    + w_features_map.knight_count * N_PHASE_WEIGHT
    + b_features_map.queen_count * Q_PHASE_WEIGHT
    + b_features_map.rook_count * R_PHASE_WEIGHT
    + b_features_map.bishop_count * B_PHASE_WEIGHT
    + b_features_map.knight_count * N_PHASE_WEIGHT;

    let extra_score = (midgame_extra_score * phase + endgame_extra_score * (TOTAL_PHASE - phase)) / TOTAL_PHASE;

    material_score + extra_score * score_sign + TEMPO_VAL
}

pub fn extract_features(state: &State) -> (FeatureMap, FeatureMap) {
    let squares = state.squares;
    let index_masks = state.bitmask.index_masks;
    let file_masks = state.bitmask.file_masks;
    let bitboard = state.bitboard;
    let bitmask = state.bitmask;

    let mut w_feature_map = FeatureMap::empty();
    let mut b_feature_map = FeatureMap::empty();

    let mut wp_attack_mask = 0;
    let mut wn_attack_mask = 0;
    let mut wb_attack_mask = 0;
    let mut wr_attack_mask = 0;

    let mut bp_attack_mask = 0;
    let mut bn_attack_mask = 0;
    let mut bb_attack_mask = 0;
    let mut br_attack_mask = 0;

    let occupy_mask = bitboard.w_all | bitboard.b_all;
    let start_index = occupy_mask.trailing_zeros() as usize;
    let end_index = def::BOARD_SIZE - occupy_mask.leading_zeros() as usize;

    for index in start_index..end_index {
        let moving_piece = squares[index];

        if moving_piece == 0 {
            continue
        }

        let index_mask = index_masks[index];

        match moving_piece {
            def::WP => {
                wp_attack_mask |= bitmask.wp_attack_masks[index];

                let file_mask = file_masks[index];
                let rank = def::get_rank(def::PLAYER_W, index) as i32;

                if bitmask.wp_forward_masks[index] & bitboard.b_pawn == 0 {
                    w_feature_map.passed_pawn_count += rank;

                    if bitmask.wp_behind_masks[index] & bitboard.w_pawn != 0 {
                        w_feature_map.passed_pawn_count += rank / 2;
                    }
                }

                if bitmask.wp_behind_masks[index] & bitboard.w_pawn == 0 {
                    if file_mask & bitboard.b_pawn == 0 {
                        w_feature_map.open_isolate_pawn_count += 1;
                    } else {
                        w_feature_map.isolate_pawn_count += 1;
                    }
                }

                if (file_mask & bitboard.w_pawn).count_ones() > 1 {
                    w_feature_map.dup_pawn_count += 1;
                }
            },
            def::BP => {
                bp_attack_mask |= bitmask.bp_attack_masks[index];

                let file_mask = file_masks[index];
                let rank = def::get_rank(def::PLAYER_B, index) as i32;

                if bitmask.bp_forward_masks[index] & bitboard.w_pawn == 0 {
                    b_feature_map.passed_pawn_count += rank;

                    if bitmask.bp_behind_masks[index] & bitboard.b_pawn != 0 {
                        b_feature_map.passed_pawn_count += rank / 2;
                    }
                }

                if bitmask.bp_behind_masks[index] & bitboard.b_pawn == 0 {
                    if file_mask & bitboard.w_pawn == 0 {
                        b_feature_map.open_isolate_pawn_count += 1;
                    } else {
                        b_feature_map.isolate_pawn_count += 1;
                    }
                }

                if (file_mask & bitboard.b_pawn).count_ones() > 1 {
                    b_feature_map.dup_pawn_count += 1;
                }
            },

            def::WN => {
                wn_attack_mask |= bitmask.n_attack_masks[index];
            },
            def::BN => {
                bn_attack_mask |= bitmask.n_attack_masks[index];
            },

            def::WB => {
                let mut mov_mask = 0;

                let up_left_attack_mask = bitmask.up_left_attack_masks[index];
                mov_mask ^= up_left_attack_mask;
                if up_left_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
                }

                let up_right_attack_mask = bitmask.up_right_attack_masks[index];
                mov_mask ^= up_right_attack_mask;
                if up_right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
                }

                let down_left_attack_mask = bitmask.down_left_attack_masks[index];
                mov_mask ^= down_left_attack_mask;
                if down_left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
                }

                let down_right_attack_mask = bitmask.down_right_attack_masks[index];
                mov_mask ^= down_right_attack_mask;
                if down_right_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
                }

                wb_attack_mask |= mov_mask;
            },
            def::BB => {
                let mut mov_mask = 0;

                let up_left_attack_mask = bitmask.up_left_attack_masks[index];
                mov_mask ^= up_left_attack_mask;
                if up_left_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
                }

                let up_right_attack_mask = bitmask.up_right_attack_masks[index];
                mov_mask ^= up_right_attack_mask;
                if up_right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
                }

                let down_left_attack_mask = bitmask.down_left_attack_masks[index];
                mov_mask ^= down_left_attack_mask;
                if down_left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
                }

                let down_right_attack_mask = bitmask.down_right_attack_masks[index];
                mov_mask ^= down_right_attack_mask;
                if down_right_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
                }

                bb_attack_mask |= mov_mask;
            },

            def::WR => {
                let mut mov_mask = 0;

                let up_attack_mask = bitmask.up_attack_masks[index];
                mov_mask ^= up_attack_mask;
                if up_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
                }

                let right_attack_mask = bitmask.right_attack_masks[index];
                mov_mask ^= right_attack_mask;
                if right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
                }

                let down_attack_mask = bitmask.down_attack_masks[index];
                mov_mask ^= down_attack_mask;
                if down_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_attack_masks[highest_blocker_index];
                }

                let left_attack_mask = bitmask.left_attack_masks[index];
                mov_mask ^= left_attack_mask;
                if left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.left_attack_masks[highest_blocker_index];
                }

                wr_attack_mask |= mov_mask;

                let file_mask = file_masks[index];
                if file_mask & (bitboard.w_all ^ bitboard.w_rook) == 0 {
                    if file_mask & bitboard.b_all == 0 {
                        w_feature_map.open_rook_count += 1;
                    } else {
                        w_feature_map.semi_open_rook_count += 1;
                    }
                }
            },
            def::BR => {
                let mut mov_mask = 0;

                let up_attack_mask = bitmask.up_attack_masks[index];
                mov_mask ^= up_attack_mask;
                if up_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
                }

                let right_attack_mask = bitmask.right_attack_masks[index];
                mov_mask ^= right_attack_mask;
                if right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
                }

                let down_attack_mask = bitmask.down_attack_masks[index];
                mov_mask ^= down_attack_mask;
                if down_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_attack_masks[highest_blocker_index];
                }

                let left_attack_mask = bitmask.left_attack_masks[index];
                mov_mask ^= left_attack_mask;
                if left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.left_attack_masks[highest_blocker_index];
                }

                br_attack_mask |= mov_mask;

                let file_mask = file_masks[index];
                if file_mask & (bitboard.b_all ^ bitboard.b_rook) == 0 {
                    if file_mask & bitboard.w_all == 0 {
                        b_feature_map.open_rook_count += 1;
                    } else {
                        b_feature_map.semi_open_rook_count += 1;
                    }
                }
            },

            def::WQ => {
                let file_mask = file_masks[index];
                if file_mask & ((bitboard.w_all | bitboard.b_all) ^ index_mask) == 0 {
                    w_feature_map.open_queen_count += 1;
                }
            },
            def::BQ => {
                let file_mask = file_masks[index];
                if file_mask & ((bitboard.w_all | bitboard.b_all) ^ index_mask) == 0 {
                    b_feature_map.open_queen_count += 1;
                }
            },

            def::WK => {
                let file_mask = file_masks[index];

                if index_mask & WK_MIDGAME_SAFE_MASK != 0 {
                    w_feature_map.king_midgame_safe_sqr_count = 1;
                }

                if index_mask & K_ENDGAME_PREF_MASK != 0 {
                    w_feature_map.king_endgame_pref_sqr_count = 1;
                } else if index_mask & K_ENDGAME_AVOID_MASK != 0 {
                    w_feature_map.king_endgame_avoid_sqr_count = 1;
                }

                if file_mask & bitboard.w_pawn == 0 {
                    w_feature_map.king_expose_count += 1;
                }

                if file_mask & bitboard.b_pawn == 0 {
                    w_feature_map.king_expose_count += 1;
                }

                if index % def::DIM_SIZE != 7 {
                    let right_file_mask = file_masks[index + 1];

                    if right_file_mask & bitboard.b_pawn == 0 {
                        w_feature_map.king_semi_expose_count += 1;
                    }
                }

                if index % def::DIM_SIZE != 0 {
                    let left_file_mask = file_masks[index - 1];

                    if left_file_mask & bitboard.b_pawn == 0 {
                        w_feature_map.king_semi_expose_count += 1;
                    }
                }
            },
            def::BK => {
                let file_mask = file_masks[index];

                if index_mask & BK_MIDGAME_SAFE_MASK != 0 {
                    b_feature_map.king_midgame_safe_sqr_count = 1;
                }

                if index_mask & K_ENDGAME_PREF_MASK != 0 {
                    b_feature_map.king_endgame_pref_sqr_count = 1;
                } else if index_mask & K_ENDGAME_AVOID_MASK != 0 {
                    b_feature_map.king_endgame_avoid_sqr_count = 1;
                }

                if file_mask & bitboard.b_pawn == 0 {
                    b_feature_map.king_expose_count += 1;
                }

                if file_mask & bitboard.w_pawn == 0 {
                    b_feature_map.king_expose_count += 1;
                }

                if index % def::DIM_SIZE != 7 {
                    let right_file_mask = file_masks[index + 1];

                    if right_file_mask & bitboard.w_pawn == 0 {
                        b_feature_map.king_semi_expose_count += 1;
                    }
                }

                if index % def::DIM_SIZE != 0 {
                    let left_file_mask = file_masks[index - 1];

                    if left_file_mask & bitboard.w_pawn == 0 {
                        b_feature_map.king_semi_expose_count += 1;
                    }
                }
            },
            _ => {},
        }
    }

    w_feature_map.pawn_count = bitboard.w_pawn.count_ones() as i32;
    w_feature_map.knight_count = bitboard.w_knight.count_ones() as i32;
    w_feature_map.bishop_count = bitboard.w_bishop.count_ones() as i32;
    w_feature_map.rook_count = bitboard.w_rook.count_ones() as i32;
    w_feature_map.queen_count = bitboard.w_queen.count_ones() as i32;

    b_feature_map.pawn_count = bitboard.b_pawn.count_ones() as i32;
    b_feature_map.knight_count = bitboard.b_knight.count_ones() as i32;
    b_feature_map.bishop_count = bitboard.b_bishop.count_ones() as i32;
    b_feature_map.rook_count = bitboard.b_rook.count_ones() as i32;
    b_feature_map.queen_count = bitboard.b_queen.count_ones() as i32;

    let w_light_pieces_mask = bitboard.w_pawn | bitboard.w_knight | bitboard.w_bishop;
    let b_light_pieces_mask = bitboard.b_pawn | bitboard.b_knight | bitboard.b_bishop;

    let wk_protector_mask = bitmask.wk_protect_masks[state.wk_index] & (bitboard.w_pawn | bitboard.w_knight | bitboard.w_bishop);
    w_feature_map.king_protector_count = wk_protector_mask.count_ones() as i32;

    let wk_pawn_attack_defend_mask = wk_protector_mask & (bp_attack_mask & !wp_attack_mask);
    let wk_other_attack_defend_mask = wk_protector_mask & ((bn_attack_mask | bb_attack_mask | br_attack_mask) & !(wn_attack_mask | wb_attack_mask | wr_attack_mask));

    w_feature_map.king_protector_count -= (wk_pawn_attack_defend_mask.count_ones() + wk_other_attack_defend_mask.count_ones()) as i32;

    let bk_protector_mask = bitmask.bk_protect_masks[state.bk_index] & & (bitboard.b_pawn | bitboard.b_knight | bitboard.b_bishop);
    b_feature_map.king_protector_count = bk_protector_mask.count_ones() as i32;

    let bk_pawn_attack_defend_mask = bk_protector_mask & (wp_attack_mask & !bp_attack_mask);
    let bk_other_attack_defend_mask = bk_protector_mask & ((wn_attack_mask | wb_attack_mask | wr_attack_mask) & !(bn_attack_mask | bb_attack_mask | br_attack_mask));
    b_feature_map.king_protector_count -= (bk_pawn_attack_defend_mask.count_ones() + bk_other_attack_defend_mask.count_ones()) as i32;

    w_feature_map.center_count = (w_light_pieces_mask & CENTER_CONTROL_MASK).count_ones() as i32;
    b_feature_map.center_count = (b_light_pieces_mask & CENTER_CONTROL_MASK).count_ones() as i32;

    w_feature_map.invasion_count = (w_light_pieces_mask & W_INVASION_MASK).count_ones() as i32;
    b_feature_map.invasion_count = (b_light_pieces_mask & B_INVASION_MASK).count_ones() as i32;

    w_feature_map.threat_count = ((bitboard.w_rook | bitboard.w_queen) & WR_THREAT_MASK).count_ones() as i32;
    w_feature_map.threat_count += (bitboard.w_pawn & WP_THREAT_MASK).count_ones() as i32;

    b_feature_map.threat_count = ((bitboard.b_rook | bitboard.b_queen) & BR_THREAT_MASK).count_ones() as i32;
    b_feature_map.threat_count += (bitboard.b_pawn & BP_THREAT_MASK).count_ones() as i32;

    w_feature_map.knight_mobility = (wn_attack_mask & !bitboard.w_all & !bp_attack_mask).count_ones() as i32;
    w_feature_map.bishop_mobility = (wb_attack_mask & !bitboard.w_all & !bp_attack_mask).count_ones() as i32;
    w_feature_map.rook_mobility = (wr_attack_mask & !bitboard.w_all & !(bp_attack_mask | bn_attack_mask | bb_attack_mask)).count_ones() as i32;

    b_feature_map.knight_mobility = (bn_attack_mask & !bitboard.b_all & !wp_attack_mask).count_ones() as i32;
    b_feature_map.bishop_mobility = (bb_attack_mask & !bitboard.b_all & !wp_attack_mask).count_ones() as i32;
    b_feature_map.rook_mobility = (br_attack_mask & !bitboard.b_all & !(wp_attack_mask | wn_attack_mask | wb_attack_mask)).count_ones() as i32;

    let w_trapped_piece_mask = ((bitboard.w_knight | bitboard.w_bishop | bitboard.w_rook | bitboard.w_queen) & ALL_TRAP_MASK)
        | ((bitboard.w_knight & N_TRAP_MASK) | (bitboard.w_bishop & B_TRAP_MASK))
        | (bitboard.w_rook & R_TRAP_MASK);

    let b_trapped_piece_mask = ((bitboard.b_knight | bitboard.b_bishop | bitboard.b_rook | bitboard.b_queen) & ALL_TRAP_MASK)
        | ((bitboard.b_knight & N_TRAP_MASK) | (bitboard.b_bishop & B_TRAP_MASK))
        | (bitboard.b_rook & R_TRAP_MASK);

    w_feature_map.trapped_count = w_trapped_piece_mask.count_ones() as i32;
    b_feature_map.trapped_count = b_trapped_piece_mask.count_ones() as i32;

    w_feature_map.guarded_piece_count += ((bitboard.w_knight | bitboard.w_bishop) & (wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask)).count_ones() as i32;
    w_feature_map.guarded_piece_count -= ((bitboard.w_knight | bitboard.w_bishop | bitboard.w_rook | bitboard.w_queen) & (bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask)).count_ones() as i32;

    b_feature_map.guarded_piece_count += ((bitboard.b_knight | bitboard.b_bishop) & (bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask)).count_ones() as i32;
    b_feature_map.guarded_piece_count -= ((bitboard.b_knight | bitboard.b_bishop | bitboard.b_rook | bitboard.b_queen) & (wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask)).count_ones() as i32;

    (w_feature_map, b_feature_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bitboard::BitMask,
        state::State,
        prng::XorshiftPrng,
   };

    #[test]
    fn test_extract_features_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1kr2r2/pp2nppp/1bn2q2/3pp3/3P4/1BN1P3/PPP1NP1P/R2Q1RK1 b Q - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(FeatureMap {
            pawn_count: 7,
            queen_count: 1,
            rook_count: 2,
            bishop_count: 1,
            knight_count: 2,

            dup_pawn_count: 0,
            isolate_pawn_count: 2,
            open_isolate_pawn_count: 0,
            passed_pawn_count: 0,

            knight_mobility: 6,
            bishop_mobility: 2,
            rook_mobility: 3,

            semi_open_rook_count: 0,
            open_rook_count: 0,
            open_queen_count: 0,

            center_count: 1,
            invasion_count: 0,
            threat_count: 0,
            trapped_count: 0,

            guarded_piece_count: 3,

            king_expose_count: 1,
            king_semi_expose_count: 0,
            king_protector_count: 2,
            king_midgame_safe_sqr_count: 1,
            king_endgame_pref_sqr_count: 0,
            king_endgame_avoid_sqr_count: 1,
        }, w_features);

        assert_eq!(FeatureMap {
            pawn_count: 7,
            queen_count: 1,
            rook_count: 2,
            bishop_count: 1,
            knight_count: 2,

            dup_pawn_count: 0,
            isolate_pawn_count: 0,
            open_isolate_pawn_count: 0,
            passed_pawn_count: 0,

            knight_mobility: 6,
            bishop_mobility: 3,
            rook_mobility: 5,

            semi_open_rook_count: 0,
            open_rook_count: 0,
            open_queen_count: 0,

            center_count: 2,
            invasion_count: 0,
            threat_count: 0,
            trapped_count: 0,

            guarded_piece_count: 3,

            king_expose_count: 0,
            king_semi_expose_count: 0,
            king_protector_count: 4,
            king_midgame_safe_sqr_count: 1,
            king_endgame_pref_sqr_count: 0,
            king_endgame_avoid_sqr_count: 1,
        }, b_features);
    }

    #[test]
    fn test_extract_features_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1kr2r2/1p4pp/1p1P1qn1/p2pp3/3P4/RB2P3/P1P1NP1P/3Q1RK1 b - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(FeatureMap {
            pawn_count: 7,
            queen_count: 1,
            rook_count: 2,
            bishop_count: 1,
            knight_count: 1,

            dup_pawn_count: 2,
            isolate_pawn_count: 2,
            open_isolate_pawn_count: 2,
            passed_pawn_count: 7,

            knight_mobility: 3,
            bishop_mobility: 2,
            rook_mobility: 2,

            semi_open_rook_count: 0,
            open_rook_count: 0,
            open_queen_count: 0,

            center_count: 1,
            invasion_count: 1,
            threat_count: 1,
            trapped_count: 1,

            guarded_piece_count: 1,

            king_expose_count: 1,
            king_semi_expose_count: 1,
            king_protector_count: 2,
            king_midgame_safe_sqr_count: 1,
            king_endgame_pref_sqr_count: 0,
            king_endgame_avoid_sqr_count: 1,
        }, w_features);

        assert_eq!(FeatureMap {
            pawn_count: 7,
            queen_count: 1,
            rook_count: 2,
            bishop_count: 0,
            knight_count: 1,

            dup_pawn_count: 2,
            isolate_pawn_count: 0,
            open_isolate_pawn_count: 2,
            passed_pawn_count: 0,

            knight_mobility: 2,
            bishop_mobility: 0,
            rook_mobility: 6,

            semi_open_rook_count: 1,
            open_rook_count: 0,
            open_queen_count: 0,

            center_count: 2,
            invasion_count: 0,
            threat_count: 0,
            trapped_count: 0,

            guarded_piece_count: 1,

            king_expose_count: 1,
            king_semi_expose_count: 0,
            king_protector_count: 2,
            king_midgame_safe_sqr_count: 1,
            king_endgame_pref_sqr_count: 0,
            king_endgame_avoid_sqr_count: 1,
        }, b_features);
    }

    #[test]
    fn test_extract_features_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1kr2r2/pp2qpp1/1bn2n2/1p1p4/1P1P4/1BN3N1/PPP2P1P/R2Q1RK1 b Q - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(FeatureMap {
            pawn_count: 7,
            queen_count: 1,
            rook_count: 2,
            bishop_count: 1,
            knight_count: 2,

            dup_pawn_count: 2,
            isolate_pawn_count: 1,
            open_isolate_pawn_count: 1,
            passed_pawn_count: 0,

            knight_mobility: 7,
            bishop_mobility: 1,
            rook_mobility: 3,

            semi_open_rook_count: 0,
            open_rook_count: 0,
            open_queen_count: 0,

            center_count: 1,
            invasion_count: 0,
            threat_count: 0,
            trapped_count: 0,

            guarded_piece_count: 3,

            king_expose_count: 1,
            king_semi_expose_count: 1,
            king_protector_count: 3,
            king_midgame_safe_sqr_count: 1,
            king_endgame_pref_sqr_count: 0,
            king_endgame_avoid_sqr_count: 1,
        }, w_features);

        assert_eq!(FeatureMap {
            pawn_count: 6,
            queen_count: 1,
            rook_count: 2,
            bishop_count: 1,
            knight_count: 2,

            dup_pawn_count: 2,
            isolate_pawn_count: 1,
            open_isolate_pawn_count: 0,
            passed_pawn_count: 0,

            knight_mobility: 10,
            bishop_mobility: 3,
            rook_mobility: 5,

            semi_open_rook_count: 0,
            open_rook_count: 0,
            open_queen_count: 1,

            center_count: 1,
            invasion_count: 0,
            threat_count: 0,
            trapped_count: 0,

            guarded_piece_count: 3,

            king_expose_count: 0,
            king_semi_expose_count: 0,
            king_protector_count: 4,
            king_midgame_safe_sqr_count: 1,
            king_endgame_pref_sqr_count: 0,
            king_endgame_avoid_sqr_count: 1,
        }, b_features);
    }

    #[test]
    fn test_extract_features_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("n2qk2b/pppppppp/8/r7/2R1Q3/8/PPPPPPPP/N3K2B w - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(3, w_features.trapped_count);
        assert_eq!(3, b_features.trapped_count);
    }

    #[test]
    fn test_extract_features_5() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/p2R2p1/1p3p1p/2p5/8/1P6/Pr1r1PPP/8 w - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.threat_count);
        assert_eq!(2, b_features.threat_count);
    }

    #[test]
    fn test_extract_features_6() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1kr2r2/pp2qpp1/1bn5/1p1p2n1/1P1P4/PBNP2N1/1P3P1P/R2Q1RK1 b Q - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(2, w_features.guarded_piece_count);
        assert_eq!(2, b_features.guarded_piece_count);
    }

    #[test]
    fn test_extract_features_7() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1kr1br2/1p1n1ppp/1p1P1b2/p2N3n/3P4/RB2P1N1/P1P2P1P/3Q1RK1 b - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(2, w_features.guarded_piece_count);
        assert_eq!(1, b_features.guarded_piece_count);
    }

    #[test]
    fn test_extract_features_8() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1q4kn/3r1p1p/1pbN1Pp1/r1ppP1P1/P4R2/2B1P3/2Q4P/3R2K1 b - - 2 29", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(5, w_features.knight_mobility);
        assert_eq!(0, b_features.knight_mobility);
    }

    #[test]
    fn test_extract_features_9() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnb1kbnr/ppP2Rpp/8/8/8/4pP2/PrP1P1qP/RNBQKBNR w KQkq - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(2, w_features.threat_count);
        assert_eq!(3, b_features.threat_count);
    }

    #[test]
    fn test_extract_features_10() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1kr3r1/pp3p1p/P1pn4/2Bpb3/4p2q/3PP3/PPP1NPPP/R2Q1RK1 b - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.king_protector_count);
        assert_eq!(1, b_features.king_protector_count);
    }
}
