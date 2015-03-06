
use std::old_io;
use cell::Cell;
use cell::CellStatus;
use player::Player;

static DEFAULT_GRID: u8 = 10;

pub struct Grid {
	vector: Vec<Vec<Cell>>,
	length: u8
}

impl Grid {
	//For performances considerations, sides can't be > to 255, like u8
	pub fn new(side_length: u8) -> Grid {
		let mut vector: Vec<Vec<Cell>> = Vec::with_capacity(side_length as usize);
		for _ in 0..side_length {
			let mut vector_internal: Vec<Cell> = Vec::with_capacity(side_length as usize);
			for _ in 0..side_length {
				vector_internal.push(Cell::new());
			}
			vector.push(vector_internal);
		}
		println!("Your grid have now a size of {size}", size = side_length);
		return Grid{vector: vector, length: side_length};
	}

	pub fn get_u8_in_range() -> u8 {
		let mut stdin = old_io::stdin();
		println!("Please give us grid size (3 - 255) :");
		let grid_input = stdin.lock().lines().next().unwrap().ok().unwrap();
		return match grid_input.trim().parse::<u8>() {
			Err(_) => DEFAULT_GRID,
			Ok(x) if x < 3 => DEFAULT_GRID,
			Ok(x) => x
		};
	}

	pub fn print(&self) {
		let mut spaces_before = "".to_string();
		for x in self.vector.iter() {
			print!("{}", spaces_before);
			for y in x.iter() {
				print!(" {} ", y.to_string());
			}
			print!("\n");
			spaces_before = spaces_before + "  ";
		}
	}


	fn get_cell(&self, cell: [u8; 2]) -> &Cell {
		return &self.vector[cell[0] as usize][cell[1] as usize];
	}

	fn get_mut_cell(&mut self, cell: [u8; 2]) -> &mut Cell {
		return &mut self.vector[cell[0] as usize][cell[1] as usize];
	}

	pub fn cells_close(&self, cell: [u8; 2]) -> Vec<[u8; 2]> {
		let mut cells_values: Vec<[u8; 2]> = Vec::with_capacity(8);
		let relative_positions: [[i8; 2]; 8] = [
			[-1, -1],	[-1, 0],	[-1, 1],
			[0, -1], 	/*0,0*/		[0, 1],
			[1, -1],	[1, 0],		[1, 1]];
		for relative_position in relative_positions.iter() {
			let new_i_x = cell[0] as i8 + relative_position[0];
			if new_i_x < 0 || new_i_x >= self.length as i8 {
				continue;
			}
			let new_x = new_i_x as u8;

			let new_i_y = cell[1] as i8 + relative_position[1];
			if new_i_y < 0 || new_i_y >= self.length as i8 {
				continue;
			}
			let new_y = new_i_y as u8;
			cells_values.push([new_x, new_y]);
		}
		return cells_values;
	}

	pub fn get_coord(&self) -> [u8; 2] {
		return [
			self.tell_a_position("X".to_string()),
			self.tell_a_position("Y".to_string()),
		]
	}

	fn tell_a_position(&self, axe: String) -> u8 {
		let position: u8;
		let mut stdin = old_io::stdin();
		loop {
			println!("Give us {} position", axe);
			let grid_input = stdin.lock().lines().next().unwrap().ok().unwrap();
			match grid_input.trim().parse::<u8>() {
				Ok(c) if c < self.length => { position = c; break; },
				Ok(_) => {continue;},
				Err(_) => {continue;}
			};
		};
		return position;
	}

	pub fn is_free_cell(&self, coord: [u8; 2]) -> bool {
		let cell: &Cell = self.get_cell(coord);
		return cell.is_empty();
	}

	pub fn edit(&mut self, value: &CellStatus, coord: [u8; 2]) {
		let cell: &mut Cell = self.get_mut_cell(coord);
		cell.status = *value;
	}

	pub fn reset(&mut self, coord: [u8; 2]) {
		let cell: &mut Cell = self.get_mut_cell(coord);
		cell.status = CellStatus::Empty;
	}

	pub fn get_shuffled_free_cells(&self) -> Vec<[u8; 2]> {
		use std::rand::{thread_rng, Rng};

		let mut free_cells: Vec<[u8; 2]> = Vec::new();
		for x in 0..self.length {
			for y in 0..self.length {
				let cell: [u8; 2] = [x as u8, y as u8];
				if self.is_free_cell(cell) {
					free_cells.push(cell);
				}
			}
		}
		let sliced_free_cells: &mut [[u8; 2]] = free_cells.as_mut_slice();
		thread_rng().shuffle(sliced_free_cells);

		let mut sliced_shuffled_free_cells: Vec<[u8; 2]> = Vec::new();
		sliced_shuffled_free_cells.push_all(&sliced_free_cells);
		return sliced_shuffled_free_cells;
	}
}

fn has_pos_in_vector(positions: &Vec<[u8; 2]>, search: [u8; 2]) -> bool {
	for pos in positions.iter() {
		if search == *pos {
			return true;
		}
	}
	return false;
}