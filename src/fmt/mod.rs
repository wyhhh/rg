pub mod json;
pub mod xml;

#[derive(Debug)]
struct LevelPrinter {
    level: i32,
    print: &'static str,
}

impl LevelPrinter {
    const fn new(print: &'static str) -> Self {
        Self { level: 0, print }
    }

    fn level(&self) -> i32 {
        self.level
    }

    fn upgrade(&mut self) {
        self.level += 1;
    }

    fn downgrade(&mut self) {
        self.level -= 1;
    }

    fn print(&mut self, buf: &mut String, delta: i32) {
        for _ in 0..self.level + delta {
            buf.push_str(self.print);
        }
    }
}
