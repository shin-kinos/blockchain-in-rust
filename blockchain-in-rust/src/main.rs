
mod blockchain;
mod utils;

fn main() {
	println!( "Hello, from main.rs!" );

	let mut blockchain_freddie = blockchain::Blockchain::new();
	blockchain_freddie.create_new_block( "Freddie", "Common octopus", "Octopus", "3" );
	blockchain_freddie.get_current_block();
}
