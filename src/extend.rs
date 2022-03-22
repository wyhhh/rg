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
        self.numeric_with_buf(String::new(), rg, negative, true)
    }

    pub fn numeric_with_buf(
        &mut self,
        mut buf: String,
        rg: RangeInclusive<u32>,
        negative: bool,
        push_dec: bool,
    ) -> String {
        let cnt = util::rand_range(rg);
        let neg = if negative && util::rand_or() { "-" } else { "" };

        if cnt == 0 {
            buf
        } else if cnt == 1 {
            self.push_left(&mut buf, push_dec);
            let number = *util::rand_slice(b"0123456789");

            if number == 0 {
                buf.push('0');
            } else {
                buf.push_str(neg);
                buf.push(number as char);
            }

            self.push_right(&mut buf, push_dec);

            buf
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

    pub fn boolean(&self) -> String {
        self.boolean_with_buf(String::new())
    }

    pub fn boolean_with_buf(&self, mut buf: String) -> String {
        let _res = self.core(&Mode::Diy(&["true", "false"]), &mut buf, true, true);
        buf
    }

    pub fn float_with_buf(
        &mut self,
        mut buf: String,
        int_rg: RangeInclusive<u32>,
        float_rg: RangeInclusive<u32>,
        negative: bool,
    ) -> String {
        self.push_left(&mut buf, true);
        buf = self.numeric_with_buf(buf, int_rg, negative, false);
        buf.push('.');

        let _res = self.core::<&str>(
            &Mode::Others(Others::Digits(float_rg)),
            &mut buf,
            true,
            false,
        );
        self.push_right(&mut buf, true);
        buf
    }

    pub fn float(
        &mut self,
        int_rg: RangeInclusive<u32>,
        float_rg: RangeInclusive<u32>,
        negative: bool,
    ) -> String {
        self.float_with_buf(String::new(), int_rg, float_rg, negative)
    }
}

#[cfg(test)]
mod tests {
    use crate::extend::Case;
    use crate::Rg;

    #[test]
    fn numeric() {
        let mut rg = Rg::with_dec("{{", "}}");
        let res = rg.numberic(1..=1, true);
        println!("{:?}", res);
    }

    #[test]
    fn word() {
        let mut rg = Rg::with_dec("{{", "}}");
        let res = rg.word(1..=22, Case::Upper);
        println!("{:?}", res);
    }

    #[test]
    fn float() {
        let mut rg = Rg::with_dec("{{", "}}");
        let res = rg.float(1..=5, 1..=2, true);
        println!("{:?}", res);
    }
}
