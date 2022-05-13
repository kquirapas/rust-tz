use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();

	for (pos, arg) in args.iter().enumerate() {
		if pos == 1 {
			print!("Enter GMT+8 in 24:00 time: ");
			io::stdout().flush().unwrap();

			let mut guess: String = String::new();

			io::stdin()
				.read_line(&mut guess)
				.expect("Failed to read line");

			let _base = guess.trim_end().parse::<i32>().unwrap();

			let f: fs::File = fs::File::open(arg).unwrap();
			let reader = BufReader::new(f);
			
			println!("[==24:00 FORMAT==]");
			for l in reader.lines() {
				let line = l.unwrap();
				let words = line.split_whitespace().take(2).collect::<Vec<&str>>();

				if let [name, offset] = &words[..] {
					let offset_float = offset.trim_end().parse::<f32>().unwrap() / 60.0;
					let result: f32 = _base as f32 + offset_float;
					let hours: i32 = result as i32;
					let minutes: f32 = (result - hours as f32) * 60.0;

					println!("{:0>2}:{:0<2}{:>10}", hours, minutes, name);
				}
			}
		}
	}
	
	// let contents = fs::read_to_string();
}
