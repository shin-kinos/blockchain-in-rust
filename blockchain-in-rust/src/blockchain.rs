

use crate::utils;
use std::collections::HashMap; // For HashMap

// For colourise asset info
static GREEN : &str = "\x1b[32m";
static RED   : &str = "\x1b[31m";
static RESET : &str = "\x1b[0m";

pub struct Block {
	pub owner         : String,
	pub index         : usize,
	pub asset         : HashMap<String, String>,
	pub timestamp     : String,
	pub hash          : String,
	pub previous_hash : String,
}

impl Block {
	fn new() -> Block {
		return Block {
			owner         : String::new(),
			index         : 0,
			asset         : HashMap::new(),
			timestamp     : String::new(),
			hash          : String::new(),
			previous_hash : String::new(),
		};
	}
}

pub struct Blockchain {
	pub blockchain : Vec<Block>,
}

impl Blockchain {
	pub fn create_genesis_block(
		asset_owner    : &str, // Asset owner
		asset_name     : &str, // Asset Name
		asset_type     : &str, // Asset Type
		asset_quantity : &str, // Asset Quantity
	) -> Blockchain {
		// Create empty block and blockchain
		let mut block      : Block      = Block::new();
		let mut blockchain : Vec<Block> = Vec::new();

		// Define owner
		let owner : String = asset_owner.to_string();

		// Define initial index
		let index : usize = 1;

		// Initialise asset and its composition
		let mut asset  : HashMap<String, String> = HashMap::new();

		// Set asset name
		asset.insert( "name".to_string(), asset_name.to_string() );

		// Set asset type
		asset.insert( "type".to_string(), asset_type.to_string() );

		// Set asset quantity
		asset.insert( "quantity".to_string(), asset_quantity.to_string() );

		// Get timestamp
		let timestamp : String = utils::get_iso_timestamp();

		// Get previous hash (but genesis block might not have previous hash!)
		let previous_hash : String = "genesis-block".to_string();

		// Assign components into Block
		block.owner         = owner;
		block.index         = index;
		block.asset         = asset;
		block.timestamp     = timestamp;
		block.previous_hash = previous_hash;

		// Get current hash
		let hash : String = utils::create_hash( &block );

		// Create a new block and assign component
		block.hash = hash;

		// push genesis block into blockchain
		blockchain.push( block );

		println!( "Genesis block is {}successfully{} created!\n", GREEN, RESET );

		return Blockchain { blockchain : blockchain, };
	}

	pub fn show_current_block( &self ) {
		// Index of last element blockchain
		let index_last = ( self.blockchain ).len();

		// Get all element
		let current_block : &Block = &( ( self.blockchain )[ index_last - 1 ] );
		println!( "{}Owner{}: {}",         GREEN, RESET, (   *current_block ).owner                 );
		println!( "{}Index{}: {}",         GREEN, RESET, (   *current_block ).index                 );
		println!( "{}Asset{}:",            GREEN, RESET                                             );
		println!( "    {}Name{}: {}",      GREEN, RESET, ( ( *current_block ).asset )[ "name" ]     );
		println!( "    {}Type{}: {}",      GREEN, RESET, ( ( *current_block ).asset )[ "type" ]     );
		println!( "    {}Quantity{}: {}",  GREEN, RESET, ( ( *current_block ).asset )[ "quantity" ] );
		println!( "{}Timestamp{}: {}",     GREEN, RESET, (   *current_block ).timestamp             );
		println!( "{}Hash{}: {}",          GREEN, RESET, (   *current_block ).hash                  );
		println!( "{}Previous hash{}: {}", GREEN, RESET, (   *current_block ).previous_hash         );
	}

	pub fn show_all_blocks( &self ) {
		let mut block_number : usize = 1;
		for block in ( self.blockchain ).iter() {
			println!( "Block {}:", block_number                                                       );
			println!( "    {}Owner{}: {}",           GREEN, RESET, (   *block ).owner                 );
			println!( "    {}Index{}: {}",           GREEN, RESET, (   *block ).index                 );
			println!( "    {}Asset{}:",              GREEN, RESET                                     );
			println!( "        {}Name{}: {}",        GREEN, RESET, ( ( *block ).asset )[ "name" ]     );
			println!( "        {}Type{}: {}",        GREEN, RESET, ( ( *block ).asset )[ "type" ]     );
			println!( "        {}Quantity{}: {}",    GREEN, RESET, ( ( *block ).asset )[ "quantity" ] );
			println!( "    {}Timestamp{}: {}",       GREEN, RESET, (   *block ).timestamp             );
			println!( "    {}Hash{}: {}",            GREEN, RESET, (   *block ).hash                  );
			println!( "    {}Previous hash{}: {}\n", GREEN, RESET, (   *block ).previous_hash         );
			block_number = block_number + 1;
		}
	}

	pub fn update_asset( &mut self, which : &str, update_to : &str ) {
		// Create new block
		let mut new_block = Block::new();

		// Get current block index
		let current_block_index = ( self.blockchain ).len() - 1 ;

		// Get current block 
		let current_block : &Block = &( ( self.blockchain  )[ current_block_index ] );

		// Get owner
		let owner : String = ( current_block.owner ).to_string();

		// Update index
		let index : usize  = ( ( *current_block ).index ) + 1;

		// Set and update asset composition
		let mut asset : HashMap<String, String> = ( ( *current_block ).asset ).clone();
		match which {
			"name"     => asset.insert(     "name".to_string(), update_to.to_string() ),
			"type"     => asset.insert(     "type".to_string(), update_to.to_string() ),
			"quantity" => asset.insert( "quantity".to_string(), update_to.to_string() ),
			_          => panic!( "{}Invalid argument in update_asset( &str, &str ) function!{}", RED, RESET ),
		};

		// Update timestamp
		let timestamp : String = utils::get_iso_timestamp();

		// Move current hash to previous hash
		let previous_hash : String = ( current_block.hash ).to_string();

		// Set all components into block
		new_block.owner         = owner;
		new_block.index         = index;
		new_block.asset         = asset;
		new_block.timestamp     = timestamp;
		new_block.previous_hash = previous_hash;

		// Create new hash
		let hash : String = utils::create_hash( &new_block );

		// Assign new hash
		new_block.hash = hash;

		// Push new block into blockchain
		( self.blockchain ).push( new_block );

		println!( "The asset was {}successfully{} updated!\n", GREEN, RESET );
	}

	pub fn check_chain_valid( &self ) {
		let mut check_result : bool = true;

		for i in 1 .. ( self.blockchain ).len() {
			// Get current and previous blocks
			let current_block  : &Block = &( ( self.blockchain )[ i ]     );
			let previous_block : &Block = &( ( self.blockchain )[ i - 1 ] );

			// Get possessed hash, and calculate hash in current block
			let current_hash_possess   : String  = ( current_block.hash ).to_string();
			let current_hash_calculate : String  = utils::create_hash( current_block );

			// If different, let's say "Don't falsify, criminal!"
			if current_hash_possess != current_hash_calculate {
				check_result = false;
				break;
			}

			// Get previous hash in current block, and current hash in previous block
			let previous_hash_in_current : String = ( current_block.previous_hash ).to_string();
			let current_hash_in_previous : String = ( previous_block.hash ).to_string();

			// If different, let's say "Don't falsify, criminal!"
			if previous_hash_in_current != current_hash_in_previous {
				check_result = false;
				break;
			}
		}

		println!( "Valid?: {}\n", check_result );
	}
}
