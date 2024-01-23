
mod blockchain;
mod utils;

fn main() {
	println!( "Hello, from main.rs!" );

	let mut block_freddie = blockchain::Block::new( "Freddie" );
	block_freddie.create_new_block( "Common octopus", "Octopus", "3" );
}
