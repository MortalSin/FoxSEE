static BEST_SEED: u64 = 117;

pub struct XorshiftPrng {
    state: u64,
}

impl XorshiftPrng {
    pub fn new() -> XorshiftPrng {
        XorshiftPrng {
            state: BEST_SEED,
        }
    }

    fn gen_rand(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 15;
        x ^= x >> 7;
        x ^= x << 19;
        self.state = x;

        x
    }

    pub fn create_prn_table(&mut self, fst_dim: usize, snd_dim: usize) -> Vec<Vec<u64>> {
        let mut prn_table = vec![vec![0; snd_dim]; fst_dim];

        for i in 0..120 {
            for j in 0..131 {
                let mut prn = self.gen_rand();

                'trail_error: loop {
                    for ii in 0..i {
                        for jj in 0..j {
                            if prn_table[ii][jj] == prn {
                                prn = self.gen_rand();
                                continue 'trail_error
                            }
                        }
                    }

                    break
                }

                prn_table[i][j] = prn;
            }
        }

        prn_table
    }
}
