
use chrono::{ DateTime, Utc }; // For ISO time generation
use std::collections::HashMap; // For HashMap

const ASSET_COMPOSITION : [ &str; 3 ] = [ "Name", "Type", "Quantity" ];

pub fn define_asset_keys() -> Vec<String> {
	let mut keys : Vec<String> = Vec::new();
	for key in ASSET_COMPOSITION { keys.push( key.to_string() ); }

	return keys;
}

pub fn get_iso_timestamp() -> String {
	let now     : DateTime<Utc> = Utc::now();
	let now_iso : String        = now.to_rfc3339();
	//println!( "{}", now_iso );

	return now_iso;
}

pub fn stringfy_hashmap( hashmap : &HashMap<String, String> ) -> String {
	// Define last element of asset composition
	let last_element : &str = ASSET_COMPOSITION[ ASSET_COMPOSITION.len() - 1];

	// Start stringfy
	let mut hashmap_str = "{".to_string();
	for key in ASSET_COMPOSITION {
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
			hashmap_str.push_str( "\""  );
			hashmap_str.push_str( value );
			hashmap_str.push_str( "\", ");
		}
	}
	hashmap_str.push_str( "}" );

	//println!( "{}", hashmap_str );
	return hashmap_str;
}