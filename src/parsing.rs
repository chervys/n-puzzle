use std::fs::File;
use std::io::prelude::Read;

fn read_file(path: String, buffer: &mut String) -> std::io::Result<()> {
	let mut file: File = File::open(path).expect("Open file.");
	file.read_to_string(buffer).expect("Read to string.");
	Ok(())
}

fn _extract_puzzle(buffer: String) -> Vec<usize> {
	let mut puzzle: Vec<usize> = Vec::new();
	let mut tmp : String = String::new();

	for line in buffer.lines() {
		for c in line.chars() {
			if !tmp.is_empty() && !c.is_ascii_digit() {
				puzzle.push(tmp.parse().expect("String to usize"));
				tmp.clear();
			}
			match c {
				c if c.is_ascii_digit() => tmp.push(c),
				c if c.is_whitespace() => continue,
				'#' => break,
				_ => panic!("Parsing: Invalid char."),
			}
		}
		if !tmp.is_empty() {
			puzzle.push(tmp.parse().expect("String to usize"));
			tmp.clear();
		}
	}
	puzzle
}

fn is_valid_puzzle(puzzle: &Vec<usize>) -> bool {
	let len: usize = puzzle.len();
	if len < 5 {
		return false;
	}

	let size: usize = puzzle[0];
	if (size * size + 1) != len {
		return false;
	}

	let mut board = puzzle[1..].to_vec();
	board.sort();

	for (index, value) in board.iter().enumerate() {
		if index != *value {
			return false;
		}
	}

	dbg!(board);
	true
}

pub fn parsing(path: String) -> std::io::Result<(usize, Vec<usize>)> {
	let mut content: String = String::new();
	read_file(path, &mut content)?;

	let mut _puzzle: Vec<usize> = Vec::new();
	_puzzle = _extract_puzzle(content);

	let mut _is_valid: bool = is_valid_puzzle(&_puzzle);
	if _is_valid == false {
		panic!("Parsing: Invalid.");
	}

	Ok((_puzzle[0], _puzzle[1..].to_vec()))
}
