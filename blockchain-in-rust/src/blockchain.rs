

use crate::utils;
use std::collections::HashMap; // For HashMap
use sha256::digest;            // For generating hash

/*
pub struct Blockchain {
	pub blockchain : something,
}
*/

pub struct Block {
	pub owner         : String,
	pub index         : usize,
	pub asset         : HashMap<String, String>,
	pub timestamp     : String,
	pub hash          : String,
	pub previous_hash : String,
}

impl Block {
	pub fn new( owner : &str ) -> Block {
		return Block {
			owner         : owner.to_string(),
			index         : 0,
			asset         : HashMap::new(),
			timestamp     : String::new(),
			hash          : String::new(),
			previous_hash : String::new(),
		};
	}

	pub fn create_new_block(
		&mut self,
		asset_name     : &str, // Asset Name
		asset_type     : &str, // Asset Type
		asset_quantity : &str, // Asset Quantity
	) {
		// Set initial index
		self.index = 1;

		// Initialise asset and its composition
		let asset_keys : Vec<String> = utils::define_asset_keys();
		for key in asset_keys { ( self.asset ).insert( key, String::new() ); }

		// Set asset name
		( self.asset ).insert( "Name".to_string(), asset_name.to_string() );

		// Set asset type
		( self.asset ).insert( "Type".to_string(), asset_type.to_string() );

		// Set asset quantity
		( self.asset ).insert( "Quantity".to_string(), asset_quantity.to_string() );

		// Get timestamp
		self.timestamp = utils::get_iso_timestamp();

		// Get previous hash (but genesis block might not have previous hash!)
		self.previous_hash = "Genesis block".to_string();

		// Get current hash
		self.hash = self.create_hash();

		println!( "Genesis block for {} is created!", self.owner );
		//println!( "{:?}", asset );
	}

	fn create_hash( &self ) -> String {
		// Stringfy HashMap
		let asset_str = utils::stringfy_hashmap( &( self.asset ) );
		//println!( "{}", asset_str );

		// Create hash index
		let mut hash_index : String = "".to_string();
		hash_index.push_str( ( self.index ).to_string().as_str() );
		hash_index.push_str(                  asset_str.as_str() );
		hash_index.push_str(         ( self.timestamp ).as_str() );
		hash_index.push_str(     ( self.previous_hash ).as_str() );
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
