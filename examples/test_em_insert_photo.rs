extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;


use rustorm::pool::ManagedPool;

use rustorm::em::EntityManager;
use gen::bazaar::Photo;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use rustc_serialize::json::{self, ToJson, Json};


mod gen;

/// run using  cargo run --release --example test_em_insert_product
fn main(){
let pool = ManagedPool::init("postgres://postgres:p0stgr3s@localhost/bazaar_v7",1).unwrap();
    let db = pool.connect().unwrap();
    
    let em = EntityManager::new(db.as_ref());
    let photo = Photo{
            name: Some("Ghost Chair".to_owned()),
            data: None,
            seq_no: Some(10),
            url: Some("./photo1.png".to_owned()),
            description: Some("A transparent chair".to_owned()),
            organization_id: Some(Uuid::new_v4()),
            client_id: Some(Uuid::new_v4()),
            created: UTC::now(),
            created_by: Some(Uuid::new_v4()),
            updated: UTC::now(),
            updated_by: Some(Uuid::new_v4()),
            priority: Some(10.01),
            help: Some("A transparent chair".to_owned()),
            active: true,
            photo_id: Uuid::new_v4(),
            photo_sizes: vec![],
            user_info: vec![],
            product: vec![],
        };
    
    let photo  = em.insert(&photo);
    println!("photo: {:#?}", photo);
}