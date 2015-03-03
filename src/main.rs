#![feature(old_io)]
#![feature(collections)]
#![feature(rand)]

mod grid;
mod cell;
mod game;
mod player;

#[cfg(test)]
mod test;

fn main() {
	let mut game: game::Game = game::Game::new();
	game.play();
}