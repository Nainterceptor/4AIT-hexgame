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

	pub fn get_best_move(&self, grid: &Grid) -> Option<[u8; 2]> {
		let mut best_move: Option<[u8; 2]> = None;
		let mut best_move_cost: Option<u8> = None;
		let shuffled_positions: Vec<[u8; 2]> = grid.get_shuffled_free_cells();
		for cell in shuffled_positions.iter() {
			let mut grid_to_test = grid.clone();
			grid_to_test.edit(&self.cell_code, *cell);
			for i in 0..grid.length {
				let coord: [u8; 2] = match self.cell_code {
					CellStatus::White => [0, i],
					CellStatus::Black => [i, 0],
					_ => { panic!("Failed to match the color"); }
				};
				let to_ignore_init: Vec<[u8; 2]> = Vec::new();
				let weight_to_goal = self.get_weight_to_goal(&grid_to_test, &coord, &to_ignore_init);
				if weight_to_goal == None {
					continue;
				}
				if best_move_cost == None || best_move_cost.unwrap() > weight_to_goal.unwrap() {
					best_move_cost = weight_to_goal;
					best_move = Some(*cell);
				}
			}
		}
		return best_move;
	}


	pub fn get_weight_to_goal(&self, grid: &Grid, cell: &[u8; 2], to_ignore: &Vec<[u8; 2]>) -> Option<u8> {
		let cell_object = grid.get_cell(*cell);
		if cell_object.is_a_goal(&self, grid) {
			return Some(cell_object.get_weight(&self));
		}
		let mut to_ignore_cell = to_ignore.clone();
		to_ignore_cell.push(*cell);
		let available_cells_close = cell_object.get_close(grid, self.cell_code);
		let mut shorter_path: Option<u8> = None;
		let mut last_weight: Option<i8> = None;
		let mut goal_reached: bool = false;
		for cell in available_cells_close.iter() {
			if cell.is_in_vector(&to_ignore_cell) {
				continue;
			}
			let weight_to_goal = self.get_weight_to_goal(grid, &[cell.x, cell.y], &to_ignore_cell);
			if weight_to_goal == None {
				continue;
			}
			goal_reached = true;
			if shorter_path == None || weight_to_goal.unwrap() < shorter_path.unwrap() {
				shorter_path = weight_to_goal;
			}
			last_weight = Some(cell.weight);
		}
		if !goal_reached || shorter_path == None {
			return None;
		}
		return Some(shorter_path.unwrap() + cell_object.get_weight(&self));
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