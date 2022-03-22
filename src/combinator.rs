use crate::util;
use crate::Iter;
use crate::Mode;
use crate::Rg;
use core::fmt;
use rand::thread_rng;
use rand::Rng;
use std::marker::PhantomData;

pub trait Generator {
    fn once(&mut self, buf: String) -> String;

    /// Init generate for first time
    fn generate(mut self) -> String
    where
        Self: Sized,
    {
        self.once(String::new())
    }

    fn generate_by(mut self, buf: String) -> String
    where
        Self: Sized,
    {
        self.once(buf)
    }

    fn and<G: Generator>(self, g: G) -> And<Self, G>
    where
        Self: Sized,
    {
        And {
            me: self,
            another: g,
        }
    }

    fn or<G: Generator>(self, g: G) -> OrBy<Self, G, fn() -> bool>
    where
        Self: Sized,
    {
        OrBy {
            me: self,
            another: g,
            f: util::rand_or,
        }
    }

    fn or_by<G: Generator, F>(self, g: G, f: F) -> OrBy<Self, G, F>
    where
        F: FnMut() -> bool,
        Self: Sized,
    {
        OrBy {
            me: self,
            another: g,
            f,
        }
    }

    fn repeat(self, times: u32) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat { g: self, times }
    }

    fn tail(self, tail: &str) -> Tail<'_, Self>
    where
        Self: Sized,
    {
        Tail { g: self, tail }
    }

    fn map<F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(String) -> String,
    {
        Map { g: self, f }
    }

    fn check(self) -> Check<Self>
    where
        Self: Sized,
    {
        Check { g: self }
    }
}

pub fn select_by<'a, F>(generators: &'a mut [&'a mut dyn Generator], f: F) -> SelectBy<'a, F>
where
    F: FnMut(usize) -> usize,
{
    SelectBy { generators, f }
}

pub fn select<'a>(generators: &'a mut [&'a mut dyn Generator]) -> SelectBy<'a, fn(usize) -> usize> {
    SelectBy {
        generators,
        f: |len| thread_rng().gen_range(0..len),
    }
}
pub struct And<G, G2> {
    me: G,
    another: G2,
}

impl<G, G2> Generator for And<G, G2>
where
    G: Generator,
    G2: Generator,
{
    fn once(&mut self, buf: String) -> String {
        self.another.once(self.me.once(buf))
    }
}

pub struct OrBy<G, G2, F> {
    me: G,
    another: G2,
    f: F,
}

impl<G, G2, F> Generator for OrBy<G, G2, F>
where
    G: Generator,
    G2: Generator,
    F: FnMut() -> bool,
{
    fn once(&mut self, buf: String) -> String {
        if (self.f)() {
            self.me.once(buf)
        } else {
            self.another.once(buf)
        }
    }
}

pub struct Map<G, F> {
    g: G,
    f: F,
}

impl<G, F> Generator for Map<G, F>
where
    G: Generator,
    F: FnMut(String) -> String,
{
    fn once(&mut self, buf: String) -> String {
        (self.f)(self.g.once(buf))
    }
}

pub struct Repeat<G> {
    g: G,
    times: u32,
}

impl<G: Generator> Generator for Repeat<G> {
    fn once(&mut self, mut buf: String) -> String {
        for _ in 0..self.times {
            buf = self.g.once(buf);
        }
        buf
    }
}

pub struct Tail<'a, G> {
    g: G,
    tail: &'a str,
}

impl<G: Generator> Generator for Tail<'_, G> {
    fn once(&mut self, mut buf: String) -> String {
        buf = self.g.once(buf);
        buf.push_str(self.tail);
        buf
    }
}

pub struct Check<G> {
    g: G,
}

impl<G> Generator for Check<G>
where
    G: Generator + fmt::Debug,
{
    fn once(&mut self, buf: String) -> String {
        dbg!(buf)
    }
}

pub struct SelectBy<'a, F> {
    generators: &'a mut [&'a mut dyn Generator],
    f: F,
}

impl<'a, F> Generator for SelectBy<'a, F>
where
    F: FnMut(usize) -> usize,
{
    fn once(&mut self, buf: String) -> String {
        if self.generators.is_empty() {
            return buf;
        }
        let idx = (self.f)(self.generators.len());
        debug_assert!(idx < self.generators.len());

        unsafe { self.generators.get_unchecked_mut(idx).once(buf) }
    }
}

pub struct RgBindMode<'a, S> {
    rg: Rg<'a>,
    mode: Mode<'a, S>,
}

impl<'a, S: AsRef<str>> Generator for RgBindMode<'a, S> {
    fn once(&mut self, mut buf: String) -> String {
        let _res = self.rg.core(&self.mode, &mut buf, true, true);
        buf
    }
}

impl<'a, S: AsRef<str>> RgBindMode<'a, S> {
    pub fn new(mode: Mode<'a, S>) -> Self {
        Self {
            rg: Rg::new(),
            mode,
        }
    }

    pub fn with_dec(
        left_dec: Option<&'a str>,
        right_dec: Option<&'a str>,
        mode: Mode<'a, S>,
    ) -> Self {
        Self {
            rg: Rg {
                left_dec,
                right_dec,
            },
            mode,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Generator;
    use super::RgBindMode;
    use crate::combinator::select;
    use crate::Mode;
    use crate::Others;

    #[test]
    fn test() {
        let g: RgBindMode<&str> = RgBindMode::new(Mode::Noun);
        let g = g.and(RgBindMode::<&str>::new(Mode::Verb));
        let g = g.or(RgBindMode::<&str>::new(Mode::Others(Others::Digits(1..=3))));
        let g = g.map(|mut s| {
            s.push_str("~~");
            s
        });
        let g = g.tail("**");
        let mut g = g.repeat(3);
        let mut g2 = RgBindMode::<&str>::new(Mode::Noun);
        let arr: &mut [&mut dyn Generator] = &mut [&mut g, &mut g2];
        let g = select(arr);

        println!("{:?}", g.generate());
    }
}
