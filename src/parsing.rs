use std::{fs::File, io::Read};

fn read_file(path: &String, buffer: &mut String) -> std::io::Result<()> {
	let mut file: File = File::open(path).expect("Open file.");
	file.read_to_string(buffer).expect("Read to string.");
	Ok(())
}

fn _extract_puzzle(buffer: String) -> Vec<usize> {
	let mut puzzle: Vec<usize> = Vec::new();
	let mut piece : String = String::new();

	for line in buffer.lines() {
		for c in line.chars() {
			if !piece.is_empty() && !c.is_ascii_digit() {
				puzzle.push(piece.parse().expect("String to usize"));
				piece.clear();
			}
			match c {
				c if c.is_ascii_digit() => piece.push(c),
				c if c.is_whitespace() => continue,
				'#' => break,
				_ => panic!("Parsing: Invalid char."),
			}
		}
		if !piece.is_empty() {
			puzzle.push(piece.parse().expect("String to usize"));
			piece.clear();
		}
	}
	puzzle
}

fn is_valid_puzzle(puzzle: &Vec<usize>) -> bool {
	let len: usize = puzzle.len();
	if len < (1 + (2 * 2)) {
		return false;
	}

	let size: usize = puzzle[0];
	if (size * size + 1) != len {
		return false;
	}

	let mut pieces: Vec<usize> = puzzle[1..].to_vec();
	pieces.sort();
	for (index, value) in pieces.iter().enumerate() {
		if index != *value {
			return false;
		}
	}
	
	true
}

pub fn parsing(path: &String) -> std::io::Result<(usize, Vec<usize>)> {
	let mut buffer: String = String::new();
	read_file(path, &mut buffer)?;

	let _puzzle: Vec<usize> = _extract_puzzle(buffer);

	if !is_valid_puzzle(&_puzzle) {
		panic!("Parsing: Invalid.");
	}

	Ok((_puzzle[0], _puzzle[1..].to_vec()))
}
