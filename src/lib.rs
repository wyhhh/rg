extern crate alloc;
extern crate rand;

use enum_len::EnumLen;
use rand::thread_rng;
use rand::Rng;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::ops::RangeInclusive;

pub mod combinator;
mod data;
pub mod extend;
pub mod fmt;
mod macros;
mod util;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Others {
    Lowers(RangeInclusive<u32>),
    Uppers(RangeInclusive<u32>),
    LowersAndUppers(RangeInclusive<u32>),
    Digits(RangeInclusive<u32>),
    DigitsNonZero(RangeInclusive<u32>),
}

#[derive(EnumLen, Debug, Clone, PartialEq, Eq)]
pub enum Mode<'a, S> {
    // ---------Borrowed-----------
    Noun,
    Verb,
    Pred,
    Adj,
    Adverb,
    // 	主谓宾subject + verb + object
    SVO,
    // 主系表subject + link verb + Predicative
    SLP,
    // A = Adverb 状语
    Diy(&'a [S]),
    Others(Others),
    ASVO(S),
    SVOA(S),
    ASLP(S),
    SLPA(S),
    /// **must be last, for working properly**
    Rand,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Rg<'a> {
    left_dec: Option<&'a str>,
    right_dec: Option<&'a str>,
}

pub struct Iter<'a, S> {
    rg: &'a Rg<'a>,
    mode: &'a Mode<'a, S>,
}

impl<'a, S: AsRef<str>> Iterator for Iter<'a, S> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rg.once(self.mode))
    }
}

impl<'a, S: AsRef<str>> Iter<'a, S> {
    fn new(rg: &'a Rg<'a>, mode: &'a Mode<'a, S>) -> Self {
        Self { rg, mode }
    }
}

impl<'a> Rg<'a> {
    pub fn new() -> Self {
        Self {
            left_dec: None,
            right_dec: None,
        }
    }

    pub fn with_dec(l: &'a str, r: &'a str) -> Self {
        Self {
            left_dec: Some(l),
            right_dec: Some(r),
        }
    }

    pub fn iter<S: AsRef<str>>(&'a self, mode: &'a Mode<'a, S>) -> Iter<'a, S> {
        Iter::new(self, mode)
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn left_dec(&mut self, d: &'a str) -> &mut Self {
        self.left_dec = Some(d);
        self
    }

    pub fn right_dec(&mut self, d: &'a str) -> &mut Self {
        self.right_dec = Some(d);
        self
    }

    pub(crate) fn push_left(&mut self, buf: &mut String, push_dec: bool) {
        if push_dec {
            if let Some(l) = self.left_dec {
                buf.push_str(l);
            }
        }
    }
    pub(crate) fn push_right(&mut self, buf: &mut String, push_dec: bool) {
        if push_dec {
            if let Some(r) = self.right_dec {
                buf.push_str(r);
            }
        }
    }

    pub fn combine<'b, S: AsRef<str> + 'b, M: Borrow<Mode<'b, S>>>(
        &mut self,
        modes: &[M],
        seps: &[S],
    ) -> String {
        self.combine_with_buf(String::new(), modes, seps)
    }

    pub fn combine_with_buf<'b, S: AsRef<str> + 'b, M: Borrow<Mode<'b, S>>>(
        &mut self,
        mut buf: String,
        modes: &[M],
        seps: &[S],
    ) -> String {
        self.push_left(&mut buf, true);
        for (i, mode) in modes.iter().enumerate() {
            let _res = self.core(mode.borrow(), &mut buf, true, false);

            if !seps.is_empty() {
                let idx = if i < seps.len() { i } else { seps.len() - 1 };
                buf.push_str(unsafe { seps.get_unchecked(idx).as_ref() });
            }
        }
        self.push_right(&mut buf, true);

        buf
    }

    pub fn once<'b, S: AsRef<str> + 'b, M: Borrow<Mode<'b, S>> + 'b>(
        &self,
        mode: M,
    ) -> Cow<'b, str> {
        let mut buf = String::new();
        let ret = self.core(mode.borrow(), &mut buf, false, true);

        if buf.is_empty() {
            Cow::Borrowed(ret.unwrap())
        } else {
            debug_assert!(ret.is_none(), "{:?}", ret);
            Cow::Owned(buf)
        }
    }

    fn has_dec(&self) -> bool {
        self.left_dec.is_some() || self.right_dec.is_some()
    }

    fn core<'b, S: AsRef<str>>(
        &self,
        mode: &Mode<'b, S>,
        buf: &mut String,
        push_buf: bool,
        push_dec: bool,
    ) -> Option<&'b str> {
        if push_dec {
            if let Some(d) = self.left_dec {
                buf.push_str(d);
            }
        }

        let ret = match mode {
            Mode::Others(name) => {
                self.push_others(buf, name);
                None
            }
            Mode::SVO => {
                self.push_svo(buf);
                None
            }
            Mode::SLP => {
                self.push_slp(buf);
                None
            }
            Mode::ASVO(s) => {
                self.push_asvo(buf, s.as_ref());
                None
            }
            Mode::SVOA(s) => {
                self.push_svoa(buf, s.as_ref());
                None
            }
            Mode::ASLP(s) => {
                self.push_aslp(buf, s.as_ref());
                None
            }
            Mode::SLPA(s) => {
                self.push_slpa(buf, s.as_ref());
                None
            }
            Mode::Rand => self.rand_mode(buf, mode),
            _ => {
                let res = match mode {
                    Mode::Noun => self.get_noun(),
                    Mode::Verb => self.get_verb(),
                    Mode::Pred => self.get_pred(),
                    Mode::Adj => self.get_adj(),
                    Mode::Adverb => self.get_adverb(),
                    Mode::Diy(s) => self.get_diy(s).as_ref(),
                    _ => unreachable!(),
                };

                if self.has_dec() || push_buf {
                    buf.push_str(res);
                    None
                } else {
                    Some(res)
                }
            }
        };

        if push_dec {
            if let Some(d) = self.right_dec {
                buf.push_str(d);
            }
        }

        if push_buf {
            debug_assert!(ret.is_none());
        }
        ret
    }

    fn rand_mode<'b, S2: AsRef<str>>(&self, buf: &mut String, _: &Mode<'b, S2>) -> Option<&'b str> {
        let idx = thread_rng().gen_range(0..ENUM_LEN as u8 - 7);

        let rmode: &Mode<'_, &str> = match idx {
            0 => &Mode::Noun,
            1 => &Mode::Verb,
            2 => &Mode::Pred,
            3 => &Mode::Adj,
            4 => &Mode::Adverb,
            5 => &Mode::SVO,
            6 => &Mode::SLP,
            _ => unreachable!(),
        };

        self.core(rmode, buf, false, false)
    }

    fn get_diy<'b, S2: AsRef<str>>(&self, s: &'b [S2]) -> &'b S2 {
        util::rand_slice(s)
    }

    fn get_pred(&self) -> &'static str {
        *util::rand_slice(data::PREDS)
    }

    fn get_adverb(&self) -> &'static str {
        *util::rand_slice(data::ADVERBS)
    }

    fn get_adj(&self) -> &'static str {
        *util::rand_slice(data::adjs())
    }

    fn get_noun(&self) -> &'static str {
        *util::rand_slice(data::nouns())
    }

    fn get_verb(&self) -> &'static str {
        *util::rand_slice(data::VERBS)
    }

    fn push_svo(&self, buf: &mut String) {
        buf.push_str(*util::rand_slice(data::nouns()));
        buf.push_str(*util::rand_slice(data::VERBS));
        buf.push_str(*util::rand_slice(data::nouns()));
    }

    fn push_slp(&self, buf: &mut String) {
        buf.push_str(*util::rand_slice(data::nouns()));
        buf.push_str(*util::rand_slice(data::LINKS));
        buf.push_str(*util::rand_slice(data::PREDS));
    }

    fn push_asvo(&self, buf: &mut String, sep: &str) {
        buf.push_str(self.get_adverb());
        buf.push_str(sep);
        self.push_svo(buf);
    }

    fn push_svoa(&self, buf: &mut String, sep: &str) {
        self.push_svo(buf);
        buf.push_str(sep);
        buf.push_str(self.get_adverb());
    }

    // ASLP(&'a str),
    fn push_aslp(&self, buf: &mut String, sep: &str) {
        buf.push_str(self.get_adverb());
        buf.push_str(sep);
        self.push_slp(buf);
    }

    // SLPA(&'a str),
    fn push_slpa(&self, buf: &mut String, sep: &str) {
        self.push_slp(buf);
        buf.push_str(sep);
        buf.push_str(self.get_adverb());
    }

    fn push_others(&self, buf: &mut String, others: &Others) {
        macro_rules! loop_n {
            ($s:expr,$rg: expr) => {{
                let cnt = util::rand_range($rg.clone());

                for _ in 0..cnt {
                    let x = util::rand_slice($s);
                    buf.push(*x as char);
                }
            }};
        }

        match others {
            Others::Lowers(rg) => loop_n!(b"qwertyuiopasdfghjklzxcvbnm", rg),
            Others::Uppers(rg) => loop_n!(b"QWERTYUIOPASDFGHJKLZXCVBNM", rg),
            Others::Digits(rg) => loop_n!(b"0123456789", rg),
            Others::DigitsNonZero(rg) => loop_n!(b"123456789", rg),
            Others::LowersAndUppers(rg) => {
                loop_n!(b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM", rg)
            }
        }
    }
}
