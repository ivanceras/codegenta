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

use gen::bazaar;

use rustorm::table::IsTable;
use rustorm::pool::ManagedPool;
use rustorm::query::HasEquality;
use rustorm::query::QueryBuilder;
use rustorm::query::ToTableName;
use rustorm::query::function::COUNT;
use rustorm::query::order::HasDirection;
use rustorm::query::join::ToJoin;
use rustorm::query::operand::ToOperand;
use rustorm::query::builder::SELECT_ALL;

mod gen;

fn main(){
    let mut pool = ManagedPool::init("postgres://postgres:p0stgr3s@localhost/bazaar_v8",1).unwrap();
    let db = pool.connect().unwrap();
    
    let frag = SELECT_ALL().FROM(&bazaar::product)
        .LEFT_JOIN(bazaar::product_category
            .ON(
				 	product_category::product_id.EQ(&product::product_id)
            	.AND(product_category::product_id.EQ(&product::product_id))
				)
			)
         .LEFT_JOIN(bazaar::category
            .ON(category::category_id.EQ(&product_category::category_id)))
        .LEFT_JOIN(bazaar::product_photo
            .ON(product::product_id.EQ(&product_photo::product_id)))
        .LEFT_JOIN(bazaar::photo 
            .ON(product_photo::photo_id.EQ(&photo::photo_id)))
        .WHERE(
		 	product::name.EQ(&"GTX660 Ti videocard".to_owned())
			.AND(category::name.EQ(&"Electronic".to_owned()))
		)
        .GROUP_BY(&[category::name])
        .HAVING(COUNT(&"*").GT(&1))
        .HAVING(COUNT(&product::product_id).GT(&1))
        .ORDER_BY(&[product::name.ASC(), product::created.DESC()])
    	.build(db.as_ref());
   

    let expected = "
SELECT *
     FROM bazaar.product
          LEFT JOIN bazaar.product_category
          ON ( product_category.product_id = product.product_id AND product_category.product_id = product.product_id  )
          LEFT JOIN bazaar.category
          ON category.category_id = product_category.category_id 
          LEFT JOIN bazaar.product_photo
          ON product.product_id = product_photo.product_id 
          LEFT JOIN bazaar.photo
          ON product_photo.photo_id = photo.photo_id 
    WHERE ( product.name = $1  AND category.name = $2   )
 GROUP BY category.name 
   HAVING  COUNT(*) > $3  ,  COUNT(product.product_id) > $4  
 ORDER BY product.name ASC, product.created DESC

    ".to_string();
    println!("actual:   {{{}}} [{}]", frag.sql, frag.sql.len());
    println!("expected: {{{}}} [{}]", expected, expected.len());
    assert!(frag.sql.trim() == expected.trim());
}
