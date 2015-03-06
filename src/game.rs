use player::Player;
use player::PlayerType;
use grid::Grid;

pub struct Game {
	grid: Grid,
	players: [Player; 2],
	turn: u16
}

impl Game {
	pub fn new() -> Game {
		println!("First, please answer to some questions...");

		return Game {
			grid: Grid::new(Grid::get_u8_in_range()),
			players: [Player::new(1), Player::new(2)],
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
					let shuffled_grid: Vec<[u8; 2]> = self.grid.get_shuffled_free_cells();
					self.grid.edit(&self.players[(self.turn % 2)  as usize].cell_code, shuffled_grid[0]);
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
				_ => {
					panic!("Player type not supported");
				}
			};
			self.turn_increase();
		}
	}

	fn has_winner(&self) -> bool {
		for player in self.players.iter() {
			/*if self.grid.has_winner_path(player) {
				println!("Congratulations, {}, you've won.", player.name);
				return true;
			}*/
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