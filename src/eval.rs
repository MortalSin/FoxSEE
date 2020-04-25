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

pub static DELTA_MARGIN: i32 = 90;
pub static DELTA_MAX_MARGIN: i32 = 1090;

pub const FUTILITY_MARGIN_BASE: i32 = 230;
pub const MAX_POS_VAL: i32 = 90;

static Q_VAL: i32 = 1000;
static R_VAL: i32 = 525;
static B_VAL: i32 = 350;
static N_VAL: i32 = 340;
static P_VAL: i32 = 100;

static KING_EXPOSED_PEN: i32 = -50;
static KING_THREAT_BASE_PEN: i32 = -30;
static KING_PAWN_THREAT_BASE_PEN: i32 = -30;
static KING_LOST_CAS_RIGHTS_PEN: i32 = -50;

static PASS_PAWN_BASE_VAL: i32 = 30;
static PASS_PAWN_RANK_VAL: i32 = 20;
static UNSTOPPABLE_PASS_PAWN_VAL: i32 = 200;
static CONTROLLED_PASS_PAWN_VAL: i32 = 30;

static DUP_PAWN_PEN: i32 = -20;
static ISOLATE_PAWN_PEN: i32 = -20;
static BEHIND_PAWN_PEN: i32 = -10;

static ROOK_SEMI_OPEN_LINE_VAL: i32 = 20;
static ROOK_OPEN_LINE_VAL: i32 = 25;

static QUEEN_OPEN_LINE_VAL: i32 = 20;

static THREATENED_PIECE_PEN: i32 = -30;
static DEFENDED_PIECE_VAL: i32 = 15;
static DEFENDED_PAWN_VAL: i32 = 10;

static MIDGAME_MOB_BASE_VAL: i32 = 2;
static ENDGMAE_MOB_BASE_VAL: i32 = 2;

static ENDGAME_ROOK_EXTRA_VAL: i32 = 30;
static ENDGAME_QUEEN_EXTRA_VAL: i32 = 30;

static TRAPPED_Q_PEN: i32 = -90;
static TRAPPED_R_PEN: i32 = -80;
static TRAPPED_B_PEN: i32 = -60;
static TRAPPED_N_PEN: i32 = -60;

static BLOCKED_Q_PEN: i32 = -30;
static BLOCKED_R_PEN: i32 = -30;
static BLOCKED_B_PEN: i32 = -30;
static BLOCKED_N_PEN: i32 = -30;

static TOTAL_PHASE: i32 = 96;
static Q_PHASE_WEIGHT: i32 = 16;
static R_PHASE_WEIGHT: i32 = 8;
static B_PHASE_WEIGHT: i32 = 4;
static N_PHASE_WEIGHT: i32 = 4;

pub static ENDGAME_PHASE: i32 = 16;

static TEMPO_VAL: i32 = 10;

static BOARD_A_FILE: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
static BOARD_H_FILE: u64 = 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;

static WK_PAWN_COVER_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_11111111_11111111_00000000;
static BK_PAWN_COVER_MASK: u64 = 0b00000000_11111111_11111111_00000000_00000000_00000000_00000000_00000000;

// The square values below are mostly taken from the CPW page authored by Tomasz Michniewski, with slight modification.

static SQR_TABLE_BP: [i32; def::BOARD_SIZE] = [
     0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 20, 20, 30, 30, 20, 20, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0,
];

static SQR_TABLE_BP_ENDGAME: [i32; def::BOARD_SIZE] = [
     0,  0,  0,  0,  0,  0,  0,  0,
    20, 30, 30, 30, 30, 30, 30, 20,
    15, 20, 20, 20, 20, 20, 20, 15,
    10, 15, 15, 15, 15, 15, 15, 10,
     5, 10, 10, 10, 10, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,
];

static SQR_TABLE_WP: [i32; def::BOARD_SIZE] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10,-20,-20, 10, 10,  5,
     5, -5,-10,  0,  0,-10, -5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5,  5, 10, 25, 25, 10,  5,  5,
    10, 20, 20, 30, 30, 20, 20, 10,
    50, 50, 50, 50, 50, 50, 50, 50,
     0,  0,  0,  0,  0,  0,  0,  0,
];

static SQR_TABLE_WP_ENDGAME: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      5, 10, 10, 10, 10, 10, 10,  5,
     10, 15, 15, 15, 15, 15, 15, 10,
     15, 20, 20, 20, 20, 20, 20, 15,
     20, 30, 30, 30, 30, 30, 30, 20,
      0,  0,  0,  0,  0,  0,  0,  0,
];

static SQR_TABLE_BN: [i32; def::BOARD_SIZE] = [
    -30,-20,-20,-20,-20,-20,-20,-30,
    -20,-20,  0,  0,  0,  0,-20,-20,
    -20,  0, 10, 15, 15, 10,  0,-20,
    -20,  5, 15, 20, 20, 15,  5,-20,
    -20,  0, 15, 20, 20, 15,  0,-20,
    -20,  5, 10, 15, 15, 10,  5,-20,
    -20,-20,  0,  0,  0,  0,-20,-20,
    -30,-20,-20,-20,-20,-20,-20,-30,
];

static SQR_TABLE_WN: [i32; def::BOARD_SIZE] = [
    -30,-20,-20,-20,-20,-20,-20,-30,
    -20,-20,  0,  0,  0,  0,-20,-20,
    -20,  5, 10, 15, 15, 10,  5,-20,
    -20,  0, 15, 20, 20, 15,  0,-20,
    -20,  5, 15, 20, 20, 15,  5,-20,
    -20,  0, 10, 15, 15, 10,  0,-20,
    -20,-20,  0,  0,  0,  0,-20,-20,
    -30,-20,-20,-20,-20,-20,-20,-30,
];

static SQR_TABLE_BB: [i32; def::BOARD_SIZE] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -20,  0, 10,  0,  0, 10,  0,-20,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10,  5,  5, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

static SQR_TABLE_WB: [i32; def::BOARD_SIZE] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -10, 10, 10,  5,  5, 10, 10,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -20,  0, 10,  0,  0, 10,  0,-20,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

static SQR_TABLE_BR: [i32; def::BOARD_SIZE] = [
    5, 10, 20, 20, 20, 20, 10,  5,
   10, 20, 30, 30, 30, 30, 20, 10,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
  -10, -5,  0,  0,  0,  0, -5,-10,
    0,  0,  0,  0,  0,  0,  0,  0,
];

static SQR_TABLE_WR: [i32; def::BOARD_SIZE] = [
    0,  0,  0,  0,  0,  0,  0,  0,
  -10, -5,  0,  0,  0,  0, -5,-10,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   10, 20, 30, 30, 30, 30, 20, 10,
    5, 10, 20, 20, 20, 20, 10,  5,
];

static SQR_TABLE_BQ: [i32; def::BOARD_SIZE] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];

static SQR_TABLE_WQ: [i32; def::BOARD_SIZE] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -10,  5,  5,  5,  5,  5,  0,-10,
      0,  0,  5,  5,  5,  5,  0, -5,
     -5,  0,  5,  5,  5,  5,  0, -5,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];

static SQR_TABLE_BK: [i32; def::BOARD_SIZE] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20,
];

static SQR_TABLE_WK: [i32; def::BOARD_SIZE] = [
     20, 30, 10,  0,  0, 10, 30, 20,
     20, 20,  0,  0,  0,  0, 20, 20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
];

static SQR_TABLE_BK_ENDGAME: [i32; def::BOARD_SIZE] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50,
];

static SQR_TABLE_WK_ENDGAME: [i32; def::BOARD_SIZE] = [
    -50,-30,-30,-30,-30,-30,-30,-50,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -50,-40,-30,-20,-20,-30,-40,-50,
];

#[derive(PartialEq, Debug)]
pub struct FeatureMap {
    pawn_count: i32,
    queen_count: i32,
    rook_count: i32,
    bishop_count: i32,
    knight_count: i32,

    midgame_sqr_point_count: i32,
    endgame_sqr_point_count: i32,

    passed_pawn_count: i32,
    passed_pawn_rank_count: i32,
    unstoppable_passed_pawn_count: i32,
    controlled_passed_pawn_count: i32,

    dup_pawn_count: i32,
    isolate_pawn_count: i32,
    behind_pawn_count: i32,

    mobility: i32,

    trapped_knight_count: i32,
    trapped_bishop_count: i32,
    trapped_rook_count: i32,
    trapped_queen_count: i32,

    blocked_knight_count: i32,
    blocked_bishop_count: i32,
    blocked_rook_count: i32,
    blocked_queen_count: i32,

    semi_open_rook_count: i32,
    open_rook_count: i32,

    open_queen_count: i32,

    threatened_piece_count: i32,
    defended_piece_count: i32,
    defended_pawn_count: i32,

    king_exposed: i32,
    king_threat_count: i32,
    king_pawn_threat_count: i32,
    king_lost_cas_rights: i32,
}

impl FeatureMap {
    pub fn empty() -> Self {
        FeatureMap {
            pawn_count: 0,
            queen_count: 0,
            rook_count: 0,
            bishop_count: 0,
            knight_count: 0,

            midgame_sqr_point_count: 0,
            endgame_sqr_point_count: 0,

            passed_pawn_count: 0,
            passed_pawn_rank_count: 0,
            unstoppable_passed_pawn_count: 0,
            controlled_passed_pawn_count: 0,

            dup_pawn_count: 0,
            isolate_pawn_count: 0,
            behind_pawn_count: 0,

            mobility: 0,

            trapped_knight_count: 0,
            trapped_bishop_count: 0,
            trapped_rook_count: 0,
            trapped_queen_count: 0,

            blocked_knight_count: 0,
            blocked_bishop_count: 0,
            blocked_rook_count: 0,
            blocked_queen_count: 0,

            semi_open_rook_count: 0,
            open_rook_count: 0,

            open_queen_count: 0,

            threatened_piece_count: 0,
            defended_piece_count: 0,
            defended_pawn_count: 0,

            king_exposed: 0,
            king_threat_count: 0,
            king_pawn_threat_count: 0,
            king_lost_cas_rights: 0,
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

pub fn eval_materials(state: &State) -> (i32, bool) {
    let bitboard = state.bitboard;

    if bitboard.w_pawn | bitboard.b_pawn | bitboard.w_queen | bitboard.b_queen | bitboard.w_rook | bitboard.b_rook == 0 {
        if ((bitboard.w_bishop | bitboard.w_knight).count_ones() as i32 - (bitboard.b_bishop | bitboard.b_knight).count_ones() as i32).abs() < 2 {
            return (0, true)
        }

        if (bitboard.w_bishop | bitboard.w_knight) == 0 && bitboard.b_bishop == 0 && bitboard.b_knight.count_ones() < 3 {
            return (0, true)
        }

        if (bitboard.b_bishop | bitboard.b_knight) == 0 && bitboard.w_bishop == 0 && bitboard.w_knight.count_ones() < 3 {
            return (0, true)
        }
    }

    let score_sign = if state.player == def::PLAYER_W {
        1
    } else {
        -1
    };

    let score = (bitboard.w_queen.count_ones() as i32 * Q_VAL
    + bitboard.w_rook.count_ones() as i32 * R_VAL
    + bitboard.w_bishop.count_ones() as i32 * B_VAL
    + bitboard.w_knight.count_ones() as i32 * N_VAL
    + bitboard.w_pawn.count_ones() as i32 * P_VAL
    - bitboard.b_queen.count_ones() as i32 * Q_VAL
    - bitboard.b_rook.count_ones() as i32 * R_VAL
    - bitboard.b_bishop.count_ones() as i32 * B_VAL
    - bitboard.b_knight.count_ones() as i32 * N_VAL
    - bitboard.b_pawn.count_ones() as i32 * P_VAL) * score_sign;

    (score, false)
}

pub fn get_phase(state: &State) -> i32 {
    let bitboard = state.bitboard;

    (bitboard.w_queen | bitboard.b_queen).count_ones() as i32 * Q_PHASE_WEIGHT
    + (bitboard.w_rook | bitboard.b_rook).count_ones() as i32 * R_PHASE_WEIGHT
    + (bitboard.w_bishop | bitboard.b_bishop).count_ones() as i32 * B_PHASE_WEIGHT
    + (bitboard.w_knight | bitboard.b_knight).count_ones() as i32 * N_PHASE_WEIGHT
}

pub fn eval_state(state: &State, material_score: i32) -> i32 {
    let score_sign = if state.player == def::PLAYER_W {
        1
    } else {
        -1
    };

    let (w_features_map, b_features_map) = extract_features(state);

    let shared_positional_score =
        w_features_map.trapped_knight_count * TRAPPED_N_PEN
        + w_features_map.trapped_bishop_count * TRAPPED_B_PEN
        + w_features_map.trapped_rook_count * TRAPPED_R_PEN
        + w_features_map.trapped_queen_count * TRAPPED_Q_PEN
        + w_features_map.blocked_knight_count * BLOCKED_N_PEN
        + w_features_map.blocked_bishop_count * BLOCKED_B_PEN
        + w_features_map.blocked_rook_count * BLOCKED_R_PEN
        + w_features_map.blocked_queen_count * BLOCKED_Q_PEN
        + w_features_map.threatened_piece_count * THREATENED_PIECE_PEN
        + w_features_map.defended_piece_count * DEFENDED_PIECE_VAL
        + w_features_map.defended_pawn_count * DEFENDED_PAWN_VAL
        - b_features_map.trapped_knight_count * TRAPPED_N_PEN
        - b_features_map.trapped_bishop_count * TRAPPED_B_PEN
        - b_features_map.trapped_rook_count * TRAPPED_R_PEN
        - b_features_map.trapped_queen_count * TRAPPED_Q_PEN
        - b_features_map.blocked_knight_count * BLOCKED_N_PEN
        - b_features_map.blocked_bishop_count * BLOCKED_B_PEN
        - b_features_map.blocked_rook_count * BLOCKED_R_PEN
        - b_features_map.blocked_queen_count * BLOCKED_Q_PEN
        - b_features_map.threatened_piece_count * THREATENED_PIECE_PEN
        - b_features_map.defended_piece_count * DEFENDED_PIECE_VAL
        - b_features_map.defended_pawn_count * DEFENDED_PAWN_VAL;

    let midgame_positional_score =
        w_features_map.midgame_sqr_point_count
        + w_features_map.semi_open_rook_count * ROOK_SEMI_OPEN_LINE_VAL
        + w_features_map.open_rook_count * ROOK_OPEN_LINE_VAL
        + w_features_map.open_queen_count * QUEEN_OPEN_LINE_VAL
        + w_features_map.mobility * MIDGAME_MOB_BASE_VAL
        + w_features_map.king_exposed * KING_EXPOSED_PEN
        + w_features_map.king_threat_count * KING_THREAT_BASE_PEN
        + w_features_map.king_pawn_threat_count * KING_PAWN_THREAT_BASE_PEN
        + w_features_map.behind_pawn_count * BEHIND_PAWN_PEN
        + w_features_map.king_lost_cas_rights * KING_LOST_CAS_RIGHTS_PEN
        - b_features_map.midgame_sqr_point_count
        - b_features_map.semi_open_rook_count * ROOK_SEMI_OPEN_LINE_VAL
        - b_features_map.open_rook_count * ROOK_OPEN_LINE_VAL
        - b_features_map.open_queen_count * QUEEN_OPEN_LINE_VAL
        - b_features_map.mobility * MIDGAME_MOB_BASE_VAL
        - b_features_map.king_exposed * KING_EXPOSED_PEN
        - b_features_map.king_threat_count * KING_THREAT_BASE_PEN
        - b_features_map.king_pawn_threat_count * KING_PAWN_THREAT_BASE_PEN
        - b_features_map.behind_pawn_count * BEHIND_PAWN_PEN
        - b_features_map.king_lost_cas_rights * KING_LOST_CAS_RIGHTS_PEN;

    let endgame_positional_score =
        w_features_map.endgame_sqr_point_count
        + w_features_map.passed_pawn_count * PASS_PAWN_BASE_VAL
        + w_features_map.passed_pawn_rank_count * PASS_PAWN_RANK_VAL
        + w_features_map.unstoppable_passed_pawn_count * UNSTOPPABLE_PASS_PAWN_VAL
        + w_features_map.controlled_passed_pawn_count * CONTROLLED_PASS_PAWN_VAL
        + w_features_map.isolate_pawn_count * ISOLATE_PAWN_PEN
        + w_features_map.dup_pawn_count * DUP_PAWN_PEN
        + w_features_map.mobility * ENDGMAE_MOB_BASE_VAL
        + w_features_map.rook_count * ENDGAME_ROOK_EXTRA_VAL
        + w_features_map.queen_count * ENDGAME_QUEEN_EXTRA_VAL
        - b_features_map.endgame_sqr_point_count
        - b_features_map.passed_pawn_count * PASS_PAWN_BASE_VAL
        - b_features_map.passed_pawn_rank_count * PASS_PAWN_RANK_VAL
        - b_features_map.unstoppable_passed_pawn_count * UNSTOPPABLE_PASS_PAWN_VAL
        - b_features_map.controlled_passed_pawn_count * CONTROLLED_PASS_PAWN_VAL
        - b_features_map.isolate_pawn_count * ISOLATE_PAWN_PEN
        - b_features_map.dup_pawn_count * DUP_PAWN_PEN
        - b_features_map.mobility * ENDGMAE_MOB_BASE_VAL
        - b_features_map.rook_count * ENDGAME_ROOK_EXTRA_VAL
        - b_features_map.queen_count * ENDGAME_QUEEN_EXTRA_VAL;

    let phase = w_features_map.queen_count * Q_PHASE_WEIGHT
    + w_features_map.rook_count * R_PHASE_WEIGHT
    + w_features_map.bishop_count * B_PHASE_WEIGHT
    + w_features_map.knight_count * N_PHASE_WEIGHT
    + b_features_map.queen_count * Q_PHASE_WEIGHT
    + b_features_map.rook_count * R_PHASE_WEIGHT
    + b_features_map.bishop_count * B_PHASE_WEIGHT
    + b_features_map.knight_count * N_PHASE_WEIGHT;

    let extra_score = shared_positional_score + (midgame_positional_score * phase + endgame_positional_score * (TOTAL_PHASE - phase)) / TOTAL_PHASE;

    material_score + extra_score * score_sign + TEMPO_VAL
}

#[inline]
fn extract_features(state: &State) -> (FeatureMap, FeatureMap) {
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
    let mut wq_attack_mask = 0;

    let mut bp_attack_mask = 0;
    let mut bn_attack_mask = 0;
    let mut bb_attack_mask = 0;
    let mut br_attack_mask = 0;
    let mut bq_attack_mask = 0;

    let mut w_semi_open_line_mask = 0;
    let mut b_semi_open_line_mask = 0;

    let mut mov_mask_map = [0; def::BOARD_SIZE];

    let occupy_mask = bitboard.w_all | bitboard.b_all;
    let start_index = occupy_mask.trailing_zeros() as usize;
    let end_index = def::BOARD_SIZE - occupy_mask.leading_zeros() as usize;

    let piece_mask = bitboard.w_knight | bitboard.w_bishop | bitboard.w_rook | bitboard.w_queen | bitboard.b_knight | bitboard.b_bishop | bitboard.b_rook | bitboard.b_queen;

    for index in start_index..end_index {
        let moving_piece = squares[index];

        if moving_piece == 0 {
            continue
        }

        let index_mask = index_masks[index];

        match moving_piece {
            def::WP => {
                w_feature_map.midgame_sqr_point_count += SQR_TABLE_WP[index];
                w_feature_map.endgame_sqr_point_count += SQR_TABLE_WP_ENDGAME[index];

                wp_attack_mask |= bitmask.wp_attack_masks[index];

                let file_mask = file_masks[index];
                let forward_mask = bitmask.wp_forward_masks[index];
                let behind_mask = bitmask.wp_behind_masks[index];
                let rank = def::get_rank(def::PLAYER_W, index) as i32;

                if forward_mask & (bitboard.b_pawn | (bitboard.w_pawn & file_mask)) == 0 {
                    w_feature_map.passed_pawn_count += 1;
                    w_feature_map.passed_pawn_rank_count += rank;

                    if behind_mask & bitmask.k_attack_masks[index] & bitboard.w_pawn != 0 {
                        w_feature_map.passed_pawn_rank_count += 1;
                    }

                    if piece_mask == 0 {
                        let king_distance = def::get_file_distance(index, state.bk_index);
                        if state.player == def::PLAYER_W {
                            if king_distance > 7 - rank {
                                w_feature_map.unstoppable_passed_pawn_count += 1;
                            }
                        } else {
                            if king_distance - 1 > 7 - rank {
                                w_feature_map.unstoppable_passed_pawn_count += 1;
                            }
                        }

                        let pawn_control_mask = bitmask.wp_front_control_sqr_masks[index];
                        if pawn_control_mask == 0 || pawn_control_mask & bitmask.index_masks[state.wk_index] != 0 {
                            w_feature_map.controlled_passed_pawn_count += 1;
                        }
                    }
                }

                if behind_mask & bitboard.w_pawn == 0 {
                    if forward_mask & !file_mask & bitboard.w_pawn == 0 {
                        w_feature_map.isolate_pawn_count += 1;
                    } else {
                        w_feature_map.behind_pawn_count += 1;

                        if forward_mask & file_mask & bitboard.b_pawn == 0 {
                            w_feature_map.behind_pawn_count += 1;
                        }
                    }
                }

                if (file_mask & bitboard.w_pawn).count_ones() > 1 {
                    w_feature_map.dup_pawn_count += 1;
                }
            },
            def::BP => {
                b_feature_map.midgame_sqr_point_count += SQR_TABLE_BP[index];
                b_feature_map.endgame_sqr_point_count += SQR_TABLE_BP_ENDGAME[index];

                bp_attack_mask |= bitmask.bp_attack_masks[index];

                let file_mask = file_masks[index];
                let forward_mask = bitmask.bp_forward_masks[index];
                let behind_mask = bitmask.bp_behind_masks[index];
                let rank = def::get_rank(def::PLAYER_B, index) as i32;

                if forward_mask & (bitboard.w_pawn | (bitboard.b_pawn & file_mask)) == 0 {
                    b_feature_map.passed_pawn_count += 1;
                    b_feature_map.passed_pawn_rank_count += rank;

                    if behind_mask & bitmask.k_attack_masks[index] & bitboard.b_pawn != 0 {
                        b_feature_map.passed_pawn_rank_count += 1;
                    }

                    if piece_mask == 0 {
                        let king_distance = def::get_file_distance(index, state.wk_index);
                        if state.player == def::PLAYER_B {
                            if king_distance > 7 - rank {
                                b_feature_map.unstoppable_passed_pawn_count += 1;
                            }
                        } else {
                            if king_distance - 1 > 7 - rank {
                                b_feature_map.unstoppable_passed_pawn_count += 1;
                            }
                        }

                        let pawn_control_mask = bitmask.bp_front_control_sqr_masks[index];
                        if pawn_control_mask == 0 || pawn_control_mask & bitmask.index_masks[state.bk_index] != 0 {
                            b_feature_map.controlled_passed_pawn_count += 1;
                        }
                    }
                }

                if behind_mask & bitboard.b_pawn == 0 {
                    if forward_mask & !file_mask & bitboard.b_pawn == 0 {
                        b_feature_map.isolate_pawn_count += 1;
                    } else {
                        b_feature_map.behind_pawn_count += 1;

                        if forward_mask & file_mask & bitboard.w_pawn == 0 {
                            b_feature_map.behind_pawn_count += 1;
                        }
                    }
                }

                if (file_mask & bitboard.b_pawn).count_ones() > 1 {
                    b_feature_map.dup_pawn_count += 1;
                }
            },

            def::WN => {
                w_feature_map.midgame_sqr_point_count += SQR_TABLE_WN[index];

                let mov_mask = bitmask.n_attack_masks[index];
                wn_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },
            def::BN => {
                b_feature_map.midgame_sqr_point_count += SQR_TABLE_BN[index];

                let mov_mask = bitmask.n_attack_masks[index];
                bn_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },

            def::WB => {
                w_feature_map.midgame_sqr_point_count += SQR_TABLE_WB[index];

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
                mov_mask_map[index] = mov_mask;
            },
            def::BB => {
                b_feature_map.midgame_sqr_point_count += SQR_TABLE_BB[index];

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
                mov_mask_map[index] = mov_mask;
            },

            def::WR => {
                w_feature_map.midgame_sqr_point_count += SQR_TABLE_WR[index];

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
                mov_mask_map[index] = mov_mask;

                let file_mask = file_masks[index];
                if file_mask & (bitboard.w_all ^ bitboard.w_rook) == 0 {
                    if file_mask & bitboard.b_all == 0 {
                        w_feature_map.open_rook_count += 1;
                    } else if w_semi_open_line_mask & file_mask == 0 {
                        w_feature_map.semi_open_rook_count += 1;
                        w_semi_open_line_mask |= file_mask;
                    }
                }
            },
            def::BR => {
                b_feature_map.midgame_sqr_point_count += SQR_TABLE_BR[index];

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
                mov_mask_map[index] = mov_mask;

                let file_mask = file_masks[index];
                if file_mask & (bitboard.b_all ^ bitboard.b_rook) == 0 {
                    if file_mask & bitboard.w_all == 0 {
                        b_feature_map.open_rook_count += 1;
                    } else if b_semi_open_line_mask & file_mask == 0 {
                        b_feature_map.semi_open_rook_count += 1;
                        b_semi_open_line_mask |= file_mask;
                    }
                }
            },

            def::WQ => {
                w_feature_map.midgame_sqr_point_count += SQR_TABLE_WQ[index];

                let file_mask = file_masks[index];
                if file_mask & ((bitboard.w_all | bitboard.b_all) ^ index_mask) == 0 {
                    w_feature_map.open_queen_count += 1;
                }

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

                wq_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },
            def::BQ => {
                b_feature_map.midgame_sqr_point_count += SQR_TABLE_BQ[index];

                let file_mask = file_masks[index];
                if file_mask & ((bitboard.w_all | bitboard.b_all) ^ index_mask) == 0 {
                    b_feature_map.open_queen_count += 1;
                }

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

                bq_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },

            def::WK => {
                w_feature_map.midgame_sqr_point_count += SQR_TABLE_WK[index];
                w_feature_map.endgame_sqr_point_count += SQR_TABLE_WK_ENDGAME[index];

                let file_mask = file_masks[index];

                if bitboard.b_rook | bitboard.b_queen != 0 {
                    if file_mask & bitboard.w_pawn & WK_PAWN_COVER_MASK == 0 {
                        w_feature_map.king_exposed += 1;

                        if file_mask & bitboard.w_pawn == 0 {
                            w_feature_map.king_exposed += 1;
                        }
                    }

                    if file_mask & BOARD_A_FILE == 0 {
                        let left_file_mask = file_masks[index - 1];
                        if left_file_mask & bitboard.w_pawn == 0 && left_file_mask & bitboard.w_rook == 0 {
                            w_feature_map.king_exposed += 1;
                        }
                    }

                    if file_mask & BOARD_H_FILE == 0 {
                        let right_file_mask = file_masks[index + 1];
                        if right_file_mask & bitboard.w_pawn == 0 && right_file_mask & bitboard.w_rook == 0 {
                            w_feature_map.king_exposed += 1;
                        }
                    }

                    if bitboard.b_rook != 0 && file_mask & bitboard.b_pawn == 0 {
                        w_feature_map.king_exposed += 1;
                    }
                }
            },
            def::BK => {
                b_feature_map.midgame_sqr_point_count += SQR_TABLE_BK[index];
                b_feature_map.endgame_sqr_point_count += SQR_TABLE_BK_ENDGAME[index];

                let file_mask = file_masks[index];

                if bitboard.w_rook | bitboard.w_queen != 0 {
                    if file_mask & bitboard.b_pawn & BK_PAWN_COVER_MASK == 0 {
                        b_feature_map.king_exposed += 1;

                        if file_mask & bitboard.b_pawn == 0 {
                            b_feature_map.king_exposed += 1;
                        }
                    }

                    if file_mask & BOARD_A_FILE == 0 {
                        let left_file_mask = file_masks[index - 1];
                        if left_file_mask & bitboard.b_pawn == 0 && left_file_mask & bitboard.b_rook == 0 {
                            b_feature_map.king_exposed += 1;
                        }
                    }

                    if file_mask & BOARD_H_FILE == 0 {
                        let right_file_mask = file_masks[index + 1];
                        if right_file_mask & bitboard.b_pawn == 0 && right_file_mask & bitboard.b_rook == 0 {
                            b_feature_map.king_exposed += 1;
                        }
                    }

                    if bitboard.w_rook != 0 && file_mask & bitboard.w_pawn == 0 {
                        b_feature_map.king_exposed += 1;
                    }
                }
            },
            _ => {}
        }
    }

    let w_attack_mask = wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask | wq_attack_mask;
    let b_attack_mask = bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask | bq_attack_mask;

    // piece counts

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

    // check trapped pieces

    for index in start_index..end_index {
        let moving_piece = squares[index];

        if moving_piece == 0 {
            continue
        }

        match moving_piece {
            def::WN => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !bp_attack_mask;
                mov_mask &= !(b_attack_mask & !wp_attack_mask);

                if mov_mask == 0 && w_attack_mask & index_masks[index] == 0 {
                    w_feature_map.trapped_knight_count += 1;
                } else if mov_mask & !bitboard.w_all == 0 {
                    w_feature_map.blocked_knight_count += 1;
                }
            },
            def::WB => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !bp_attack_mask;
                mov_mask &= !(b_attack_mask & !wp_attack_mask);

                if mov_mask == 0 && w_attack_mask & index_masks[index] == 0 {
                    w_feature_map.trapped_bishop_count += 1;
                } else if mov_mask & !bitboard.w_all == 0 {
                    w_feature_map.blocked_bishop_count += 1;
                }
            },
            def::WR => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !(bp_attack_mask | bn_attack_mask | bb_attack_mask);

                if mov_mask == 0 && w_attack_mask & index_masks[index] == 0 {
                    w_feature_map.trapped_rook_count += 1;
                } else if mov_mask & !bitboard.w_all == 0 {
                    w_feature_map.blocked_rook_count += 1;
                }
            },
            def::WQ => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask);

                if mov_mask == 0 {
                    w_feature_map.trapped_queen_count += 1;
                } else if mov_mask & !bitboard.w_all == 0 {
                    w_feature_map.blocked_queen_count += 1;
                }
            },

            def::BN => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !wp_attack_mask;
                mov_mask &= !(w_attack_mask & !bp_attack_mask);

                if mov_mask == 0 && b_attack_mask & index_masks[index] == 0 {
                    b_feature_map.trapped_knight_count += 1;
                } else if mov_mask & !bitboard.b_all == 0 {
                    b_feature_map.blocked_knight_count += 1;
                }
            },
            def::BB => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !wp_attack_mask;
                mov_mask &= !(w_attack_mask & !bp_attack_mask);

                if mov_mask == 0 && b_attack_mask & index_masks[index] == 0 {
                    b_feature_map.trapped_bishop_count += 1;
                } else if mov_mask & !bitboard.b_all == 0 {
                    b_feature_map.blocked_bishop_count += 1;
                }
            },
            def::BR => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !(wp_attack_mask | wn_attack_mask | wb_attack_mask);

                if mov_mask == 0 && b_attack_mask & index_masks[index] == 0 {
                    b_feature_map.trapped_rook_count += 1;
                } else if mov_mask & !bitboard.b_all == 0 {
                    b_feature_map.blocked_rook_count += 1;
                }
            },
            def::BQ => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask);

                if mov_mask == 0 {
                    b_feature_map.trapped_queen_count += 1;
                } else if mov_mask & !bitboard.b_all == 0 {
                    b_feature_map.blocked_queen_count += 1;
                }
            },
            _ => {},
        }
    }

    // king threats

    if bitboard.b_queen != 0 {
        let protector_mask = bitmask.k_attack_masks[state.wk_index];
        w_feature_map.king_pawn_threat_count = (bp_attack_mask & protector_mask).count_ones() as i32;
        w_feature_map.king_threat_count += ((bb_attack_mask & !wp_attack_mask) & protector_mask).count_ones() as i32;
        w_feature_map.king_threat_count += ((br_attack_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask)) & protector_mask).count_ones() as i32;
        w_feature_map.king_threat_count += ((bq_attack_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask)) & protector_mask).count_ones() as i32;
    }

    if bitboard.w_queen != 0 {
        let protector_mask = bitmask.k_attack_masks[state.bk_index];
        b_feature_map.king_pawn_threat_count = (wp_attack_mask & protector_mask).count_ones() as i32;
        b_feature_map.king_threat_count += ((wb_attack_mask & !bp_attack_mask) & protector_mask).count_ones() as i32;
        b_feature_map.king_threat_count += ((wr_attack_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask)) & protector_mask).count_ones() as i32;
        b_feature_map.king_threat_count += ((wq_attack_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask)) & protector_mask).count_ones() as i32;
    }

    // mobility

    w_feature_map.mobility = (wn_attack_mask & !bitboard.w_pawn & !bp_attack_mask).count_ones() as i32;
    w_feature_map.mobility += (wb_attack_mask & !bitboard.w_pawn & !bp_attack_mask).count_ones() as i32;

    b_feature_map.mobility = (bn_attack_mask & !bitboard.b_pawn & !wp_attack_mask).count_ones() as i32;
    b_feature_map.mobility += (bb_attack_mask & !bitboard.b_pawn & !wp_attack_mask).count_ones() as i32;

    // penalty for losing castling rights

    if (state.cas_rights | state.cas_history) & 0b1100 == 0 {
        w_feature_map.king_lost_cas_rights = 1;
    }

    if (state.cas_rights | state.cas_history) & 0b0011 == 0 {
        b_feature_map.king_lost_cas_rights = 1;
    }

    // defense on pieces

    for index in 0..def::BOARD_SIZE {
        let index_mask = index_masks[index];

        if index_mask & occupy_mask == 0 {
            continue
        }

        match state.squares[index] {
            def::WP => {
                if bp_attack_mask & index_mask == 0 && (w_attack_mask ^ wq_attack_mask) & index_mask != 0 {
                    w_feature_map.defended_pawn_count += 1;
                }
            },
            def::WN => {
                if bp_attack_mask & index_mask != 0 {
                    w_feature_map.threatened_piece_count += 1;
                } else if w_attack_mask & index_mask != 0 {
                    w_feature_map.defended_piece_count += 1;
                }
            },
            def::WB => {
                if bp_attack_mask & index_mask != 0 {
                    w_feature_map.threatened_piece_count += 1;
                } else if w_attack_mask & index_mask != 0 {
                    w_feature_map.defended_piece_count += 1;
                }
            },
            def::WR => {
                if (bp_attack_mask | bn_attack_mask | bb_attack_mask) & index_mask != 0 {
                    w_feature_map.threatened_piece_count += 1;
                } else if w_attack_mask & index_mask != 0 {
                    w_feature_map.defended_piece_count += 1;
                }
            },
            def::WQ => {
                if (bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask) & index_mask != 0 {
                    w_feature_map.threatened_piece_count += 1;
                }
            },

            def::BP => {
                if wp_attack_mask & index_mask == 0 && (b_attack_mask ^ bq_attack_mask) & index_mask != 0 {
                    b_feature_map.defended_pawn_count += 1;
                }
            },
            def::BN => {
                if wp_attack_mask & index_mask != 0 {
                    b_feature_map.threatened_piece_count += 1;
                } else if b_attack_mask & index_mask != 0 {
                    b_feature_map.defended_piece_count += 1;
                }
            },
            def::BB => {
                if wp_attack_mask & index_mask != 0 {
                    b_feature_map.threatened_piece_count += 1;
                } else if b_attack_mask & index_mask != 0 {
                    b_feature_map.defended_piece_count += 1;
                }
            },
            def::BR => {
                if (wp_attack_mask | wn_attack_mask | wb_attack_mask) & index_mask != 0 {
                    b_feature_map.threatened_piece_count += 1;
                } else if b_attack_mask & index_mask != 0 {
                    b_feature_map.defended_piece_count += 1;
                }
            },
            def::BQ => {
                if (wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask) & index_mask != 0 {
                    b_feature_map.threatened_piece_count += 1;
                }
            },

            _ => {},
        }
    }

    (w_feature_map, b_feature_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bitboard::BitMask,
        state::State,
        prng::XorshiftPrng,
        util,
   };

    #[test]
    fn test_extract_features_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("r1b2rk1/pp2ppbp/1qnp2p1/2nP4/P2QPP2/N1P2N2/2B3PP/1RB2RK1 b - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(5, w_features.defended_piece_count);
        assert_eq!(1, w_features.threatened_piece_count);
        assert_eq!(3, b_features.defended_piece_count);
        assert_eq!(2, b_features.threatened_piece_count);
    }

    #[test]
    fn test_extract_features_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1q4kn/3r1p1p/1pbN1Pp1/r1ppP1P1/P4R2/2B1P3/2Q4P/3R2K1 b - - 2 29", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(9, w_features.mobility);
        assert_eq!(4, b_features.mobility);
    }

    #[test]
    fn test_extract_features_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1kr3r1/pp3p1p/P1pn4/2Bpb3/4p2q/3PP3/PPP1NPPP/R2Q1RK1 b - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.king_exposed);
        assert_eq!(3, w_features.king_threat_count);

        assert_eq!(0, b_features.king_exposed);
        assert_eq!(1, b_features.king_threat_count);
    }

    #[test]
    fn test_extract_features_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/p1p2pp1/5pp1/8/7P/5PPP/1P3P2/8 w - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(4, w_features.dup_pawn_count);
        assert_eq!(1, w_features.isolate_pawn_count);
        assert_eq!(1, w_features.behind_pawn_count);

        assert_eq!(4, b_features.dup_pawn_count);
        assert_eq!(2, b_features.isolate_pawn_count);
        assert_eq!(0, b_features.behind_pawn_count);
    }

    #[test]
    fn test_extract_features_5() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/p7/1p6/2p5/4P3/4P3/4P3/8 w - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(3, w_features.dup_pawn_count);
        assert_eq!(3, w_features.isolate_pawn_count);

        assert_eq!(0, b_features.dup_pawn_count);
        assert_eq!(0, b_features.isolate_pawn_count);
    }

    #[test]
    fn test_extract_features_6() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnbqkbnr/3p4/6P1/3P4/1pp5/1N6/3P4/R1BQKBNR w KQkq - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.passed_pawn_count);
        assert_eq!(5, w_features.passed_pawn_rank_count);

        assert_eq!(1, b_features.passed_pawn_count);
        assert_eq!(5, b_features.passed_pawn_rank_count);
    }

    #[test]
    fn test_extract_features_7() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/8/2K1kp2/8/3P4/8/8/8 w - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.passed_pawn_count);
        assert_eq!(1, w_features.controlled_passed_pawn_count);

        assert_eq!(1, b_features.passed_pawn_count);
        assert_eq!(0, b_features.controlled_passed_pawn_count);
    }

    #[test]
    fn test_extract_features_8() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnbqkbnr/ppp3p1/3p1p1p/4p3/2P1P3/P2P4/1P3PPP/RNBQKBNR w KQkq - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(2, w_features.behind_pawn_count);
        assert_eq!(1, b_features.behind_pawn_count);
    }

    #[test]
    fn test_extract_features_9() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1krq1bnr/p1p1ppp1/2P5/8/1P3P2/6pP/2PPP1P1/RNBQR1K1 w Qk - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(0, w_features.king_exposed);
        assert_eq!(2, w_features.king_pawn_threat_count);

        assert_eq!(2, b_features.king_exposed);
        assert_eq!(1, b_features.king_pawn_threat_count);
    }

    #[test]
    fn test_extract_features_x() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let mut state = State::new("r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(0, w_features.king_lost_cas_rights);
        assert_eq!(0, b_features.king_lost_cas_rights);

        state.do_mov(util::map_sqr_notation_to_index("e1"), util::map_sqr_notation_to_index("e2"), def::MOV_REG, 0);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.king_lost_cas_rights);
        assert_eq!(0, b_features.king_lost_cas_rights);

        state.do_mov(util::map_sqr_notation_to_index("h8"), util::map_sqr_notation_to_index("f8"), def::MOV_REG, 0);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.king_lost_cas_rights);
        assert_eq!(0, b_features.king_lost_cas_rights);

        state.do_mov(util::map_sqr_notation_to_index("e2"), util::map_sqr_notation_to_index("e1"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("a8"), util::map_sqr_notation_to_index("b8"), def::MOV_REG, 0);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.king_lost_cas_rights);
        assert_eq!(1, b_features.king_lost_cas_rights);
    }

    #[test]
    fn test_extract_features_x1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1nbqkbrr/p1ppppp1/1p6/8/8/8/RPPPPPPP/RNBQKBN1 w Qk - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.semi_open_rook_count);
        assert_eq!(1, b_features.semi_open_rook_count);
    }

    #[test]
    fn test_extract_features_pawn_promo() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/6k1/8/6p1/1P6/8/1K6/8 w - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.passed_pawn_count);
        assert_eq!(1, w_features.unstoppable_passed_pawn_count);
        assert_eq!(1, b_features.passed_pawn_count);
        assert_eq!(0, b_features.unstoppable_passed_pawn_count);

        let state = State::new("8/6k1/8/6p1/1P6/8/1K6/8 b - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.passed_pawn_count);
        assert_eq!(0, w_features.unstoppable_passed_pawn_count);
        assert_eq!(1, b_features.passed_pawn_count);
        assert_eq!(1, b_features.unstoppable_passed_pawn_count);

        let state = State::new("8/3k4/8/8/8/8/3K1p2/8 b - - 0 1", &zob_keys, &bitmask);
        let (_w_features, b_features) = extract_features(&state);

        assert_eq!(1, b_features.passed_pawn_count);
        assert_eq!(1, b_features.unstoppable_passed_pawn_count);

        let state = State::new("8/1P1k4/8/8/8/2K5/8/8 b - - 0 1", &zob_keys, &bitmask);
        let (w_features, _b_features) = extract_features(&state);

        assert_eq!(1, w_features.passed_pawn_count);
        assert_eq!(0, w_features.unstoppable_passed_pawn_count);
    }

    #[test]
    fn test_draw_endgame_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/2k5/8/8/8/4N3/5K2/8 w - - 0 1", &zob_keys, &bitmask);
        let (score, is_draw) = eval_materials(&state);
        assert_eq!(0, score);
        assert!(is_draw);
    }

    #[test]
    fn test_draw_endgame_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/2k5/8/4N3/8/4N3/5K2/8 w - - 0 1", &zob_keys, &bitmask);
        let (score, is_draw) = eval_materials(&state);
        assert_eq!(0, score);
        assert!(is_draw);
    }

    #[test]
    fn test_draw_endgame_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("4b3/2k5/8/5N2/4K3/8/8/8 b - - 0 1", &zob_keys, &bitmask);
        let (score, is_draw) = eval_materials(&state);
        assert_eq!(0, score);
        assert!(is_draw);
    }
}
