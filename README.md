# n-puzzle

python3 npuzzle-gen.py 4 | wc -l



use std::fs::File;
use std::io::prelude::*;

fn file_to_string(_path: &str, _buffer: &mut String) -> std::io::Result<()> {
	let mut _file: File = File::open(_path).expect("Open file.");
	_file.read_to_string(_buffer).expect("Read to string.");
	Ok(())
}

fn parsing() -> std::io::Result<()> {
	
	let mut _buffer: String = String::new();
	file_to_string("./input/puzzle_1.txt", &mut _buffer)?;

	let mut _size: u32 = 0;
	let mut _puzzle: Vec<u32> = Vec::new();

	for _char in _buffer.as_str().chars() {

		let mut _data_extract : String = String::new();
		let mut _is_comment: bool = false;

		if _char == '#' {
			_is_comment = true;
		}
		else if _char == '\n' {
			_is_comment = false;
		}
		else if _is_comment {
			if _char.is_ascii_digit() {
				_data_extract.push(_char);
			}
			else if !_char.is_whitespace() {
				break;
			}
		}

		match _char {
			'#' => {
				_is_comment = true;
			},
			'\n' => {
				_is_comment = true;
			},
			_char if _char.is_ascii_digit() => _data_extract.push(_char),
			_char if _char.is_whitespace() => continue,
			_ => break,
		}
		println!("HEY {_data_extract}");
		if !_data_extract.is_empty() && !_char.is_ascii_digit() {
			println!("HEY");
			_puzzle.push(_data_extract.parse().expect("String to unsigned integer."));
			_data_extract.clear();
		}
	}

	//for _line in _buffer.lines() {
	//	let mut _tmp : String = String::new();
	//	for _char in _line.chars() {
	//		if !_tmp.is_empty() && !_char.is_ascii_digit() {
	//			_puzzle.push(_tmp.parse().expect("String to interger"));
	//			_tmp.clear();
	//		}
	//		match _char {
	//			_char if _char.is_ascii_digit() => _tmp.push(_char),
	//			_char if _char.is_whitespace() => continue,
	//			'#' => break,
	//			_ => panic!("Invalid Map !!!"),
	//		}
	//	}
	//	if !_tmp.is_empty() {
	//		_puzzle.push(_tmp.parse().expect("String to interger"));
	//		_tmp.clear();
	//	}
	//}

	for x in _puzzle {
		println!("{x}");
	}

    Ok(())
}

fn main() {
	match parsing() {
		Ok(_) => {},
		Err(err) => eprintln!("{err}"),
	}
}
