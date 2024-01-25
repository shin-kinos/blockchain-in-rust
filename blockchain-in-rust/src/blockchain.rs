

use crate::utils;
use std::collections::HashMap; // For HashMap
//use json::{object, JsonValue}; // For JSON parser
use sha256::digest;            // For generating hash

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
	pub fn new() -> Blockchain {
		// Create empty asset
		/*
		let asset : JsonValue = object![
			Name     : String::new(),
			Type     : String::new(),
			Quantity : String::new(),
		];
		*/

		// Create empty block
		let block : Block = Block {
			owner         : String::new(),
			index         : 0,
			asset         : HashMap::new(),
			timestamp     : String::new(),
			hash          : String::new(),
			previous_hash : String::new(),
		};

		// Create empty blockchain
		let mut blockchain : Vec<Block> = Vec::new();
		blockchain.push( block );

		return Blockchain { blockchain : blockchain, };
	}

	pub fn create_new_block(
		&mut self,
		asset_owner    : &str, // Asset owner
		asset_name     : &str, // Asset Name
		asset_type     : &str, // Asset Type
		asset_quantity : &str, // Asset Quantity
	) {
		// Create empty block
		let mut block = Block::new();

		// Define owner
		let owner : String = asset_owner.to_string();

		// Define initial index
		let index : usize = 1;

		// Initialise asset and its composition
		let mut asset  : HashMap<String, String> = HashMap::new();
		let asset_keys : Vec<String> = utils::define_asset_keys();
		for key in asset_keys { asset.insert( key, String::new() ); }

		// Set asset name
		asset.insert( "Name".to_string(), asset_name.to_string() );

		// Set asset type
		asset.insert( "Type".to_string(), asset_type.to_string() );

		// Set asset quantity
		asset.insert( "Quantity".to_string(), asset_quantity.to_string() );

		// Get timestamp
		let timestamp : String = utils::get_iso_timestamp();

		// Get previous hash (but genesis block might not have previous hash!)
		let previous_hash : String = "Genesis block".to_string();

		// Assign components into Block
		block.owner         = owner;
		block.index         = index;
		block.asset         = asset;
		block.timestamp     = timestamp;
		block.previous_hash = previous_hash;

		// Get current hash
		let hash : String = self.create_hash( &block );

		// Create a new block and assign component
		block.hash = hash;

		self.blockchain.push( block );

		println!( "Genesis block for {} is created!", *( &( ( *( self.blockchain ) )[ 1 ].owner ) ) );
		//println!( "{:?}", asset );
	}

	pub fn get_current_block( &self ) {
		// Index of last element blockchain
		let index_last = ( *self.blockchain ).len();

		// Get all element
		let current_block = &( ( *( self.blockchain ) )[ index_last - 1 ] );
		println!( "owner: {}", ( *current_block ).owner                          );
		println!( "Index: {}", ( *current_block ).index                          );
		println!( "Asset:"                                                       );
		println!( "    Name: {}", ( ( *current_block ).asset )[ "Name" ]         );
		println!( "    Type: {}", ( ( *current_block ).asset )[ "Type" ]         );
		println!( "    Quantity: {}", ( ( *current_block ).asset )[ "Quantity" ] );
		println!( "Timestamp: {}", ( *current_block ).timestamp                  );
		println!( "Hash: {}", ( *current_block ).hash                            );
		println!( "Previous hash: {}", ( *current_block ).previous_hash          );
	}

	fn create_hash( &self, block : &Block ) -> String {
		// Stringfy HashMap
		let asset_str = utils::stringfy_hashmap( &( ( *block ).asset ) );
		//println!( "{}", asset_str );

		// Create hash index
		let mut hash_index : String = "".to_string();
		hash_index.push_str( ( ( *block ).index ).to_string().as_str() );
		hash_index.push_str(                        asset_str.as_str() );
		hash_index.push_str(         ( ( *block ).timestamp ).as_str() );
		hash_index.push_str(     ( ( *block ).previous_hash ).as_str() );
		println!( "{}", hash_index );

		// Create hash
		let hash : String = digest( hash_index );
		println!( "Hash is created!: {}", hash );

		return hash;
	}

	/*
	pub fn show_asset( &self ) {
		println!( "{:?}", self.asset );
	}
	*/
}
