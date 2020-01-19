type Key = u64;
type Player = u8;
type CasRights = u8;
type Depth = u8;
type EnpSqr = usize;
type Flag = u8;

type TableEntry = (Key, Player, Depth, CasRights, EnpSqr, Flag, i32, u32);

pub const HASH_TYPE_ALPHA: u8 = 2;
pub const HASH_TYPE_BETA: u8 = 3;

#[derive(PartialEq, Debug)]
pub enum LookupResult {
    Match(u8, i32, u32),
    MovOnly(u32),
    NoMatch,
}

use LookupResult::*;

pub struct HashTable {
    mod_base: u64,
    table: Vec<TableEntry>,
}

impl HashTable {
    pub fn new(size: usize) -> Self {
        HashTable {
            mod_base: (size - 1) as u64,
            table: vec![(0, 0, 0, 0, 0, 0, 0, 0); size],
        }
    }

    pub fn get(&self, key: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize) -> LookupResult {
        let (k, p, d, c, e, f, s, m) = self.table[(key & self.mod_base) as usize];
        if k == key && p == player && c == cas_rights && e == enp_sqr {
            if d >= depth {
                Match(f, s, m)
            } else {
                MovOnly(m)
            }
        } else {
            NoMatch
        }
    }

    pub fn set(&mut self, key: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize, flag: u8, score: i32, mov: u32) {
        self.table[(key & self.mod_base) as usize] = (key, player, depth, cas_rights, enp_sqr, flag, score, mov);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_entries() {
        let mut table = HashTable::new(32786);
        table.set(32789, 2, 5, 0b1100, 0, 0, -100, 123);

        assert_eq!(LookupResult::Match(0, -100, 123), table.get(32789, 2, 5, 0b1100, 0));
        assert_eq!(LookupResult::Match(0, -100, 123), table.get(32789, 2, 3, 0b1100, 0));
        assert_eq!(LookupResult::MovOnly(123), table.get(32789, 2, 6, 0b1100, 0));
        assert_eq!(LookupResult::NoMatch, table.get(32789, 2, 5, 0b1110, 0));
        assert_eq!(LookupResult::NoMatch, table.get(32789, 2, 5, 0b1100, 1));
        assert_eq!(LookupResult::NoMatch, table.get(32789, 1, 5, 0b1100, 0));
        assert_eq!(LookupResult::NoMatch, table.get(3, 2, 5, 0b1110, 0));
    }
}
