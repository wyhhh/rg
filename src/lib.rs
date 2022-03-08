use enum_len::EnumLen;
use rand::thread_rng;
use rand::Rng;
use std::borrow::Cow;
use std::ops::RangeInclusive;

mod data;
mod macros;
mod util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// letter + digit
pub enum NameKind {
    Lowers,
    Uppers,
    Digits,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    kind: NameKind,
    range: RangeInclusive<u32>,
}

#[derive(EnumLen, Debug, Clone, PartialEq, Eq)]
pub enum Mode<'a, S: AsRef<str>> {
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
    Name(Name),
    ASVO(&'a str),
    SVOA(&'a str),
    ASLP(&'a str),
    SLPA(&'a str),
    /// **must be last, for working properly**
    Rand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Rg<'a> {
    left_dec: Option<&'a str>,
    right_dec: Option<&'a str>,
}

pub struct Iter<'a, S: AsRef<str>> {
    rg: &'a Rg<'a>,
    mode: &'a Mode<'a, S>,
}

impl<'a, S: AsRef<str>> Iterator for Iter<'a, S> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rg.generate(self.mode))
    }
}

impl<'a, S: AsRef<str>> Iter<'a, S> {
    fn new(rg: &'a Rg<'a>, mode: &'a Mode<'a, S>) -> Self {
        Self { rg, mode }
    }
}

impl Name {
    pub fn new(kind: NameKind, range: RangeInclusive<u32>) -> Self {
        Self { kind, range }
    }
}

impl<'a> Rg<'a> {
    pub fn new() -> Self {
        Self {
            left_dec: None,
            right_dec: None,
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

    pub fn combine<'b, S: AsRef<str>>(&mut self, modes: &[&Mode<'b, S>], seps: &[S]) -> String {
        let mut buf = String::new();

        for (i, mode) in modes.iter().enumerate() {
            let _res = self.core(mode, &mut buf, true, true);

            if !seps.is_empty() {
                let idx = if i < seps.len() { i } else { seps.len() - 1 };
                buf.push_str(unsafe { seps.get_unchecked(idx).as_ref() });
            }
        }

        buf
    }

    pub fn generate<'b, S: AsRef<str>>(&self, mode: &Mode<'b, S>) -> Cow<'b, str> {
        let mut buf = String::new();
        let ret = self.core(mode, &mut buf, false, true);

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
            Mode::Name(name) => {
                self.push_name(buf, name);
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
                self.push_asvo(buf, s);
                None
            }
            Mode::SVOA(s) => {
                self.push_svoa(buf, s);
                None
            }
            Mode::ASLP(s) => {
                self.push_aslp(buf, s);
                None
            }
            Mode::SLPA(s) => {
                self.push_slpa(buf, s);
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
        ret
    }

    fn rand_mode<'b, S: AsRef<str>>(&self, buf: &mut String, _: &Mode<'b, S>) -> Option<&'b str> {
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

    fn get_diy<'b, S: AsRef<str>>(&self, s: &'b [S]) -> &'b S {
        util::slice_rgen(s)
    }

    fn get_pred(&self) -> &'static str {
        *util::slice_rgen(data::PREDS)
    }

    fn get_adverb(&self) -> &'static str {
        *util::slice_rgen(data::ADVERBS)
    }

    fn get_adj(&self) -> &'static str {
        *util::slice_rgen(data::adjs())
    }

    fn get_noun(&self) -> &'static str {
        *util::slice_rgen(data::nouns())
    }

    fn get_verb(&self) -> &'static str {
        *util::slice_rgen(data::VERBS)
    }

    fn push_svo(&self, buf: &mut String) {
        buf.push_str(*util::slice_rgen(data::nouns()));
        buf.push_str(*util::slice_rgen(data::VERBS));
        buf.push_str(*util::slice_rgen(data::nouns()));
    }

    fn push_slp(&self, buf: &mut String) {
        buf.push_str(*util::slice_rgen(data::nouns()));
        buf.push_str(*util::slice_rgen(data::LINKS));
        buf.push_str(*util::slice_rgen(data::PREDS));
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

    fn push_name(&self, buf: &mut String, Name { kind, range }: &Name) {
        let cnt = thread_rng().gen_range(range.clone());

        macro_rules! loop_n {
            ($s:expr) => {
                for _ in 0..cnt {
                    let x = util::slice_rgen($s);
                    buf.push(*x as char);
                }
            };
        }

        match kind {
            NameKind::Lowers => loop_n!(b"qwertyuiopasdfghjklzxcvbnm"),
            NameKind::Uppers => loop_n!(b"QWERTYUIOPASDFGHJKLZXCVBNM"),
            NameKind::Digits => loop_n!(b"0123456789"),
        }
    }
}
