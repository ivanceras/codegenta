extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;


use rustorm::pool::ManagedPool;

use rustorm::em::EntityManager;
use gen::public::Exchangables;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use rustc_serialize::json::{self, ToJson, Json};


mod gen;

/// run using  cargo run --release --example test_em_insert_product
fn main(){
let pool = ManagedPool::init("postgres://postgres:p0stgr3s@localhost/bazaar_v7",1).unwrap();
    let db = pool.connect().unwrap();
    
    let em = EntityManager::new(db.as_ref());
    let item = Exchangables{
            data: Some("Ghost Chair".to_owned()),
            price: Some(99.99),
            seq_no: Some(10),
            url: Some("./photo1.png".to_owned()),
            priority: Some(10.01),
            photo_id: Uuid::new_v4(),
        };
    
    let item  = em.insert(&item);
    println!("item: {:#?}", item);
}