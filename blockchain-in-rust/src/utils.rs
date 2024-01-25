
use chrono::{ DateTime, Utc }; // For ISO time generation
use std::collections::HashMap; // For HashMap
use crate::blockchain::Block;  // For using Block structure
use sha256::digest;            // For generating hash

pub fn get_iso_timestamp() -> String {
	let now     : DateTime<Utc> = Utc::now();
	let now_iso : String        = now.to_rfc3339();
	//println!( "{}", now_iso );

	return now_iso;
}

pub fn create_hash( block : &Block ) -> String {
	// Stringfy HashMap
	let asset_str = stringfy_hashmap( &( ( *block ).asset ) );
	//println!( "{}", asset_str );

	// Create hash index
	let mut hash_index : String = "".to_string();
	hash_index.push_str( ( ( *block ).index ).to_string().as_str() );
	hash_index.push_str(                        asset_str.as_str() );
	hash_index.push_str(         ( ( *block ).timestamp ).as_str() );
	hash_index.push_str(     ( ( *block ).previous_hash ).as_str() );
	//println!( "{}", hash_index );

	// Create hash
	let hash : String = digest( hash_index );
	//println!( "Hash is created!: {}", hash );

	return hash;
}

fn stringfy_hashmap( hashmap : &HashMap<String, String> ) -> String {
	// Define asset composition
	let asset_composition : [ &str; 3 ] = [ "name", "type", "quantity" ];

	// Define last element of asset composition
	let last_element : &str = asset_composition[ asset_composition.len() - 1];

	// Start stringfy
	let mut hashmap_str : String = "{".to_string();
	for key in asset_composition {
		// Add double quotations and colon
		hashmap_str.push_str( "\""   );
		hashmap_str.push_str( key    );
		hashmap_str.push_str( "\": " );

		// Add double quotations and comma
		// If last element, dismiss
		let value = &( *hashmap )[ key ];
		if key == last_element {
			hashmap_str.push_str( "\""  );
			hashmap_str.push_str( value );
			hashmap_str.push_str( "\""  );
		} else {
			hashmap_str.push_str( "\""   );
			hashmap_str.push_str( value  );
			hashmap_str.push_str( "\", " );
		}
	}
	hashmap_str.push_str( "}" );

	//println!( "{}", hashmap_str );
	return hashmap_str;
}