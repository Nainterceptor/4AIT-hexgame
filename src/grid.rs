
use std::old_io;
use cell::Cell;
use cell::CellStatus;
use player::Player;

static DEFAULT_GRID: u8 = 10;

pub struct Grid {
	vector: Vec<Vec<Cell>>,
	pub length: u8
}

impl Grid {
	//For performances considerations, sides can't be > to 255, like u8
	pub fn new(side_length: u8) -> Grid {
		let mut vector: Vec<Vec<Cell>> = Vec::with_capacity(side_length as usize);
		for x in 0..side_length {
			let mut vector_internal: Vec<Cell> = Vec::with_capacity(side_length as usize);
			for y in 0..side_length {
				vector_internal.push(Cell::new(x, y));
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

	pub fn get_cell(&self, cell: [u8; 2]) -> &Cell {
		return &self.vector[cell[0] as usize][cell[1] as usize];
	}

	fn get_mut_cell(&mut self, cell: [u8; 2]) -> &mut Cell {
		return &mut self.vector[cell[0] as usize][cell[1] as usize];
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

	pub fn has_path_to_goal(&self, player: &Player) -> bool {
		let mut positions_visited: Vec<[u8; 2]> = Vec::new();
		for i in 0..self.length {
			if (player.cell_code == CellStatus::Black && player.cell_code == self.get_cell([0, i]).status && self.go_to_goal(player, [0, i], &mut positions_visited))
			|| (player.cell_code == CellStatus::White && player.cell_code == self.get_cell([i, 0]).status && self.go_to_goal(player, [i, 0], &mut positions_visited)) {
				return true
			}
		}
		return false
	}

	fn go_to_goal(&self, player: &Player, coord: [u8; 2], positions_visited: &mut Vec<[u8; 2]>) -> bool {
		positions_visited.push(coord);
		let cell: &Cell = self.get_cell(coord);
		if cell.is_a_goal(player, self) {
			return true;
		}
		for cell in cell.get_same_close(self) {
			if !cell.is_in_vector(positions_visited)
			&& self.go_to_goal(player, cell.to_coord(), positions_visited) {
				return true;
			}
		}
		return false;
	}

	pub fn count_for_player(&self, player: &Player) -> u16 {
		let mut sum = 0;
		for x in self.vector.iter() {
			for y in x.iter() {
				if y.status == player.cell_code {
					sum = sum + 1;
				}
			}
		}
		return sum;
	}
}