extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;


use rustorm::pool::ManagedPool;

use rustorm::em::EntityManager;
use gen::bazaar::Product;


mod gen;

#[test]
fn test_em(){
    let pool = ManagedPool::init("postgres://postgres:p0stgr3s@localhost/bazaar_v6",1).unwrap();
    let db = pool.connect().unwrap();
    
    let em = EntityManager::new(db.as_ref());
    let products  = em.get_all::<Product>();
    println!("products: {:#?}", products);
    
    assert!(products.unwrap().len() >  0 );
}