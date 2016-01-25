extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;


use rustorm::platform::postgres::Postgres;
use rustorm::pool::ManagedPool;

use rustorm::table::IsTable;
use rustorm::dao::{IsDao,Value};
use rustorm::query::Query;
use rustorm::query::{Filter,Equality,Operand};
use rustorm::dao::Dao;
use gen::bazaar::Product;
use gen::bazaar::product;
use gen::bazaar::ProductAvailability;
use gen::bazaar::product_availability;
use gen::bazaar::ProductCategory;
use gen::bazaar::product_category;
use gen::bazaar::Category;
use gen::bazaar::category;
use gen::bazaar::ProductPhoto;
use gen::bazaar::product_photo;
use gen::bazaar::Photo;
use gen::bazaar::photo;

use rustorm::query::Function;
use rustorm::query::function::COUNT;
use rustorm::query::QueryBuilder;
use gen::bazaar;
use rustorm::query::ToTableName;
use rustorm::query::HasEquality;
use rustorm::query::order::HasDirection;
use rustorm::query::join::ToJoin;
use rustorm::query::builder::SELECT;


mod gen;


#[test]
fn test_complex(){
    let pg = Postgres::new();
    let frag = SELECT().ALL()
        .FROM(&bazaar::product)
        .LEFT_JOIN(bazaar::product_category
           .ON(product_category::product_id.EQ(&product::product_id)))
        .LEFT_JOIN(bazaar::category
            .ON(category::category_id.EQ(&product_category::category_id)))
        .LEFT_JOIN(bazaar::product_photo
            .ON(product::product_id.EQ(&product_photo::product_id)))
        .LEFT_JOIN(bazaar::photo 
            .ON(product_photo::photo_id.EQ(&photo::photo_id)))
        .WHERE(
				product::name.EQ_VALUE(&"GTX660 Ti videocard")
			.AND(category::name.EQ_VALUE(&"Electronic"))
		)
        .ORDER_BY(&[product::name.ASC(), product::created.DESC()])
        .GROUP_BY(&[category::name])
        .HAVING(COUNT(&"*").GT(&1))
    	.build(&pg);
    
    let expected = "
SELECT *
     FROM bazaar.product
          LEFT JOIN bazaar.product_category
          ON product_id = product.product_id 
          LEFT JOIN bazaar.category
          ON category_id = product_category.category_id 
          LEFT JOIN bazaar.product_photo
          ON product_id = product_photo.product_id 
          LEFT JOIN bazaar.photo
          ON photo_id = photo.photo_id 
    WHERE ( product.name = $1  AND category.name = $2   )
 GROUP BY (category.name) 
   HAVING  COUNT(*) > $3  
 ORDER BY product.name ASC, product.created DESC
 ".to_string();
    println!("actual:   {{{}}} [{}]", frag.sql, frag.sql.len());
    println!("expected: {{{}}} [{}]", expected, expected.len());
    assert_eq!(frag.sql.trim() , expected.trim());
}


