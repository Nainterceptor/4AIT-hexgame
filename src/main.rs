#![feature(old_io)]
use std::old_io;
//static ITERATIONS: u16 = 5000;
static DEFAULT_GRID: u8 = 10;

enum PlayerType {
	AI,
	Human
}

struct Player {
	name: String,
	num: u8,
	player_type: PlayerType
}

fn main() {
	let mut stdin = old_io::stdin();
	//Choose the grid size
	println!("First, please answer to some questions...");
	println!("Please give us grid size :");
	let grid_input = stdin.lock().lines().next().unwrap().ok().unwrap();
	let grid_size: u8 = match grid_input.trim().parse::<u8>() {
			Ok(0) => DEFAULT_GRID,
			Ok(x) => x,
			Err(_) => DEFAULT_GRID
		};
	//Build the grid
	let mut grid: Vec<Vec<u8>> = generate_grid(grid_size);
	println!("Your grid have now a size of {size}", size = grid_size);

	let players: [Player; 2] = [get_a_player(&mut stdin, 1), get_a_player(&mut stdin, 2)];
	//game loop
	let mut turn : u32 = 0;

	loop {
		print_grid(&grid);
		if has_winner_path(&grid, 1) {
			println!("Congratulations, {}, you've won.", players[0].name);
			break;
		} else if has_winner_path(&grid, 2) {
			println!("Congratulations, {}, you've won.", players[1].name);
			break;
		}
		let player : &Player = &players[(turn % 2)  as usize];
		let player_status = match player.player_type {
			PlayerType::AI => "AI",
			PlayerType::Human => "Human"
		};
		println!("It's your turn (turn {turn}), {player} ({status})", turn = turn + 1, player = player.name, status = player_status);
		match player.player_type {
			PlayerType::AI => {
//				let position: [u8; 2] =
			},
			PlayerType::Human => {
				let position_x: u8 = get_a_position(&mut stdin, grid_size, "X".to_string());
				let position_y: u8 = get_a_position(&mut stdin, grid_size, "Y".to_string());
				if is_free_cell(&grid, position_x, position_y) {
					edit_grid(&mut grid, player.num, position_x, position_y);
					turn = turn + 1;
				} else {
					println!("This cell is not free");
				}
			}
		};
	}
}

fn get_a_player(stdin: &mut std::old_io::stdio::StdinReader, player_num: u8) -> Player {
	println!("Please, give us a name for player {}", player_num);
	let grid_input = stdin.lock().lines().next().unwrap().ok().unwrap();
	let name = grid_input.trim();
	let player_type: PlayerType;
	loop {
		println!("{} will be human ? (yes/no)", name);
		let grid_input = stdin.lock().lines().next().unwrap().ok().unwrap();
		match grid_input.trim() {
				"yes" => { player_type = PlayerType::Human; break; },
				"no" => { player_type = PlayerType::AI; break;},
				_ => {continue;}
			};
	};
	return Player { num: player_num, name: name.to_string(), player_type: player_type};
}

fn get_a_position(stdin: &mut std::old_io::stdio::StdinReader, max_value_plus_one: u8, axe: String) -> u8 {
	let position: u8;
	loop {
		println!("Give us {} position", axe);
		let grid_input = stdin.lock().lines().next().unwrap().ok().unwrap();
		match grid_input.trim().parse::<u8>() {
			Ok(x) if x < max_value_plus_one => { position = x; break; },
			Ok(_) => {continue;},
			Err(_) => {continue;}
		};
	};
	return position;
}

//For performances considerations, sides can't be > to 255.
fn generate_grid(side_length: u8) -> Vec<Vec<u8>> {
	let mut vector: Vec<Vec<u8>> = Vec::with_capacity(side_length as usize);
	for _ in 0..side_length {
		let mut vector_internal: Vec<u8> = Vec::with_capacity(side_length as usize);
		for _ in 0..side_length {
			vector_internal.push(0);
		}
		vector.push(vector_internal);
	}
	return vector;
}

fn edit_grid(grid: &mut Vec<Vec<u8>>, new_value: u8, x: u8, y: u8) {
	grid[x as usize][y as usize] = new_value;
}

fn print_grid(grid: &Vec<Vec<u8>>) {
	for x in grid.iter() {
		for y in x.iter() {
			print!(" {} ", match *y {
				0u8 => "-",
				1u8 => "B",
				2u8 => "W",
				_ => "E"
			});
		}
		print!("\n");
	}
}

fn is_free_cell(grid: &Vec<Vec<u8>>, x: u8, y: u8) -> bool {
	if grid[x as usize][y as usize] == 0u8 { true } else { false }
}

fn has_winner_path(grid: &Vec<Vec<u8>>, player: u8) -> bool {
	for i in 0u8..grid.len() as u8 {
		let vector: Vec<[u8; 2]> = Vec::new();
		let is_win: bool = match player {
			1u8 => go_to_goal(&grid, player, 0, i, &vector),
			2u8 => go_to_goal(&grid, player, i, 0, &vector),
			_ => { panic!("Incorrect player for has_winner_path"); }
		};
		if is_win {
			return true;
		}
	}
	return false;
}

fn go_to_goal(grid: &Vec<Vec<u8>>, player: u8, x: u8, y: u8, to_ignore: &Vec<[u8; 2]>) -> bool {
	if is_a_goal(&grid, player, x, y) {
		return true;
	}
	let mut to_ignore_cell = to_ignore.clone();
	to_ignore_cell.push([x, y]);

	let available_cells_close = not_visited_cells_close(&grid, player, x, y, &to_ignore_cell, true);

	for cell in available_cells_close.iter() {
		if go_to_goal(&grid, player, cell[0], cell[1], &to_ignore_cell) {
			return true;
		}
	}
	return false;
}

fn is_a_goal(grid: &Vec<Vec<u8>>, player: u8, x: u8, y: u8) -> bool{
	let max_index: u8 = grid.len() as u8 - 1;
	(player == 1 && x == max_index) || (player == 2 && y == max_index)
}

fn not_visited_cells_close(grid: &Vec<Vec<u8>>, player: u8, x: u8, y: u8, visited: &Vec<[u8; 2]>, focus_on_player: bool) -> Vec<[u8; 2]> {
	let cells_close: Vec<[u8; 2]> = available_cells_close(&grid, player, x, y, focus_on_player);
	let mut not_visited_cells_close: Vec<[u8; 2]> = Vec::with_capacity(cells_close.len());

	for cell in cells_close.iter() {
		if !has_pos_in_vector(&visited, *cell) {
			not_visited_cells_close.push(*cell);
		}
	}
	return not_visited_cells_close;
}

fn has_pos_in_vector(positions: &Vec<[u8; 2]>, search: [u8; 2]) -> bool {
	for pos in positions.iter() {
		if search == *pos {
			return true;
		}
	}
	return false;
}

fn revert_player(player: u8) -> u8 {
	if player == 1 { 2 } else if player == 2 { 1 } else { 0 }
}

fn available_cells_close(grid: &Vec<Vec<u8>>, player: u8, x: u8, y: u8, focus_on_player: bool) -> Vec<[u8; 2]> {
	let cells_close: Vec<[u8; 2]> = cells_close(&grid, x, y);
	let mut available_cells_close: Vec<[u8; 2]> = Vec::with_capacity(cells_close.len());

	let opp_player = revert_player(player);

	for cell in cells_close.iter() {
		let grid_value: u8 = grid[cell[0] as usize][cell[1] as usize];
		if (grid_value != opp_player && !focus_on_player) || (grid_value != opp_player && grid_value != 0)  {
			available_cells_close.push(*cell);
		}
	}
	return available_cells_close;
}

fn cells_close(grid: &Vec<Vec<u8>>, x: u8, y: u8) -> Vec<[u8; 2]> {
	let mut cells_values: Vec<[u8; 2]> = Vec::with_capacity(8);
	let grid_length: u8 = grid.len() as u8;
	let relative_positions: [[i8; 2]; 8] = [
		[-1, -1],	[-1, 0],	[-1, 1],
		[0, -1], 	/*0,0*/		[0, 1],
		[1, -1],	[1, 0],		[1, 1]];
	for relative_position in relative_positions.iter() {
		let new_i_x = x as i8 + relative_position[0];
		if new_i_x < 0 || new_i_x >= grid_length as i8 {
			continue;
		}
		let new_x = new_i_x as u8;

		let new_i_y = y as i8 + relative_position[1];
		if new_i_y < 0 || new_i_y >= grid_length as i8 {
			continue;
		}
		let new_y = new_i_y as u8;
		cells_values.push([new_x, new_y]);
	}
	return cells_values;
}


/*************************/
#[test]
#[should_fail]
fn test_two_grids_shuffled_must_not_be_equals() {
	let grid: Vec<Vec<u8>> = generate_grid(10u8);
	assert_eq!(get_shuffled_free_cells(&grid), get_shuffled_free_cells(&grid));
}

#[test]
fn test_winned_game() {
	let mut grid1: Vec<Vec<u8>> = generate_grid(10u8);
	let mut grid2: Vec<Vec<u8>> = generate_grid(10u8);
	for i in 0u8..10 {
		edit_grid(&mut grid1, 1, i, 0);
		edit_grid(&mut grid2, 2, 0, i);
	}
	assert!(has_winner_path(&grid1, 1));
	assert!(has_winner_path(&grid2, 2));
}

#[test]
#[should_fail]
fn test_not_winned_game() {
	let mut grid: Vec<Vec<u8>> = generate_grid(10u8);
	for i in 0u8..grid.len() as u8 {
		edit_grid(&mut grid, i % 2 + 1, 0, i);
	}
	assert!(has_winner_path(&grid, 1));
	assert!(has_winner_path(&grid, 2));
}

#[test]
fn test_cells_close() {
	let grid: Vec<Vec<u8>> = generate_grid(10u8);
	//Corner left top, only 3 cells are available
	assert_eq!(cells_close(&grid, 0, 0).len(), 3);
	//Corner right bottom, only 3 cells are available
	assert_eq!(cells_close(&grid, 9, 9).len(), 3);
	//Corner close to center, 8 cells are available
	assert_eq!(cells_close(&grid, 4, 4).len(), 8);
}
