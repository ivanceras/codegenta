extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use rustorm::query::Query;
use rustorm::query::{Filter,Equality};
use rustorm::dao::{Dao,IsDao};
use gen::bazaar::Product;
use gen::bazaar::product;
use gen::bazaar::Photo;
use gen::bazaar::photo;
use gen::bazaar::Review;
use gen::bazaar::review;
use gen::bazaar::Category;
use gen::bazaar::category;
use gen::bazaar::product_category;
use gen::bazaar::ProductCategory;
use gen::bazaar::product_photo;
use gen::bazaar::ProductPhoto;
use gen::bazaar::ProductAvailability;
use gen::bazaar::product_availability;

use rustorm::table::IsTable;
use rustorm::pool::ManagedPool;

mod gen;

fn main(){
    let mut pool = ManagedPool::init("postgres://postgres:p0stgr3s@localhost/bazaar_v6",1);
    let db = pool.connect().unwrap();
    
    let mut query = Query::select_all();
    
    query.from(&Product::table())
        .left_join(&ProductCategory::table(),
            product_category::product_id, product::product_id)
         .left_join(&Category::table(),
            category::category_id, product_category::category_id)
        .left_join(&ProductPhoto::table(),
            product::product_id, product_photo::product_id)
        .left_join(&Photo::table(), 
            product_photo::photo_id, photo::photo_id)
        .filter(product::name, Equality::EQ, &"GTX660 Ti videocard")
        .filter(category::name, Equality::EQ, &"Electronic")
        .group_by(vec![category::name])
        .having("count(*)", Equality::GT, &1)
        .asc(product::name)
        .desc(product::created)
        ;
    let frag = query.build(db.as_ref());
    
    let expected = "
SELECT *
     FROM bazaar.product
          LEFT OUTER JOIN bazaar.product_category 
          ON product_category.product_id = product.product_id 
          LEFT OUTER JOIN bazaar.category 
          ON category.category_id = product_category.category_id 
          LEFT OUTER JOIN bazaar.product_photo 
          ON product.product_id = product_photo.product_id 
          LEFT OUTER JOIN bazaar.photo 
          ON product_photo.photo_id = photo.photo_id 
    WHERE product.name = $1 
      AND category.name = $2 
 GROUP BY category.name 
   HAVING count(*) > $3 
 ORDER BY product.name ASC, product.created DESC
    ".to_string();
    println!("actual:   {{{}}} [{}]", frag.sql, frag.sql.len());
    println!("expected: {{{}}} [{}]", expected, expected.len());
    assert!(frag.sql.trim() == expected.trim());
}