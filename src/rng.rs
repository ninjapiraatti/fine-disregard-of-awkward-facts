use std::sync::Mutex;
lazy_static::lazy_static! {
	static ref RNG :Mutex<RandGen> = Mutex::new(RandGen::new(34000));
}

pub fn rng(max: usize) -> usize {
	RNG.lock().unwrap().next_v(max)
}
pub struct RandGen {
	curr: usize,
	mul: usize,
	inc: usize,
	modulo: usize 
}

impl RandGen {
	pub fn new(curr: usize) -> Self {
		RandGen {
			curr,
			mul: 56394237,
			inc: 346423491,
			modulo: 23254544563,
		}
	}

	pub fn next_v(&mut self, max:usize) -> usize {
		self.curr = (self.curr * self.mul + self.inc) % self.modulo;
		self.curr % max 
	}
}