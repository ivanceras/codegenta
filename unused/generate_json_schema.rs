extern crate rustorm;
extern crate codegenta;

use codegenta::generator;
use codegenta::generator::Config;
use rustorm::pool::ManagedPool;
use codegenta::table_json::TableJson;
use std::fs;

/// this will generate needed model code for tests in ./examples/gen directory
fn main(){
    
    let pool = ManagedPool::init("postgres://postgres:p0stgr3s@localhost/bazaar_v8", 1).unwrap();
    let db = pool.connect().unwrap();
    let tables = generator::get_all_tables(db.as_dev());
	let base_dir = "./scripts/json_schema";
    match fs::create_dir_all(&base_dir) {
        Ok(_) => {
			for table in &tables{
				let json = table.to_json();
				println!("json: {}", json);
				let filename = format!("{}/{}.json",base_dir, table.name);
				generator::save_to_file(&filename, &json);
			}

        }
        Err(e) => println!("There was an error creating directory {}", e),
    };
}

