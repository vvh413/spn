pub struct SPN {
    rounds: usize,
}

impl SPN {
    pub const S: [u16; 8] = [4, 0, 7, 3, 5, 1, 2, 6];
    const I_S: [u16; 8] = [1, 5, 6, 3, 0, 4, 7, 2];

    pub fn new(rounds: usize) -> Self {
        Self { rounds }
    }

    fn xkey(data: u16, key: u16) -> u16 {
        (data ^ key) & 0x1ff
    }

    fn round_enc(data: u16, key: u16, last: bool) -> u16 {
        let mut result = data;
        result = Self::xkey(result, key);
        result = Self::s(result, false);
        if last {
            Self::xkey(result, key)
        } else {
            Self::p(result)
        }
    }

    fn round_dec(data: u16, key: u16, first: bool) -> u16 {
        let mut result = data;
        result = if first {
            Self::xkey(result, key)
        } else {
            Self::p(result)
        };
        result = Self::s(result, true);
        Self::xkey(result, key)
    }

    fn s(data: u16, inv: bool) -> u16 {
        let mut result: u16 = data;
        for i in 0..3 {
            result = Self::s_i(i, result, inv);
        }
        result
    }

    fn s_i(i: usize, data: u16, inv: bool) -> u16 {
        let s_block = if inv { Self::I_S } else { Self::S };
        let shift = i * 3;
        let xi = (data >> shift) & 0x7;
        (data & !(0x7 << shift)) | ((s_block[xi as usize] & 0x7) << shift)
    }

    fn p(data: u16) -> u16 {
        let mut result: u16 = 0;
        for i in 0..9 as usize {
            let shift = (i % 3) * 3 + (i / 3);
            result |= ((data >> i) & 0x1) << shift;
        }
        result
    }

    pub fn encode(&self, data: u16, key: u16) -> u16 {
        let mut result = data;
        for round in 0..self.rounds {
            result = Self::round_enc(result, key, round == self.rounds - 1);
        }
        result
    }

    pub fn decode(&self, data: u16, key: u16) -> u16 {
        let mut result = data;
        for round in 0..self.rounds {
            result = Self::round_dec(result, key, round == 0)
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn test_s_0() {
        assert_eq!(SPN::s_i(0, 0b001, false), 0b000);
        assert_eq!(SPN::s_i(0, 0b011, false), 0b011);
        assert_eq!(SPN::s_i(0, 0b101, false), 0b001);
        assert_eq!(SPN::s_i(0, 0b111, false), 0b110);

        assert_eq!(SPN::s_i(0, 0b100, true), 0b000);
        assert_eq!(SPN::s_i(0, 0b111, true), 0b010);
        assert_eq!(SPN::s_i(0, 0b101, true), 0b100);
        assert_eq!(SPN::s_i(0, 0b010, true), 0b110);
    }

    #[test]
    fn test_s_full() {
        assert_eq!(SPN::s(0b110000011, false), 0b010100011);
        assert_eq!(SPN::s(0b010001001, true), 0b110101101);
        assert_eq!(SPN::s(SPN::s(0b110101010, false), true), 0b110101010);

        for _ in 0..100 {
            let x = thread_rng().gen_range(0..=0x1ff);
            assert_eq!(SPN::s(SPN::s(x, false), true), x);
            assert_eq!(SPN::s(SPN::s(x, true), false), x);
        }
    }

    #[test]
    fn test_p() {
        assert_eq!(SPN::p(0b100010110), 0b101011000);
        assert_eq!(SPN::p(0b101011000), 0b100010110);
        assert_eq!(SPN::p(SPN::p(0b100010110)), 0b100010110);

        for _ in 0..100 {
            let x = thread_rng().gen_range(0..=0x1ff);
            assert_eq!(SPN::p(SPN::p(x)), x);
        }
    }
}
