extern crate termion;
use fdoaf::rng;
use termion::{color, async_stdin, cursor, style, get_tty};
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout}; // Add stdin if you need to switch away from async_stdin
use std::thread;
use std::fs;
use fdoaf::{self, program::Program}; // That is the name of the library of this program
use fdoaf::program;

// The UI state.
pub struct UI<R, W> {
	width: usize,
	height: usize,
	/// Standard input.
	stdin: R,
	/// Standard output.
	stdout: W,
}

impl <R: Read, W: Write> UI<R, W> {
	fn run(&mut self, program: &mut Program) {
		write!(self.stdout, "{}", cursor::Hide).unwrap();
		self.reset(program);
		loop {
			if !self.update(program) {
				return;
      }
			write!(self.stdout, "{}", style::Reset).unwrap();
      self.stdout.flush().unwrap();
		}
	}

	fn draw_debug(&mut self, program: &mut Program) {
		write!(self.stdout, "{}{}{}{:?} {:?}\n{:?}\n{}", 
		termion::color::Fg(color::Rgb(50,50,50)),
		termion::color::Bg(color::Rgb(1,5,5)), 
		cursor::Goto(2, 2 as u16),
		program.changes.len(),
		program.debugstr,
		program.debugint,
		termion::color::Bg(color::Reset))
		.unwrap();
	}

	fn draw_map(&mut self, program: &mut Program) {
		program.update_program();
		for val in 0..program.changes.len() {
			let x = program.changes[val].0;
			let y = program.changes[val].1;
			//program.debugstr = format!("Drawing {}", program.map[y][x].ch);
		}
	}

	fn draw_character(&mut self, chr: char, color: termion::color::Rgb, x: u16, y: u16) {
		write!(self.stdout, "{}{}{}{}{}", 
		termion::color::Fg(color),
		termion::color::Bg(color::Rgb(1,5,5)), 
		cursor::Goto(x, y as u16),
		chr,
		termion::color::Bg(color::Reset))
		.unwrap();
	}

	fn reset(&mut self, program: &mut Program) {
		write!(self.stdout,
			"{}{}{}",
			termion::clear::All,
			termion::cursor::Goto(1, 1),
			termion::color::Fg(color::Rgb(5,25,25)))
			.unwrap();
		self.stdout.flush().unwrap();
		for y in 0..self.height {
			for x in 0..self.width {
				self.draw_character(program.screen[y][x].ch, program.screen[y][x].color, (x + 1) as u16, (y + 1) as u16);
			}
		}
	}

	fn update(&mut self, program: &mut Program) -> bool{
		let mut key_bytes = [0];
    self.stdin.read(&mut key_bytes).unwrap();
		//println!("get_tty: {:?}", get_tty().unwrap().bytes());
		let data = fs::read_to_string("test.txt").expect("Unable to read file");
    println!("{}", data);
		match key_bytes[0] {
			b'q' => return false,
			b'l' | b'x' => {println!("L or X");},
			b'o' => {println!("O");},
			b'a' => {println!("A");},
			b't' => {println!("T");},
			b'h' => {println!("H");},
			_ => {},
		}

		self.draw_map(program);
		self.draw_debug(program);
		let delay = std::time::Duration::from_millis(30); // The player should update fast and the rest of the program slow. How to do this?
		thread::sleep(delay);
		true
	}
}

fn run(width: usize, height: usize, program: &mut Program) {
	let stdout = stdout();
	let stdout = stdout.lock().into_raw_mode().unwrap();
	let stdin = async_stdin();
	//let stdin = stdin.lock(); // Bring this back in if you change back to synced stdin
	let mut ui = UI {
		width,
		height,
		stdin,
		stdout,
	};
	// TODO: separate UI from game logic
	ui.reset(program);
	ui.run(program);
}

fn main() {
	let size: (u16, u16) = termion::terminal_size().unwrap();
	let mut program = program::init_program(size.0 as usize, size.1 as usize);
	run(size.0 as usize, size.1 as usize, &mut program);
}

