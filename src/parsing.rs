
fn extract_puzzle(buffer: String) -> std::io::Result<Vec<usize>> {
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
	Ok(puzzle)
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
	let buffer: String = std::fs::read_to_string(path)?;
	let puzzle: Vec<usize> = extract_puzzle(buffer)?;

	if !is_valid_puzzle(&puzzle) {
		panic!("Parsing: Invalid.");
	}

	Ok((puzzle[0], puzzle[1..].to_vec()))
}
