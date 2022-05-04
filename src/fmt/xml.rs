// use super::LevelPrinter;

// pub struct Xml {
// 	level: LevelPrinter,
// 	string_rg: RangeInclusive<u32>,
// }

// impl Xml {
// 	pub const fn new() -> Self {
// 		Self {
// 			string_rg: 3..=6,
// 			level: LevelPrinter::new("  "),
// 		}
// 	}

// 	pub fn tag(&mut self, buf: &mut String)  {
// 		self.level.upgrade();
// 		buf.push_str("<");

// 		buf.push_str("/>");
// 		self.level.downgrade();
// 	}

// }
