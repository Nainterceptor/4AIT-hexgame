
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

	pub fn has_winner_path(&self, player: &Player) -> bool {
		for i in 0u8..self.length {
			let vector: Vec<[u8; 2]> = Vec::new();
			return match player.cell_code {
				CellStatus::Red => self.go_to_goal(player, [0, i], &vector),
				CellStatus::Blue => self.go_to_goal(player, [i, 0], &vector),
				_ => panic!("Bad cell code")
			};
		}
		return false;
	}

	fn go_to_goal(&self, player: &Player, cell: [u8; 2], to_ignore: &Vec<[u8; 2]>) -> bool {
		if self.is_a_goal(cell, player) {
			return true;
		}
		let mut to_ignore_cell = to_ignore.clone();
		to_ignore_cell.push(cell);

		let available_cells_close = self.not_visited_cells_close(player, cell, &to_ignore_cell, true);

		for cell in available_cells_close.iter() {
			if self.go_to_goal(player, *cell, &to_ignore_cell) {
				return true;
			}
		}
		return false;
	}

	fn is_a_goal(&self, cell: [u8; 2], player: &Player) -> bool{
		let max_index: u8 = self.length - 1;
		return match player.cell_code {
			CellStatus::Red => cell[0] == max_index,
			CellStatus::Blue => cell[1] == max_index,
			_ => panic!("Bad cell code")
		};
	}

	fn not_visited_cells_close(&self, player: &Player, cell: [u8; 2], visited: &Vec<[u8; 2]>, focus_on_player: bool) -> Vec<[u8; 2]> {
		let cells_close: Vec<[u8; 2]> = self.available_cells_close(player, cell, focus_on_player);
		let mut not_visited_cells_close: Vec<[u8; 2]> = Vec::with_capacity(cells_close.len());

		for cell in cells_close.iter() {
			if !has_pos_in_vector(&visited, *cell) {
				not_visited_cells_close.push(*cell);
			}
		}
		return not_visited_cells_close;
	}

	fn available_cells_close(&self, player: &Player, cell: [u8; 2], focus_on_player: bool) -> Vec<[u8; 2]> {
		let cells_close: Vec<[u8; 2]> = self.cells_close(cell);
		let mut available_cells_close: Vec<[u8; 2]> = Vec::with_capacity(cells_close.len());

		for cell in cells_close.iter() {
			let cell_obj: &Cell = self.get_cell(*cell);
			if (cell_obj.status != player.inverse() && !focus_on_player) || (cell_obj.status != player.inverse() && !cell_obj.is_empty())  {
				available_cells_close.push(*cell);
			}
		}
		return available_cells_close;
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
			self.get_a_position("X".to_string()),
			self.get_a_position("Y".to_string()),
		]
	}

	fn get_a_position(&self, axe: String) -> u8 {
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

	pub fn edit(&mut self, player: &Player, coord: [u8; 2]) {
		let cell: &mut Cell = self.get_mut_cell(coord);
		cell.status = match player.cell_code {
			CellStatus::Red => CellStatus::Red,
			CellStatus::Blue => CellStatus::Blue,
			_ => panic!("Bad code")
		};
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

	pub fn get_lower_weight(&self, player: &Player) -> u8 {
		let mut weight: u8 = 255;
		for i in 0u8..self.length as u8 {
			let vector: Vec<[u8; 2]> = Vec::new();
			let (current_weight, has_reach_the_goal) = match player.cell_code {
				CellStatus::Red => self.get_faster_to_goal(player, [0, i], &vector),
				CellStatus::Blue => self.get_faster_to_goal(player, [i, 0], &vector),
				_ => { panic!("Incorrect player for get_lower_weight"); }
			};
			if current_weight < weight && has_reach_the_goal {
				weight = current_weight;
			}
		}
		return weight;
	}

	fn get_faster_to_goal(&self, player: &Player, coord: [u8; 2], to_ignore: &Vec<[u8; 2]>) -> (u8, bool) {
		let mut path_weight: u8 = self.get_weight(player, coord);
	//
		if self.is_a_goal(coord, player) {
			return (path_weight, true);
		}

		let mut to_ignore_cell = to_ignore.clone();
		to_ignore_cell.push(coord);

		let available_cells_close = self.not_visited_cells_close(player, coord, &to_ignore_cell, false);

		let mut goal_reached: bool = false;
		if available_cells_close.len() != 0 {
			let mut shorter_path = 255;
			for cell in available_cells_close.iter() {
				let (faster_to_goal, reach_goal) = self.get_faster_to_goal(player, *cell, &to_ignore_cell);
				if !reach_goal {
					continue;
				}
				goal_reached = true;
				if faster_to_goal < shorter_path {
					shorter_path = faster_to_goal;
				}
			}
			path_weight = path_weight + shorter_path;
		}
		return (path_weight, goal_reached);
	}

	fn get_weight(&self, player: &Player, coord: [u8; 2]) -> u8 {
		if self.get_cell(coord).status == player.cell_code { 0 } else { 1 }
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