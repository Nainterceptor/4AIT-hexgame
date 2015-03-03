use grid::Grid;
use player::Player;
use player::PlayerType;
use cell::CellStatus;

#[test]
#[should_fail]
fn test_two_grids_shuffled_must_not_be_equals() {
	let grid: Grid = Grid::new(10u8);
	assert_eq!(grid.get_shuffled_free_cells(), grid.get_shuffled_free_cells());
}

//todo: revive this test
//#[test]
//fn test_winned_game() {
//	let grid1: Grid = Grid::new(10u8);
//	let grid2: Grid = Grid::new(10u8);
//	let players: [Player; 2] = [Player {
//		name: "Foo".to_string(),
//		cell_code: CellStatus::Red,
//		player_type: PlayerType::AI
//	}, Player {
//		name: "Bar".to_string(),
//		cell_code: CellStatus::Blue,
//		player_type: PlayerType::AI
//	}];
//	for i in 0u8..10 {
//		grid1.edit(&players[0], [i, 0]);
//		grid2.edit(&players[1], [0, i]);
//	}
//	assert!(grid1.has_winner_path(&players[0]));
//	assert!(grid2.has_winner_path(&players[1]));
//}

//todo: revive this test
//#[test]
//#[should_fail]
//fn test_not_winned_game() {
//let mut grid: Vec<Vec<u8>> = generate_grid(10u8);
//for i in 0u8..grid.len() as u8 {
//edit_grid(&mut grid, i % 2 + 1, 0, i);
//}
//assert!(has_winner_path(&grid, 1));
//assert!(has_winner_path(&grid, 2));
//}

#[test]
fn test_cells_close() {
	let grid: Grid = Grid::new(10u8);
	//Corner left top, only 3 cells are available
	assert_eq!(grid.cells_close([0, 0]).len(), 3);
	//Corner right bottom, only 3 cells are available
	assert_eq!(grid.cells_close([9, 9]).len(), 3);
	//Corner close to center, 8 cells are available
	assert_eq!(grid.cells_close([4, 4]).len(), 8);
}