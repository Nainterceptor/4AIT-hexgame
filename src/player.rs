use std::old_io;
use cell::CellStatus;
use grid::Grid;
use cell::Cell;
use cell::PositionWeight;

pub struct Player {
	pub name: String,
	pub cell_code: CellStatus,
	pub player_type: PlayerType,
	pub played_cells: Vec<Cell>
}

pub enum PlayerType {
	RandomAI,
	PathAI,
	MindPathAI,
	ButeforceAI,
	Human
}

impl Player {
	pub fn new(num: u8, grid_length: u8) -> Player {
		let possibilities: u16 = match num {
			1 if grid_length % 2 != 0 => {
				(grid_length*grid_length + 1) as u16
			},
			_ => {
				(grid_length*grid_length) as u16
			}
		};grid_length as u16 * grid_length as u16;
		println!("Please, give us a name for player {}", num);

		let mut stdin = old_io::stdin();
		let grid_input = stdin.lock().lines().next().unwrap().ok().unwrap();
		let name = grid_input.trim();

		let player_type: PlayerType;
		loop {
			println!("{} will be [h]uman, [r]andom IA, [p]ath IA, [m]ind path IA, [b]ruteforce IA ?", name);
			let grid_input = stdin.lock().lines().next().unwrap().ok().unwrap();
			match grid_input.trim() {
					"h" => { player_type = PlayerType::Human; break; },
					"p" => { player_type = PlayerType::PathAI; break;},
					"m" => { player_type = PlayerType::MindPathAI; break;},
					"r" => { player_type = PlayerType::RandomAI; break;},
					"b" => { player_type = PlayerType::ButeforceAI; break;}
					_ => {continue;}
				};
		};

		return Player {
			name: (*name).to_string(),
			cell_code: CellStatus::get_color_for_player(num),
			player_type: player_type,
			played_cells: Vec::with_capacity(possibilities as usize)
		};
	}

	pub fn inverse(&self) -> CellStatus {
		return match self.cell_code {
			CellStatus::Black => CellStatus::White,
			CellStatus::White => CellStatus::Black,
			_ => panic!("Bad player color")
		}
	}

	pub fn get_next_PathAI_move(&self, grid: &Grid) -> Option<[u8; 2]> {
		use std::rand;
		use std::rand::Rng;
		if grid.count_for_player(self) == 0 {
			let aleat: u8 = rand::thread_rng().gen_range(0, grid.length-1);
			if self.cell_code == CellStatus::White {
				return Some([aleat, 0]);
			} else {
				return Some([0, aleat]);
			}
		}
		let available_moves = self.played_cells.last().unwrap().get_available_close_random_uniq_weight(grid);
		if available_moves.len() < 1 {
			return None;
		}
		let next = self.played_cells.last().unwrap().get_available_close_random_uniq_weight(grid)[0];
		return Some([next.x, next.y]);
	}

	pub fn add_played_cell(&mut self, cell: Cell) {
		self.played_cells.push(cell);
	}
}

impl PlayerType {
	pub fn to_string(&self) -> &'static str {
		return match *self {
				PlayerType::PathAI => "Path AI",
				PlayerType::MindPathAI => "Mind path AI",
				PlayerType::RandomAI => "Random AI",
				PlayerType::ButeforceAI => "Bruteforce AI",
				PlayerType::Human => "Human"
			};
	}
}