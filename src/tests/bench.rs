use test::Bencher;
use cell::CellStatus;
use player::PlayerType;
use game::Game;

#[bench]
fn bench_random_ia(b: &mut Bencher) {
	b.iter(|| {
		let mut game: Game = Game::new_for_bench(11u8, PlayerType::RandomAI);
		game.play();
		return game;
	});
}

#[bench]
fn bench_path_ia(b: &mut Bencher) {
	b.iter(|| {
		let mut game: Game = Game::new_for_bench(11u8, PlayerType::PathAI);
		game.play();
		return game;
	});
}

#[bench]
fn bench_mind_path_ia(b: &mut Bencher) {
	b.iter(|| {
		let mut game: Game = Game::new_for_bench(11u8, PlayerType::MindPathAI);
		game.play();
		return game;
	});
}

#[bench]
fn bench_bruteforce_ia(b: &mut Bencher) {
	b.iter(|| {
		let mut game: Game = Game::new_for_bench(5u8, PlayerType::ButeforceAI);
		game.play();
		return game;
	});
}