extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;


use rustorm::pool::ManagedPool;

use rustorm::em::EntityManager;
use gen::bazaar::Product;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use rustc_serialize::json::{self, ToJson, Json};


mod gen;

/// run using  cargo run --release --example test_em_insert_product
fn main(){
let pool = ManagedPool::init("postgres://postgres:p0stgr3s@localhost/bazaar_v6",1).unwrap();
    let db = pool.connect().unwrap();
    
    let em = EntityManager::new(db.as_ref());
    let product = Product{
            name: Some("Ghost Chair".to_owned()),
            description: Some("A transparent chair".to_owned()),
            organization_id: Some(Uuid::new_v4()),
            client_id: Some(Uuid::new_v4()),
            created: UTC::now(),
            created_by: Some(Uuid::new_v4()),
            updated: UTC::now(),
            updated_by: Some(Uuid::new_v4()),
            priority: None,
            help: Some("A transparent chair".to_owned()),
            active: true,
            product_id: Uuid::new_v4(),
            parent_product_id: Some(Uuid::new_v4()),
            is_service: Some(false),
            price: Some(100.00),
            use_parent_price: Some(false),
            unit: Some("each".to_owned()),
            tags: None, //Some(Json::String("glass".to_owned())),
            info: None, //Some(Json::String("transparent glass chair".to_owned())),
            seq_no: None,
            upfront_fee: None,
            barcode: None,
            owner_id: Some(Uuid::new_v4()),
            currency_id: Some(Uuid::new_v4()),
            owner: None,
            currency: None,
            availability: None,
            category: vec![],
            photo: vec![],
            review: vec![],
        };
//    let product: Product = Default::default();
    
    let product  = em.insert(&product);
    println!("product: {:#?}", product);
}