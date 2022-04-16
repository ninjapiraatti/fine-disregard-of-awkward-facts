#[derive(Clone, Copy, Debug)]
pub struct Glyph {
	pub ch: char,
	pub color: termion::color::Rgb,
	pub permissions: usize,
	pub z: u16,
}

#[derive(Clone, Debug)]
pub struct Program {
	pub changes: Vec<(usize, usize)>,
	pub screen: Vec<Vec<Glyph>>,
	pub width: usize,
	pub height: usize, 
	pub dot: Glyph,
	pub debugstr: String,
	pub debugint: i32
}

impl Program {
	pub fn update_program(&mut self) {
		self.changes = vec![(0,0)];
	}
}

pub fn init_program(x: usize, y: usize) -> Program {
	let dot = Glyph {ch: '.', color: termion::color::Rgb(15, 15, 15), permissions: 0, z: 0};
	Program {
		dot,
		changes: vec![(0, 0)],
		screen: vec![vec![dot; x as usize]; y as usize],
		width: x,
		height: y,
		debugstr: "Henlo".to_string(),
		debugint: 0,
	}
}