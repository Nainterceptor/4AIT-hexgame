use player::Player;
use player::PlayerType;
use grid::Grid;
use cell::Cell;
use cell::CellStatus;

pub struct Game {
	grid: Grid,
	players: [Player; 2],
	turn: u16
}

impl Game {
	pub fn new() -> Game {
		println!("First, please answer to some questions...");
		let length = Grid::get_u8_in_range();
		return Game {
			grid: Grid::new(length),
			players: [Player::new(1, length), Player::new(2, length)],
			turn: 0
		};
	}

	pub fn new_for_bench(length: u8, player_type: PlayerType) -> Game {
		return Game {
			grid: Grid::new(length),
			players: [Player::new_for_bench(1, length, player_type), Player::new_for_bench(2, length, player_type)],
			turn: 0
		};
	}

	pub fn play(&mut self) {
		//u16 cover u8*u8
		loop {
			self.grid.print();
			if self.has_winner() {
				break;
			}
			self.print_turn_message();
			match self.get_current_player().player_type {
				PlayerType::RandomAI => {
					let random_choice: [u8; 2] = self.grid.get_shuffled_free_cells()[0];
					self.grid.edit(&self.players[(self.turn % 2)  as usize].cell_code, random_choice);
				},
				PlayerType::PathAI => {
					let coord: [u8; 2] = match self.get_current_player().get_next_PathAI_move(&self.grid) {
						Some(x) => x,
						None => self.grid.get_shuffled_free_cells()[0]
					};
					self.grid.edit(&self.players[(self.turn % 2)  as usize].cell_code, coord);
					self.players[(self.turn % 2) as usize].add_played_cell(*self.grid.get_cell(coord));
				},
				PlayerType::MindPathAI => {
					let cell: Cell = match self.players[(self.turn % 2) as usize].get_next_MindPathAI_move(&self.grid) {
						Some(x) => x,
						None => *self.grid.get_cell(self.grid.get_shuffled_free_cells()[0])
					};
					let coord: [u8; 2] = [cell.x, cell.y];
					self.grid.edit(&self.players[(self.turn % 2)  as usize].cell_code, coord);
					self.players[(self.turn % 2) as usize].add_played_cell(*self.grid.get_cell(coord));
				},
				PlayerType::ButeforceAI => {
					let coord: [u8; 2] = match self.get_current_player().get_best_move(&self.grid) {
						Some(x) => x,
						None => {
							println!("Fallback Random");
							self.grid.get_shuffled_free_cells()[0]
						}
					};
					self.grid.edit(&self.players[(self.turn % 2)  as usize].cell_code, coord);
				},
				PlayerType::Human => {
					let coord: [u8; 2] = self.grid.get_coord();
					if self.grid.is_free_cell(coord) {
						self.grid.edit(&self.players[(self.turn % 2)  as usize].cell_code, coord);
					} else {
						println!("This cell is not free");
						continue; //prevent turn_increase
					}
				}
			};
			self.turn_increase();
		}
	}

	fn has_winner(&self) -> bool {
		for player in self.players.iter() {
			if self.grid.has_path_to_goal(player) {
				println!("Congratulations, {}, you've won.", player.name);
				return true;
			}
		}
		return false;
	}

	fn turn_increase(&mut self) {
		self.turn = self.turn +1;
	}

	fn get_current_player(&self) -> &Player {
		return &self.players[(self.turn % 2)  as usize];
	}

	fn print_turn_message(&self) {
		let player: &Player = self.get_current_player();
		println!(
			"It's your turn (turn {turn}), {player} ({color} - {status})",
			turn = self.turn + 1,
			player = player.name,
			status = player.player_type.to_string(),
			color = player.cell_code.to_string()
		);
	}
}