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
    let mut db = pool.connect().unwrap();
    
    let mut query = Query::enumerate_all();
    
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
    let frag = query.build(db.as_mut());
    
    let expected = "
SELECT product.organization_id AS product_organization_id, product.client_id AS product_client_id, product.created AS product_created, 
    product.created_by AS product_created_by, product.updated AS product_updated, product.updated_by AS product_updated_by, product.priority AS product_priority, 
    product.name AS product_name, product.description AS product_description, product.help AS product_help, product.active AS product_active, 
    product.product_id AS product_product_id, product.parent_product_id, product.is_service, product.price, 
    product.use_parent_price, product.unit, product.tags, product.info, 
    product.seq_no AS product_seq_no, product.upfront_fee, product.barcode, product.owner_id, 
    product.currency_id, product_category.organization_id AS product_category_organization_id, product_category.client_id AS product_category_client_id, product_category.created AS product_category_created, 
    product_category.created_by AS product_category_created_by, product_category.updated AS product_category_updated, product_category.updated_by AS product_category_updated_by, product_category.priority AS product_category_priority, 
    product_category.product_id AS product_category_product_id, product_category.category_id AS product_category_category_id, category.organization_id AS category_organization_id, category.client_id AS category_client_id, 
    category.created AS category_created, category.created_by AS category_created_by, category.updated AS category_updated, category.updated_by AS category_updated_by, 
    category.priority AS category_priority, category.name AS category_name, category.description AS category_description, category.help AS category_help, 
    category.active AS category_active, category.category_id AS category_category_id, product_photo.organization_id AS product_photo_organization_id, product_photo.client_id AS product_photo_client_id, 
    product_photo.created AS product_photo_created, product_photo.created_by AS product_photo_created_by, product_photo.updated AS product_photo_updated, product_photo.updated_by AS product_photo_updated_by, 
    product_photo.priority AS product_photo_priority, product_photo.product_id AS product_photo_product_id, product_photo.photo_id AS product_photo_photo_id, photo.organization_id AS photo_organization_id, 
    photo.client_id AS photo_client_id, photo.created AS photo_created, photo.created_by AS photo_created_by, photo.updated AS photo_updated, 
    photo.updated_by AS photo_updated_by, photo.priority AS photo_priority, photo.name AS photo_name, photo.description AS photo_description, 
    photo.help AS photo_help, photo.active AS photo_active, photo.photo_id AS photo_photo_id, photo.url, 
    photo.data, photo.seq_no AS photo_seq_no
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
    ORDER BY product.name ASC, product.created DESC".to_string();
    println!("actual:   {{{}}} [{}]", frag.sql, frag.sql.len());
    println!("expected: {{{}}} [{}]", expected, expected.len());
    assert!(frag.sql.trim() == expected.trim());
}