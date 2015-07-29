# Codegenta

![](https://raw.githubusercontent.com/ivanceras/codegenta/master/kutsinta.png)

[![Latest Version](https://img.shields.io/crates/v/codegenta.svg)](https://crates.io/crates/codegenta)
[![Build Status](https://api.travis-ci.org/ivanceras/codegenta.svg)](https://travis-ci.org/ivanceras/codegenta)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Generate source code for your models which corresponds from the database tables.

Codegenta complements well with rustorm to provide a more ergonomic way of manipulating data


## Example

### Terminology and tables
* Product table holds the list of products in our example app.
* Users is a user table (plural due to user being a reserved keyword in most databases).
* Category - categories of products, items etc.
* Photos - product/item listing would be much more pleasing if it has pictures.
* Reviews - a review of users for a certain product.
* ProductAvailability - determines office time of the seller when a certain product is availability. Anouncement of product before it is available. This is much more applicable to services.

## Code to look at
* Take a look at examples/generate_model_code.rs. This generates a source code that is comparible to what you could have written as a model code of your project.

* The source code generated is located at `./gen` folder

```sql


CREATE TABLE bazaar.product
(
  product_id uuid NOT NULL DEFAULT uuid_generate_v4(),
  name character varying,
  description character varying,
  price numeric,
  currency_id uuid,
  unit character varying,
  barcode character varying,
  owner_id uuid,
  currency_id uuid,
  CONSTRAINT product_pkey PRIMARY KEY (product_id),
  CONSTRAINT product_currency_id_fkey FOREIGN KEY (currency_id)
      REFERENCES payment.currency (currency_id) MATCH SIMPLE
      ON UPDATE CASCADE ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED,
  CONSTRAINT product_user_id_fkey FOREIGN KEY (owner_id)
      REFERENCES bazaar.users (user_id) MATCH SIMPLE
      ON UPDATE CASCADE ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED
)

```

The generated model code

```rust


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Product {
    pub product_id:Uuid,
    pub name:Option<String>,
    pub description:Option<String>,
    pub barcode:Option<String>,
    pub currency_id:Option<Uuid>,
    pub owner_id:Option<Uuid>,
    pub price:Option<f64>,
    pub unit:Option<String>,

    pub owner: Option<Users>,
    pub currency: Option<Currency>,
    pub availability: Option<Box<ProductAvailability>>,
    pub category: Vec<Category>,
    pub photo: Vec<Photo>,
    pub review: Vec<Review>,
}

```

* `pub product_id:Uuid,` marked the column product as `NOT NULL` therefore it will always have value.

* `pub name:Option<String>` is an option since, we did not specify that this non nullable. Same as `descrption` and etc.

* `pub owner: Option<Users>,` base on the foreign key constraint, codegenta is smart enough to recognize that a product has an `owner` based on the `owner_id` which references `Users` table. Codegenta then add an optional field owner:Option<Users>, which you can then later use in you app to hold an additional info about the seller of a certain product.


* `pub currency: Option<Currency>`, `currency_id` specifies which currency a product is using. You can put the used currency in the product in your controller code to include a more detailed information about the currency used without having make additional container structs.

## More advance features
Take a look at the table schema used in these examples provided by the project.

* `pub category: Vec<Category>,` - the codegenta is also smart to recognize that product table is referred by `Category` table with a linker table product_category,
which provides a 1:M relationship between product and category, since products can have multiple categories

* `pub photo: Vec<Photo>`,
* `pub review: Vec<Review>`,

Same applies for Photo, with linker table `product_photo`
and Review table with linker `product_review`


* `pub availability: Option<Box<ProductAvailability>>`,  - product availability is another unique feature of codegenta that determines that ProductAvailability table is just an extension table of product and has a 1:1 relationship, since each product can only have 1 product availability.


## A complex query when using codegenta
[link](https://github.com/ivanceras/codegenta/blob/master/examples/complex_query.rs)
```rust

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
```

Look at those pretty generated SQL's

## Roadmap

* Support for the other way around?
*    The user writes the model and create a corresponding table in the database
