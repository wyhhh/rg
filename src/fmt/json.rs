use crate::{
    combinator::{Generator, RgBindMode},
    extend::Case,
    util, Mode, Rg,
};
use rand::{thread_rng, Rng};
use std::ops::RangeInclusive;

pub struct Json {
    items: RangeInclusive<u32>,
    numeric_rg: RangeInclusive<u32>,
    string_rg: RangeInclusive<u32>,
    array_rg: RangeInclusive<u32>,
    string_case: Case,
    level: u32,
    max_level: u32,
}

impl Json {
    pub const fn new() -> Self {
        Self {
            items: 0..=20,
            numeric_rg: 2..=6,
            string_case: Case::Mixed,
            string_rg: 3..=6,
            array_rg: 0..=8,
            level: 0,
            max_level: 5,
        }
    }

    pub fn field_cnt(mut self, rg: RangeInclusive<u32>) -> Self {
        self.items = rg;
        self
    }

    pub fn numeric_rg(mut self, rg: RangeInclusive<u32>) -> Self {
        self.numeric_rg = rg;
        self
    }

    pub fn string_rg(mut self, rg: RangeInclusive<u32>) -> Self {
        self.string_rg = rg;
        self
    }

    pub fn string_case(mut self, case: Case) -> Self {
        self.string_case = case;
        self
    }

    pub fn array_rg(mut self, rg: RangeInclusive<u32>) -> Self {
        self.array_rg = rg;
        self
    }
	   
    pub fn max_level(mut self, max_level: u32) -> Self {
        self.max_level = max_level;
        self
    }

    pub fn generate(&mut self) -> String {
        self.json_obj(String::new())
    }

    fn json_obj(&mut self, mut buf: String) -> String {
        self.level += 1;
        buf.push_str("{\n");

        let mut ln = "";
        for _ in 0..util::rand_range(self.items.clone()) {
            buf.push_str(ln);
            for _ in 0..self.level {
                buf.push_str("  ");
            }
            buf = self.item(buf);
            ln = ",\n";
        }

        buf.push('\n');
        for _ in 0..self.level - 1 {
            buf.push_str("  ");
        }
        buf.push('}');
        self.level -= 1;

        buf
    }

    fn item(&mut self, mut buf: String) -> String {
        buf = self.string(buf);
        buf.push_str(": ");
        // println!("{:?}", buf);
        self.choose(buf)
    }

    /// Atomic node
    fn numeric(&self, buf: String) -> String {
        Rg::new().numeric_with_buf(buf, self.numeric_rg.clone(), true)
    }

    /// Atomic node
    fn string(&self, buf: String) -> String {
        Rg::with_dec("\"", "\"").word_with_buf(buf, self.string_rg.clone(), self.string_case)
    }

    fn array(&mut self, mut buf: String) -> String {
        self.level += 1;
        buf.push_str("[\n");
        let len = util::rand_range(self.array_rg.clone());
        let mut comma = "";

        for _ in 0..len {
            buf.push_str(comma);
            for _ in 0..self.level {
                buf.push_str("  ");
            }
            buf = self.choose(buf);
            comma = ",\n";
        }

        buf.push('\n');
        for _ in 0..self.level - 1 {
            buf.push_str("  ");
        }
        buf.push(']');
        self.level -= 1;
        buf
    }

    fn choose(&mut self, buf: String) -> String {
        if self.level > self.max_level {
            if util::rand_or() {
                self.string(buf)
            } else {
                self.numeric(buf)
            }
        } else {
            let choice = util::rand_range(1..=100);

            match choice {
                1..=45 => self.string(buf),
                46..=90 => self.numeric(buf),
                91..=95 => self.json_obj(buf),
                96..=100 => self.array(buf),
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Json;

    #[test]
    fn test() {
        let mut json = Json::new();
        let res = json.generate();
        println!("{}", res);
    }
}
