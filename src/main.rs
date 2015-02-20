#![feature(io)]

use std::old_io;
//static ITERATIONS: u16 = 5000;
static DEFAULT_GRID: u8 = 10;

fn main() {
	let mut stdin = old_io::stdin();
	//Choose the grid size
	println!("Please give us grid size");
	let grid_input = stdin.lock().lines().next().unwrap().ok().unwrap();
	let grid_size: u8 = match grid_input.trim().parse::<u8>() {
			Ok(0) => DEFAULT_GRID,
			Ok(x) => x,
			Err(_) => DEFAULT_GRID
		};
	println!("Your grid must have a size of {}", grid_size);
	//Build the grid
	let mut grid: Vec<Vec<u8>> = generate_grid(grid_size);

	//Print the grid
	print_grid(grid);

}


//For performances considerations, sides can't be > to 255.
fn generate_grid(side_length: u8) -> Vec<Vec<u8>> {
	let mut vector: Vec<Vec<u8>> = Vec::new();
	for i in 0..side_length {
		let mut vector_internal: Vec<u8> = Vec::new();
		for j in 0..side_length {
			vector_internal.push(0);
		}
		vector.push(vector_internal);
	}
	return vector;
}


fn print_grid(grid: Vec<Vec<u8>>) {
	for x in grid.iter() {
		for y in x.iter() {
			print!(" {}", match y {
				&0u8 => "-",
				&1u8 => "B",
				&2u8 => "W",
				_ => "E"
			});
		}
		print!("\n");
	}
}