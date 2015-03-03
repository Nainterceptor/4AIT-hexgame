#[derive(PartialEq)]
pub enum CellStatus {
	Empty,
	Red,
	Blue
}

pub struct Cell {
	pub status: CellStatus
}

impl CellStatus {
	fn to_letter(&self) -> &'static str {
		return match *self {
				CellStatus::Red => "R",
				CellStatus::Blue => "B",
				CellStatus::Empty => "-",
			};
	}
	pub fn to_string(&self) -> &'static str {
		return match *self {
				CellStatus::Red => "Red",
				CellStatus::Blue => "Blue",
				CellStatus::Empty => "Nothing",
			};
	}
	pub fn get_color_for_player(num: u8) -> CellStatus {
		return match num {
			1 => CellStatus::Red,
			2 => CellStatus::Blue,
			_ => {panic!("This game can't be player with other player than 1 or 2")}
		};
	}
}

impl Cell {
	pub fn new() -> Cell {
		return Cell {
			status: CellStatus::Empty
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
//	pub fn status_eq(&self, cell_status: CellStatus) -> bool {
//		match self.status {
//			cell_status => true,
//			_ => false
//		}
//	}
}