extern crate rustorm;
extern crate codegenta;

use rustorm::query::Query;
use rustorm::query::{Filter,Equality};
use rustorm::pool::ManagedPool;
use rustorm::database::Database;

use codegenta::generator;

// run using cargo run --release --features sqlite --example sqlite_get_tabledef
fn main(){
    let url = "sqlite:///home/lee/Dropbox/git-src/ivanceras/rustorm/file.db";
    let mut pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();
    
    //let table = db.as_dev().get_table_metadata("","product_availability", false);
    let all_tables = generator::get_all_tables(db.as_dev());
    
    println!("all_tables: {:#?}", all_tables);
}
