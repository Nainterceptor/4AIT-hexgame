use grid::Grid;
use player::Player;

#[derive(PartialEq)]
#[derive(Copy)]
pub enum CellStatus {
	Empty,
	Black,
	White
}

#[derive(Copy)]
pub struct Cell {
	pub status: CellStatus,
	x: u8,
	y: u8
}

pub struct RelativePositionWeight {
	x: i8,
	y: i8,
	weight: u8
}

pub struct PositionWeight {
	x: u8,
	y: u8,
	weight: u8
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

//	hex sides useful representation
//	[ _, _]  [ 0,-1]  [ 1,-1]
//	    [-1, 0]  [ _, _]  [ 1, 0]
//	      [-1, 1]  [ 0, 1]  [ _, _]
	fn get_close_relative(&self) -> Vec<RelativePositionWeight> {
		let mut list: Vec<RelativePositionWeight> = Vec::with_capacity(6);
		list.push(RelativePositionWeight{x:1, y: -1, weight: 3});
		list.push(RelativePositionWeight{x:1, y: 0, weight: 3});
		list.push(RelativePositionWeight{x:0, y: -1, weight: 2});
		list.push(RelativePositionWeight{x:0, y: 1, weight: 2});
		list.push(RelativePositionWeight{x:-1, y: 0, weight: 1});
		list.push(RelativePositionWeight{x:-1, y: 1, weight: 1});
		return list;
	}

	fn get_close(&self, grid: &Grid) -> Vec<PositionWeight> {
		let grid_length = grid.length as i16;
		let mut list: Vec<PositionWeight> = Vec::with_capacity(6);
		for relative_position in self.get_close_relative() {
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
		for cell_close in self.get_close(grid) {
			let cell_close_Cell = grid.get_cell([cell_close.x, cell_close.y]);
			if self.status == cell_close_Cell.status {
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