use crate::{combinator::RgBindMode, extend::Case, util, Mode, Rg};
use std::ops::RangeInclusive;

pub struct Json {
    items: RangeInclusive<u32>,
    numeric_rg: RangeInclusive<u32>,
    string_rg: RangeInclusive<u32>,
    array_rg: RangeInclusive<u32>,
    float_int_rg: RangeInclusive<u32>,
    float_rg: RangeInclusive<u32>,
    string_case: Case,
    level: u32,
    max_level: u32,
}

impl Json {
    pub const fn new() -> Self {
        Self {
            items: 10..=20,
            numeric_rg: 2..=6,
            string_case: Case::Mixed,
            string_rg: 3..=6,
            array_rg: 0..=8,
            level: 0,
            max_level: 3,
            float_int_rg: 2..=5,
            float_rg: 1..=3,
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
        Rg::new().numeric_with_buf(buf, self.numeric_rg.clone(), true, true)
    }

    /// Atomic node
    fn string(&self, buf: String) -> String {
        Rg::with_dec("\"", "\"").word_with_buf(buf, self.string_rg.clone(), self.string_case)
    }

    /// Atomic node
    fn boolean(&self, buf: String) -> String {
        Rg::new().boolean_with_buf(buf)
    }

    /// Atomic node
    fn float(&self, buf: String) -> String {
        Rg::new().float_with_buf(buf, self.float_int_rg.clone(), self.float_rg.clone(), true)
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
            let choice = util::rand_range(1..=4);

            match choice {
                1 => self.string(buf),
                2 => self.numeric(buf),
                3 => self.boolean(buf),
                4 => self.float(buf),
                _ => unreachable!(),
            }
        } else {
            let choice = util::rand_range(1..=100);

            match choice {
                1..=20 => self.string(buf),
                21..=40 => self.numeric(buf),
                41..=60 => self.boolean(buf),
                61..=80 => self.boolean(buf),
                81..=90 => self.json_obj(buf),
                91..=100 => self.array(buf),
                _ => unreachable!(),
            }
        }
    }
}

impl Default for Json {
    fn default() -> Self {
        Self::new()
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
