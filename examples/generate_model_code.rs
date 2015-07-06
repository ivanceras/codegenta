extern crate rustorm as orm;
extern crate codegenta as codegen;

use orm::platform::postgres::Postgres;
use codegen::generator;
use codegen::generator::Config;

/// this will generate needed model code for tests in ./examples/gen directory
fn main(){
    let pg = Postgres::connect_with_url("postgres://postgres:p0stgr3s@localhost/bazaar_v6").unwrap();
    let config =  Config{
            base_module:Some("gen".to_string()),
            include_table_references:true,
            use_condensed_name:true,
            generate_table_meta:true,
            base_dir:"./examples".to_string(),
        };
    generator::generate_all(&pg, &config);
}

