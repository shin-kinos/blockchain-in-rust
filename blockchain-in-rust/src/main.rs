
mod blockchain;
mod utils;

fn main() {
	println!( "Hello, from main.rs!" );

	let mut block = blockchain::Block::new( "Freddie" );
	block.create_hash();
	//block.show_asset();
}
