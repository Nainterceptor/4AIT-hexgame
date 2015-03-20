use std::old_io;
use cell::CellStatus;
use grid::Grid;
use cell::Cell;
use cell::PositionWeight;

pub struct Player {
	pub name: String,
	pub cell_code: CellStatus,
	pub player_type: PlayerType,
	pub played_cells: Vec<Cell>,
	pub path_cells: Vec<Cell>
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
			played_cells: Vec::with_capacity(possibilities as usize),
			path_cells: Vec::new()
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
			let aleat: u8 = rand::thread_rng().gen_range(1, grid.length-1);
			if self.cell_code == CellStatus::White {
				return Some([aleat, 0]);
			} else {
				return Some([0, aleat]);
			}
		}
		let available_moves = self.played_cells.last().unwrap().get_available_close_random_uniq_weight(grid, None);
		if available_moves.len() < 1 {
			return None;
		}
		let next = available_moves[0];
		return Some([next.x, next.y]);
	}

	pub fn init_path_cells(&mut self, grid: &Grid) {
		use std::rand;
		use std::rand::Rng;
		let aleat: u8 = rand::thread_rng().gen_range(1, grid.length-1);
		if self.cell_code == CellStatus::White {
			self.path_cells.push(*grid.get_cell([aleat, 0]));
		} else {
			self.path_cells.push(*grid.get_cell([0, aleat]));
		}
		while !self.path_cells.last().unwrap().is_a_goal(&self, grid) {
			let next = self.path_cells.last().unwrap().get_available_close_random_uniq_weight(grid, Some(self.cell_code))[0];
			self.path_cells.push(next);
		}
	}

	pub fn update_path_data(&mut self, grid: &Grid) {
		let temp_vector: Vec<Cell> = self.path_cells.clone();
		self.path_cells = Vec::with_capacity(temp_vector.len());
		for cell in temp_vector.iter() {
			self.path_cells.push(*grid.get_cell([cell.x, cell.y]));
		}

	}

	pub fn path_must_be_recalc(&self) -> bool {
		for cell in self.path_cells.iter() {
			if cell.status == self.inverse() {
				return true;
			}
		}
		return false;
	}

	pub fn recalc_path(&mut self, grid: &Grid) {
		let mut i = 0;
		let mut new_path: Vec<Cell> = Vec::new();
		for cell in self.path_cells.iter() {
			if cell.status != CellStatus::Empty && cell.status != self.cell_code {
				break;
			}
			new_path.push(*cell);
		}
		loop {
			let mut to_ignore: Vec<[u8;2 ]> = Vec::new();
			let last_coord: [u8; 2] = match new_path.clone().last() {
				Some(x) => { [x.x, x.y] },
				None => {
					use std::rand;
					use std::rand::Rng;
					let aleat: u8 = rand::thread_rng().gen_range(1, grid.length-1);
					if self.cell_code == CellStatus::White {
						new_path.push(*grid.get_cell([aleat, 0]));
						[aleat, 0]
					} else {
						new_path.push(*grid.get_cell([0, aleat]));
						[0, aleat]
					}
				}
			};
			match self.get_path_to_goal(grid, &last_coord, &mut to_ignore) {
				Some(x) => {
					let mut path = x;
					self.path_cells.append(&mut path);
					break;
				},
				None => {
					self.path_cells.pop();
				}
			};
		}
		self.path_cells = new_path;

//		println!("{:?}", );
//		while !self.path_cells.last().unwrap().is_a_goal(&self, grid) {
//			let next = self.path_cells.last().unwrap().get_available_close_random_uniq_weight(grid, Some(self.cell_code))[0];
//			self.path_cells.push(next);
//		}
	}

	pub fn get_next_MindPathAI_move(&mut self, grid: &Grid) -> Option<Cell> {
		if self.path_cells.len() == 0 {
			self.init_path_cells(grid);
			return Some(self.path_cells[0]);
		} else {
			self.update_path_data(grid);
		}
		if self.path_must_be_recalc() {
			self.recalc_path(grid)
		}
		for cell in self.path_cells.iter() {
			if cell.status == CellStatus::Empty {
				return Some(*cell);
			}
		}
		println!("{:?}", self.path_cells);
		return None;
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

	pub fn get_path_to_goal(&self, grid: &Grid, cell: &[u8; 2], to_ignore: &Vec<[u8; 2]>) -> Option<Vec<Cell>> {
		let cell_object = grid.get_cell(*cell);
		if cell_object.is_a_goal(&self, grid) {
			let mut vec: Vec<Cell> = Vec::new();
			vec.push(*cell_object);
			return Some(vec);
		}
		let mut to_ignore_cell = to_ignore.clone();
		to_ignore_cell.push(*cell);
		let available_cells_close = cell_object.get_close(grid, self.cell_code);
		let mut shorter_path: Vec<Cell> = Vec::new();
		let mut last_weight: Option<i8> = None;
		let mut has_shorter_path = false;
		let mut goal_reached: bool = false;
		for cell in available_cells_close.iter() {
			if cell.is_in_vector(&to_ignore_cell) {
				continue;
			}
//			if last_weight == None {
//				last_weight = Some(cell.weight);
//			} else if last_weight.unwrap > cell.weight  {
//
//			}
			let path_to_goal: Vec<Cell> = match self.get_path_to_goal(grid, &[cell.x, cell.y], &to_ignore_cell) {
				Some(x) => x,
				None => { continue; }
			};
			goal_reached = true;
			if !has_shorter_path || path_to_goal.len() < shorter_path.len() {
				shorter_path = path_to_goal;
				has_shorter_path = true;
			}
			break;

		}
		if !goal_reached || !has_shorter_path {
			return None;
		}
		let mut new_cell_list: Vec<Cell> = Vec::new();
		new_cell_list.push(*cell_object);
		new_cell_list.append(&mut shorter_path);
		return Some(shorter_path);
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