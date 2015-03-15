use grid::Grid;
use player::Player;

#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
pub enum CellStatus {
	Empty,
	Black,
	White
}

#[derive(Clone)]
#[derive(Copy)]
pub struct Cell {
	pub status: CellStatus,
	pub x: u8,
	pub y: u8
}

pub struct RelativePositionWeight {
	x: i8,
	y: i8,
	weight: i8
}

#[derive(Copy)]
pub struct PositionWeight {
	pub x: u8,
	pub y: u8,
	pub weight: i8
}

impl CellStatus {
	fn to_letter(&self) -> &'static str {
		return match *self {
				CellStatus::Black => "B",
				CellStatus::White => "W",
				CellStatus::Empty => "-",
			};
	}
	pub fn to_string(&self) -> &'static str {
		return match *self {
				CellStatus::Black => "Black (Vertically)",
				CellStatus::White => "White (Horizontally)",
				CellStatus::Empty => "Nothing",
			};
	}
	pub fn get_color_for_player(num: u8) -> CellStatus {
		return match num {
			1 => CellStatus::Black,
			2 => CellStatus::White,
			_ => {panic!("This game can't be player with other player than 1 or 2")}
		};
	}
}

impl Cell {
	pub fn new(x: u8, y: u8) -> Cell {
		return Cell {
			status: CellStatus::Empty,
			x: x,
			y: y
		};
	}
	pub fn to_string(&self) -> &'static str {
		return self.status.to_letter();
	}
	pub fn is_empty(&self) -> bool {
		match self.status {
			CellStatus::Empty => true,
			_ => false
		}
	}
	pub fn get_weight(&self, player: &Player) -> u8 {
		return if self.status == player.cell_code { 0 } else { 1 };
	}

//	hex sides useful representation
//	[ _, _]  [ 0,-1]  [ 1,-1]
//	    [-1, 0]  [ _, _]  [ 1, 0]
//	      [-1, 1]  [ 0, 1]  [ _, _]
	fn get_close_relative(&self, color: CellStatus) -> Vec<RelativePositionWeight> {
		let mut list: Vec<RelativePositionWeight> = Vec::with_capacity(6);
		match color {
			CellStatus::Empty => {
				list.push(RelativePositionWeight{x: 0, y: -1, weight: 0});
				list.push(RelativePositionWeight{x: 1, y: -1, weight: 0});
				list.push(RelativePositionWeight{x: -1, y: 0, weight: 0});
				list.push(RelativePositionWeight{x: 1, y: 0, weight: 0});
				list.push(RelativePositionWeight{x: -1, y: 1, weight: 0});
				list.push(RelativePositionWeight{x: 0, y: 1, weight: 0});
			},
			CellStatus::White => {
				list.push(RelativePositionWeight{x: -1, y: 1, weight: 1});
				list.push(RelativePositionWeight{x: 0, y: 1, weight: 1});
				list.push(RelativePositionWeight{x: -1, y: 0, weight: 0});
				list.push(RelativePositionWeight{x: 1, y: 0, weight: 0});
				list.push(RelativePositionWeight{x: 0, y: -1, weight: -1});
				list.push(RelativePositionWeight{x: 1, y: -1, weight: -1});
			},
			CellStatus::Black => {
				list.push(RelativePositionWeight{x: 1, y: -1, weight: 1});
				list.push(RelativePositionWeight{x: 1, y: 0, weight: 1});
				list.push(RelativePositionWeight{x: 0, y: -1, weight: 0});
				list.push(RelativePositionWeight{x: 0, y: 1, weight: 0});
				list.push(RelativePositionWeight{x: -1, y: 0, weight: -1});
				list.push(RelativePositionWeight{x: -1, y: 1, weight: -1});
			}
		}
		return list;
	}

	pub fn get_close(&self, grid: &Grid, color: CellStatus) -> Vec<PositionWeight> {
		let grid_length = grid.length as i16;
		let mut list: Vec<PositionWeight> = Vec::with_capacity(6);
		for relative_position in self.get_close_relative(color) {
			let i_x = self.x as i16 + relative_position.x as i16;
			let i_y = self.y as i16 + relative_position.y as i16;
			if i_x < 0 || i_y < 0 || i_x >= grid_length || i_y >= grid_length {
				continue;
			}
			list.push(PositionWeight{ weight: relative_position.weight, x: i_x as u8, y: i_y as u8 });
		}
		return list;
	}

	pub fn get_same_close(&self, grid: &Grid) -> Vec<Cell> {
		let mut list: Vec<Cell> = Vec::with_capacity(6);
		for cell_close in self.get_close(grid, self.status) {
			let cell_close_Cell = grid.get_cell([cell_close.x, cell_close.y]);
			if self.status == cell_close_Cell.status {
				list.push(*cell_close_Cell);
			}
		}
		return list;
	}

	pub fn get_available_close(&self, grid: &Grid) -> Vec<Cell> {
		let mut list: Vec<Cell> = Vec::with_capacity(6);
		for cell_close in self.get_close(grid, self.status) {
			let cell_close_Cell = grid.get_cell([cell_close.x, cell_close.y]);
			if CellStatus::Empty == cell_close_Cell.status {
					list.push(*cell_close_Cell);
			}
		}
		return list;
	}

	fn get_close_random(&self, grid: &Grid, color: CellStatus) -> Vec<PositionWeight> {
		use std::rand::{thread_rng, Rng};

		let mut cells_close = self.get_close(grid, color);
		let mut sliced_shuffled_cells_close: Vec<PositionWeight> = Vec::with_capacity(cells_close.len());
		let sliced_cells_close: &mut [PositionWeight] = cells_close.as_mut_slice();
		thread_rng().shuffle(sliced_cells_close);
		for cell in sliced_cells_close {
			sliced_shuffled_cells_close.push(*cell);
		}
		sort_by_weight(&mut sliced_shuffled_cells_close);
		return sliced_shuffled_cells_close;
	}

	pub fn get_available_close_random_uniq_weight(&self, grid: &Grid) -> Vec<Cell> {
		let mut list: Vec<Cell> = Vec::with_capacity(6);
		let mut weight_list: Vec<i8> = Vec::with_capacity(3);
		for cell_close in self.get_close_random(grid, self.status) {
			if i8_is_in_vector(cell_close.weight, &weight_list) {
				continue;
			}
			weight_list.push(cell_close.weight);
			let cell_close_Cell = grid.get_cell([cell_close.x, cell_close.y]);
			if CellStatus::Empty == cell_close_Cell.status {
					list.push(*cell_close_Cell);
			}
		}
		return list;
	}

	pub fn is_a_goal(&self, player: &Player, grid: &Grid) -> bool {
		let max_index = grid.length - 1;
		return (player.cell_code == CellStatus::Black && self.x == max_index)
		|| (player.cell_code == CellStatus::White && self.y == max_index);
	}

	pub fn to_coord(&self) -> [u8; 2] {
		return [self.x, self.y];
	}

	pub fn is_in_vector(&self, positions: &Vec<[u8; 2]>) -> bool {
		for pos in positions.iter() {
			if [self.x, self.y] == *pos {
				return true;
			}
		}
		return false;
	}
}

impl PositionWeight {
	pub fn is_in_vector(&self, positions: &Vec<[u8; 2]>) -> bool {
		for pos in positions.iter() {
			if [self.x, self.y] == *pos {
				return true;
			}
		}
		return false;
	}
}

pub fn i8_is_in_vector(value: i8, vector: &Vec<i8>) -> bool {
	for pos in vector.iter() {
		if value == *pos {
			return true;
		}
	}
	return false;
}

pub fn sort_by_weight(vector: &mut Vec<PositionWeight>){
	let mut temp;
	let length = vector.len();

	for _ in 0.. length {
		for i in 0..length - 1 {
			if vector[i].weight < vector[i+1].weight  {
				temp = vector[i+1];
				vector[i+1] = vector[i];
				vector[i] = temp;
			}
		}
	}
}