//
//      MACROS
//
#[macro_export]
macro_rules! bmask {
    ($bin:expr) => {
        BitMask::from(String::from($bin))
    };
}

#[derive(Debug)]
pub struct BitMask<const BLEN: usize> {
    mask: [u8; BLEN],
}

impl<const BLEN: usize> BitMask<BLEN> {
    pub fn count_ones(&self) -> usize {
        let mut total = 0;
        for i in self.mask {
            total += i.count_ones();
        }
        total as usize
    }

    pub fn first_mask_byte(&self) -> u8 {
        self.mask[0]
    }
}

impl<const BLEN: usize> From<String> for BitMask<BLEN> {
    fn from(x: String) -> BitMask<BLEN> {
        let mut end: [u8; BLEN] = [0; BLEN];
        let c = Vec::from_iter(x.chars().enumerate());
        let split = Vec::from_iter(c.split(|x| x.0 % 8 == 0));

        let mut final_bytes: Vec<u8> = vec![];

        for i in split {
            let chars = Vec::from_iter(i.iter().map(|x| x.1));
            let byte: u8 = chars
                .iter()
                .fold(0u8, |acc, &c| (acc << 1) | (c == '1') as u8);
            final_bytes.push(byte);
        }

        BitMask {
            mask: final_bytes.try_into().expect(" ... "),
        }
    }
}
