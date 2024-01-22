

use crate::utils;
use std::collections::HashMap; // For HashMap
use sha256::digest;            // For creating hash

pub struct Block {
	pub index         : usize,
	pub asset         : HashMap<String, String>,
	pub timestamp     : String,
	pub hash          : String,
	pub previous_hash : String,
}

impl Block {
	pub fn new( owner : &str ) -> Block {
		// Set index
		let index : usize = 0;

		// Initialise asset and its composition
		let mut asset  : HashMap<String, String> = HashMap::new();
		let asset_keys : Vec<String> = utils::define_asset_keys();
		for key in asset_keys { asset.insert( key, "".to_string() ); }

		// Set owner
		asset.insert( "Owner".to_string(), owner.to_string() );

		// Get timestamp
		let timestamp : String = utils::get_iso_timestamp();

		// Get hash (will be defined in create_hash() func!)
		let hash : String = "".to_string();

		// Get previous hash (but genesis block might not have any previous hash!)
		let previous_hash : String = "Genesis block".to_string();

		println!( "Genesis block for {} is created!", owner.to_string() );
		//println!( "{:?}", asset );

		Block {
			index         : index,
			asset         : asset,
			timestamp     : timestamp,
			hash          : hash,
			previous_hash : previous_hash,
		}
	}

	pub fn create_hash( &mut self ) {
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
		self.hash = digest( hash_index );
		println!( "Hash is created!: {}", self.hash );
	}

	/*
	pub fn show_asset( &self ) {
		println!( "{:?}", self.asset );
	}
	*/
}
