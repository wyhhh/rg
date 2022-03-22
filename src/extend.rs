use crate::util;
use crate::Mode;
use crate::Others;
use crate::Rg;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy)]
pub enum Case {
    Lower,
    Upper,
    Mixed,
}

impl<'a> Rg<'a> {
    pub fn numberic(&mut self, rg: RangeInclusive<u32>, negative: bool) -> String {
        self.numeric_with_buf(String::new(), rg, negative)
    }

    pub fn numeric_with_buf(
        &mut self,
        mut buf: String,
        rg: RangeInclusive<u32>,
        negative: bool,
    ) -> String {
        let cnt = util::rand_range(rg);
        let neg = if negative && util::rand_or() { "-" } else { "" };

        if cnt == 0 {
            buf
        } else if cnt == 1 {
            let number = *util::rand_slice(b"0123456789");

            if number == 0 {
                buf.push('0');
                buf
            } else {
                buf.push_str(neg);
                buf.push(number as char);
                buf
            }
        } else {
            self.combine_with_buf(
                buf,
                &[
                    Mode::Diy(&[neg]),
                    Mode::Others(Others::DigitsNonZero(1..=1)),
                    Mode::Others(Others::Digits(1..=cnt - 1)),
                ],
                &[],
            )
        }
    }

    pub fn word(&mut self, rg: RangeInclusive<u32>, case: Case) -> String {
        self.word_with_buf(String::new(), rg, case)
    }

    pub fn word_with_buf(
        &mut self,
        mut buf: String,
        rg: RangeInclusive<u32>,
        case: Case,
    ) -> String {
        let mode: Mode<&str> = match case {
            Case::Lower => Mode::Others(Others::Lowers(rg)),
            Case::Upper => Mode::Others(Others::Lowers(rg)),
            Case::Mixed => Mode::Others(Others::LowersAndUppers(rg)),
        };

        let _res = self.core(&mode, &mut buf, true, true);
        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::extend::Case;
    use crate::Rg;

    #[test]
    fn numeric() {
        let mut rg = Rg::new();
        let res = rg.numberic(1..=5, true);
        println!("{:?}", res);
    }

    #[test]
    fn word() {
        let mut rg = Rg::new();
        let res = rg.word(1..=22, Case::Upper);
        println!("{:?}", res);
    }
}
