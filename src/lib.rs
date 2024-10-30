use std::error;
use std::fmt;

/*
 * Yes this is ridiculous and everything is based on the concept that all the information of a Tic-Tac-Toe game fits into a u18
 * and therefore we can exploit all the bits of a u32 with extra information which is neatly done
 * in this code
 */

pub mod ticmasks {
    pub const FIELD: u32 = 0x0003FFFF;
    pub const CURRENTPLAYER: u32 = 0x00040000;
    pub const CURRENTROUND: u32 = 0x00780000;
    pub const REST: u32 = 0xF800000;
}

pub mod ticonst {
    pub const PLAYER_X: u32 = 0x0;
    pub const PLAYER_O: u32 = 0x1;
}

#[derive(Debug)]
pub struct FieldAlreadyUsed {
    failed_field: u8,
    used_by: Player,
}

impl FieldAlreadyUsed {
    fn new(failed_field: u8, used_by: Player) -> FieldAlreadyUsed {
        FieldAlreadyUsed {
            failed_field,
            used_by,
        }
    }
}

impl error::Error for FieldAlreadyUsed {}
impl fmt::Display for FieldAlreadyUsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The field {} is already in use by {}",
            self.failed_field, self.used_by
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Player {
    fn to_bit(&self) -> u32 {
        match self {
            Player::X => ticonst::PLAYER_X,
            Player::O => ticonst::PLAYER_O,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::X => "X",
                Player::O => "O",
            }
        )
    }
}

macro_rules! impl_from_t_player {
    (for $($t:ty),+) => {
        $(impl From<$t> for Player {
            fn from(value: $t) -> Self {
                match value {
                    0 => Player::X,
                    1 => Player::O,
                    _ => Player::O,
                }
            }
        })*
    }
}
impl_from_t_player!(for u8,u16,u32,u64);

pub struct TicField {
    /*
     * 0:18 Field Data
     * 19:19 Current Player
     * 20:24 Current Round
     * 25:32 TBD
     *
     * Field Description
     *
     * 18-------------------0
     * | BABABABABABABABABA |
     * ----------------------
     *  A - If the field on the board is field set
     *      if the second field is set the fourth bit is high (1)
     *      000000000000000100
     *
     *  B - The player that has set the field, only counts if the corresponding A bit is set
     *      Low is for X high is for O - X has set on field 2 and O on field 1
     *      000000000000001101
     *
     *  19 - Field 19 corresponds to the current player and low is for X and and high is for O
     *       X -> 0
     *       O -> 1
     *
     *  20:24 - is a 4 bit counter which max has a value of 1000
     */
    bits: u32,
}

impl TicField {
    pub fn new() -> TicField {
        TicField { bits: 0 }
    }
    fn select_field(&self, n: u8) -> u8 {
        0b11 & ((ticmasks::FIELD & self.bits) >> 2 * n) as u8
    }
    pub fn get_player_for_field(&self, n: u8) -> Player {
        ((self.select_field(n) & 0b10) >> 1).into()
    }
    pub fn is_field_set(&self, n: u8) -> bool {
        self.select_field(n) & 0b01 != 0
    }
    pub fn set_field(&mut self, n: u8, player: Player) -> Result<(), FieldAlreadyUsed> {
        if self.is_field_set(n) {
            Err(FieldAlreadyUsed::new(n, self.get_player_for_field(n)))
        } else {
            let field_value = player.to_bit() << 1 | 0b01;
            self.bits |= field_value << 2 * n;
            Ok(())
        }
    }
    pub fn as_bytes(&self) -> [u8; 4] {
        self.into()
    }
    pub fn from_bytes(value: &[u8]) -> TicField {
        TicField {
            bits: u32::from_ne_bytes(
                value
                    .try_into()
                    .expect("Wrong number of bytes whooopppssssss"),
            ),
        }
    }
    pub fn as_vec(&self) -> Vec<&str> {
        (0..9)
            .map(|x| {
                if self.is_field_set(x) {
                    match self.get_player_for_field(x) {
                        Player::X => "X",
                        Player::O => "O",
                    }
                } else {
                    "_"
                }
            })
            .collect()
    }
}

impl fmt::Display for TicField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ret = write!(f, "\nBits: {:#032b}\n", self.bits);
        let tmp = self.as_vec();
        for y in 0..3 {
            for x in 0..3 {
                write!(f, "|{}", tmp[x + y * 3]).expect("Impossible");
            }
            write!(f, "|\n").expect("Impossible");
        }

        ret
    }
}

impl Into<[u8; 4]> for &TicField {
    fn into(self) -> [u8; 4] {
        self.bits.to_ne_bytes()
    }
}

impl From<[u8; 4]> for TicField {
    fn from(value: [u8; 4]) -> Self {
        TicField {
            bits: u32::from_ne_bytes(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tic_field_init() {
        let field = TicField::new();
        for i in 0..9 {
            assert_eq!(field.is_field_set(i), false);
        }
    }

    #[test]
    fn test_tic_field_set() {
        let mut field = TicField::new();
        field.set_field(0, Player::O).expect("Field already set");
        for i in 1..9 {
            assert_eq!(field.is_field_set(i), false);
        }
        assert_eq!(field.is_field_set(0), true);
        assert_eq!(field.get_player_for_field(0), Player::O);
    }
}
