/*
 * Copyright (C) 2020 Zixiao Han
 */

mod bitboard;
mod def;
mod eval;
mod hashtable;
mod mov_table;
mod prng;
mod search;
mod state;
mod uci;
mod util;

use bitboard::BitMask;
use prng::XorshiftPrng;
use state::State;
use search::SearchEngine;
use uci::{UciProcessResult, Rawmov};

use std::io::{self, prelude::*};
use std::u128;

const DEFAULT_MAX_TIME_MILLIS: u128 = u128::MAX;
const DEFAULT_MAX_DEPTH: u8 = 64;

fn main() {
    if 1u8 != 0b01 {
        println!("only litte-endian systems are supported");
        std::process::exit(0);
    }

    let mut search_engine = SearchEngine::new(def::DEFAULT_HASH_SIZE_UNIT);
    let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
    let bitmask = BitMask::new();
    let mut state = State::new(uci::FEN_START_POS, &zob_keys, &bitmask);

    loop {
        let input_cmd = read_gui_input();
        let uci_cmd_process_result = uci::process_uci_cmd(input_cmd.trim());
        match uci_cmd_process_result {
            UciProcessResult::SetHashSize(hash_size) => {
                search_engine.set_hash_size(hash_size);
                print_uci_ok();
            },
            UciProcessResult::Position(fen_str, mov_list) => {
                state = State::new(&fen_str, &zob_keys, &bitmask);

                if mov_list.is_empty() {
                    continue
                }

                for Rawmov { from, to, promo, origin_mov_str } in mov_list {
                    if !promo.is_empty() {
                        let promo_piece_code = if state.player == def::PLAYER_W {
                            match promo.as_str() {
                                "q" => def::WQ,
                                "r" => def::WR,
                                "b" => def::WB,
                                "n" => def::WN,
                                _ => panic!("invalid promo piece"),
                            }
                        } else {
                            match promo.as_str() {
                                "q" => def::BQ,
                                "r" => def::BR,
                                "b" => def::BB,
                                "n" => def::BN,
                                _ => panic!("invalid promo piece"),
                            }
                        };

                        state.do_mov(from, to, def::MOV_PROMO, promo_piece_code);
                        continue
                    }

                    let moving_piece = state.squares[from];
                    if def::is_k(moving_piece) && ["e1g1", "e1c1", "e8g8", "e8c8"].contains(&origin_mov_str.as_str()) {
                        state.do_mov(from, to, def::MOV_CAS, 0);
                        continue
                    }

                    if def::is_p(moving_piece) {
                        if to == state.enp_square {
                            state.do_mov(from, to, def::MOV_ENP, 0);
                            continue
                        } else if (from as isize - to as isize).abs() == 16 {
                            state.do_mov(from, to, def::MOV_CR_ENP, 0);
                            continue
                        }
                    }

                    state.do_mov(from, to, def::MOV_REG, 0);
                }
            },
            UciProcessResult::StartSearchWithTime(time_millis) => {
                let best_mov = search_engine.search(&mut state, time_millis, DEFAULT_MAX_DEPTH);
                print_best_mov(best_mov);
            },
            UciProcessResult::StartSearchWithComplextTimeControl(time_info) => {
                let time_millis = if state.player == def::PLAYER_W {
                    time_info.white_millis
                } else {
                    time_info.black_millis
                };

                let best_mov = search_engine.search(&mut state, time_millis, DEFAULT_MAX_DEPTH);
                print_best_mov(best_mov);
            },
            UciProcessResult::StartSearchToDepth(depth) => {
                let best_mov = search_engine.search(&mut state, DEFAULT_MAX_TIME_MILLIS, depth);
                print_best_mov(best_mov);
            },
            UciProcessResult::StartSearchInfinite => {
                let best_mov = search_engine.search(&mut state, DEFAULT_MAX_TIME_MILLIS, DEFAULT_MAX_DEPTH);
                print_best_mov(best_mov);
            },
            UciProcessResult::Perft(depth) => {
                let perft_val = search_engine.perft(&mut state, depth);
                println!("depth {} perft {}", depth, perft_val);
            },
            UciProcessResult::PrintDebugInfo => {
                println!("{}", &state);
            }
            UciProcessResult::Stop => {
                search_engine.stop();
                print_uci_ok();
            },
            UciProcessResult::IgnoredOption => {
                print_uci_ok();
            },
            UciProcessResult::Noop => {},
            UciProcessResult::Reset => {
                search_engine.reset();
                print_uci_ok();
            },
            UciProcessResult::Quit => {
                std::process::exit(0);
            }
        }
    }
}

#[inline]
fn print_best_mov(best_mov: u32) {
    println!("bestmove {}", util::format_mov(best_mov));
    io::stdout().flush().ok();
}

#[inline]
fn print_uci_ok() {
    println!("uciok");
    io::stdout().flush().ok();
}

#[inline]
fn read_gui_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(error) => panic!("uable to read input {}", error),
    }
}
