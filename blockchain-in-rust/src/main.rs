
mod blockchain;
mod utils;

fn main() {
	// Create Genesis block for Freddie
	let mut blockchain_freddie = blockchain::Blockchain::create_genesis_block(
		"Freddie",        // Owner
		"Common octopus", // Asset name
		"Octopus",        // Asset type
		"3"               // Asset quantity
	);
	//blockchain_freddie.show_current_block();

	// Update asset name into Cononut octopus
	blockchain_freddie.update_asset( "name", "Webfoot octopus" );
	//blockchain_freddie.show_all_blocks();

	// Update assset quantity into 5
	blockchain_freddie.update_asset( "quantity", "5" );
	blockchain_freddie.show_all_blocks();
	blockchain_freddie.check_chain_valid();

	// Let's try to falsify!
	/*
	println!( "\nNow try to falsify the quantity into 25..." );
	let last_block_elem = ( blockchain_freddie.blockchain ).len() - 1;
	( ( ( blockchain_freddie.blockchain )[ last_block_elem ] )
		.asset )
			.insert(
				"quantity".to_string(),
				"25".to_string()
			);
	blockchain_freddie.show_current_block();
	blockchain_freddie.check_chain_valid();
	*/


}
