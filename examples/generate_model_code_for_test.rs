extern crate rustorm;
extern crate codegenta;

use codegenta::generator;
use codegenta::generator::Config;
use rustorm::pool::ManagedPool;
//use rustorm::database::DatabaseDev;

/// this will generate needed model code for tests in ./examples/gen directory
fn main(){
    
    let pool = ManagedPool::init("postgres://postgres:p0stgr3s@localhost/bazaar_v6", 1).unwrap();
    let db = pool.connect().unwrap();
    let config =  Config{
            base_module:Some("gen".to_string()),
            include_table_references:true,
            use_condensed_name:true,
            generate_table_meta:true,
            base_dir:"./tests".to_string(),
            include_views: true
        };
    generator::generate_all(db.as_dev(), &config);
}

