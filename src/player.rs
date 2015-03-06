use std::old_io;
use cell::CellStatus;

pub struct Player {
	pub name: String,
	pub cell_code: CellStatus,
	pub player_type: PlayerType
}

pub enum PlayerType {
	RandomAI,
	PathAI,
	ButeforceAI,
	Human
}

impl Player {
	pub fn new(num: u8) -> Player {

		println!("Please, give us a name for player {}", num);

		let mut stdin = old_io::stdin();
		let grid_input = stdin.lock().lines().next().unwrap().ok().unwrap();
		let name = grid_input.trim();

		let player_type: PlayerType;
		loop {
			println!("{} will be [h]uman, [r]andom IA, [p]ath IA, [b]ruteforce IA ?", name);
			let grid_input = stdin.lock().lines().next().unwrap().ok().unwrap();
			match grid_input.trim() {
					"h" => { player_type = PlayerType::Human; break; },
					"p" => { player_type = PlayerType::PathAI; break;},
					"r" => { player_type = PlayerType::RandomAI; break;},
					"b" => { player_type = PlayerType::ButeforceAI; break;}
					_ => {continue;}
				};
		};

		return Player {
			name: (*name).to_string(),
			cell_code: CellStatus::get_color_for_player(num),
			player_type: player_type
		};
	}
	pub fn inverse(&self) -> CellStatus {
		return match self.cell_code {
			CellStatus::Black => CellStatus::White,
			CellStatus::White => CellStatus::Black,
			_ => panic!("Bad player color")
		}
	}
}

impl PlayerType {
	pub fn to_string(&self) -> &'static str {
		return match *self {
				PlayerType::PathAI => "Path AI",
				PlayerType::RandomAI => "Random AI",
				PlayerType::ButeforceAI => "Bruteforce AI",
				PlayerType::Human => "Human"
			};
	}
}