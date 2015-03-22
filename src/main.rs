#![feature(old_io)]
#![feature(collections)]
#![feature(rand)]
#[warn(deprecated)]
extern crate test;

mod grid;
mod cell;
mod game;
mod player;

#[cfg(test)]
mod tests;

fn main() {
	let mut game: game::Game = game::Game::new();
	game.play();
}