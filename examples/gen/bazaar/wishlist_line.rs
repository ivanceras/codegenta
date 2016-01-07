//! WARNING: This file is generated, derived from table bazaar.wishlist_line, DO NOT EDIT

use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use gen::column;
use gen::schema;
use gen::table;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use rustorm::dao::Type;
use rustorm::dao::Value;
use rustorm::query::Operand;
use rustorm::table::Column;
use rustorm::table::Foreign;
use rustorm::table::IsTable;
use rustorm::table::Table;
use uuid::Uuid;
use gen::bazaar::Wishlist;



#[derive(RustcEncodable)]
#[derive(Debug, Clone)]
pub struct WishlistLine {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub wishlist_line_id: Uuid,
    /// default: 'false'
    /// db data type: boolean
    pub added_to_cart: Option<bool>,
    /// db data type: double precision
    pub price_momentary: Option<f64>,
    /// db data type: uuid
    pub product_id: Option<Uuid>,
    /// db data type: uuid
    pub wishlist_id: Option<Uuid>,
    /// default: 'true'
    /// not nullable 
    /// --inherited-- 
    /// db data type: boolean
    pub active: bool,
    /// --inherited-- 
    /// db data type: uuid
    pub client_id: Option<Uuid>,
    /// default: 'now()'
    /// not nullable 
    /// --inherited-- 
    /// db data type: timestamp with time zone
    pub created: DateTime<UTC>,
    /// --inherited-- 
    /// db data type: uuid
    pub created_by: Option<Uuid>,
    /// --inherited-- 
    /// db data type: character varying
    pub description: Option<String>,
    /// --inherited-- 
    /// db data type: text
    pub help: Option<String>,
    /// --inherited-- 
    /// db data type: character varying
    pub name: Option<String>,
    /// --inherited-- 
    /// db data type: uuid
    pub organization_id: Option<Uuid>,
    /// --inherited-- 
    /// db data type: double precision
    pub priority: Option<f64>,
    /// default: 'now()'
    /// not nullable 
    /// --inherited-- 
    /// db data type: timestamp with time zone
    pub updated: DateTime<UTC>,
    /// --inherited-- 
    /// db data type: uuid
    pub updated_by: Option<Uuid>,

    /// has one
    pub wishlist: Option<Wishlist>,
}



impl IsDao for WishlistLine {
    fn from_dao(dao: &Dao) -> Self {
        WishlistLine {
            organization_id: dao.get_opt(column::organization_id),
            client_id: dao.get_opt(column::client_id),
            created: dao.get(column::created),
            created_by: dao.get_opt(column::created_by),
            updated: dao.get(column::updated),
            updated_by: dao.get_opt(column::updated_by),
            priority: dao.get_opt(column::priority),
            name: dao.get_opt(column::name),
            description: dao.get_opt(column::description),
            help: dao.get_opt(column::help),
            active: dao.get(column::active),
            wishlist_id: dao.get_opt(column::wishlist_id),
            price_momentary: dao.get_opt(column::price_momentary),
            product_id: dao.get_opt(column::product_id),
            added_to_cart: dao.get_opt(column::added_to_cart),
            wishlist_line_id: dao.get(column::wishlist_line_id),
            wishlist: None,
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        match self.organization_id {
            Some(ref _value) => dao.set(column::organization_id, _value),
            None => dao.set_null(column::organization_id)
        }
        match self.client_id {
            Some(ref _value) => dao.set(column::client_id, _value),
            None => dao.set_null(column::client_id)
        }
        dao.set(column::created, &self.created);
        match self.created_by {
            Some(ref _value) => dao.set(column::created_by, _value),
            None => dao.set_null(column::created_by)
        }
        dao.set(column::updated, &self.updated);
        match self.updated_by {
            Some(ref _value) => dao.set(column::updated_by, _value),
            None => dao.set_null(column::updated_by)
        }
        match self.priority {
            Some(ref _value) => dao.set(column::priority, _value),
            None => dao.set_null(column::priority)
        }
        match self.name {
            Some(ref _value) => dao.set(column::name, _value),
            None => dao.set_null(column::name)
        }
        match self.description {
            Some(ref _value) => dao.set(column::description, _value),
            None => dao.set_null(column::description)
        }
        match self.help {
            Some(ref _value) => dao.set(column::help, _value),
            None => dao.set_null(column::help)
        }
        dao.set(column::active, &self.active);
        match self.wishlist_id {
            Some(ref _value) => dao.set(column::wishlist_id, _value),
            None => dao.set_null(column::wishlist_id)
        }
        match self.price_momentary {
            Some(ref _value) => dao.set(column::price_momentary, _value),
            None => dao.set_null(column::price_momentary)
        }
        match self.product_id {
            Some(ref _value) => dao.set(column::product_id, _value),
            None => dao.set_null(column::product_id)
        }
        match self.added_to_cart {
            Some(ref _value) => dao.set(column::added_to_cart, _value),
            None => dao.set_null(column::added_to_cart)
        }
        dao.set(column::wishlist_line_id, &self.wishlist_line_id);
        dao
    }
}

impl ToJson for WishlistLine {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for WishlistLine {

    fn default() -> Self {
        WishlistLine{
            organization_id: Default::default(),
            client_id: Default::default(),
            created: UTC::now(),
            created_by: Default::default(),
            updated: UTC::now(),
            updated_by: Default::default(),
            priority: Default::default(),
            name: Default::default(),
            description: Default::default(),
            help: Default::default(),
            active: Default::default(),
            wishlist_id: Default::default(),
            price_momentary: Default::default(),
            product_id: Default::default(),
            added_to_cart: Default::default(),
            wishlist_line_id: Default::default(),
            wishlist: Default::default(),
        }
    }
}

impl IsTable for WishlistLine {

    fn table() -> Table {
        Table {
            schema: Some(schema::bazaar.to_owned()),
            name: table::wishlist_line.to_owned(),
            parent_table: Some(table::record.to_owned()),
            sub_table: vec![],
            comment: None,
            columns: vec![
                Column {
                    name: column::organization_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::client_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::created.to_owned(),
                    data_type: Type::DateTime,
                    db_data_type: "timestamp with time zone".to_owned(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: true,
                    default: Some(Operand::Value(Value::String("'now()'".to_owned()))),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::created_by.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::updated.to_owned(),
                    data_type: Type::DateTime,
                    db_data_type: "timestamp with time zone".to_owned(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: true,
                    default: Some(Operand::Value(Value::String("'now()'".to_owned()))),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::updated_by.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::priority.to_owned(),
                    data_type: Type::F64,
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::name.to_owned(),
                    data_type: Type::String,
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::description.to_owned(),
                    data_type: Type::String,
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::help.to_owned(),
                    data_type: Type::String,
                    db_data_type: "text".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::active.to_owned(),
                    data_type: Type::Bool,
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: true,
                    default: Some(Operand::Value(Value::String("'true'".to_owned()))),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::wishlist_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: Some(
                        Foreign {
                            schema: Some("bazaar".to_owned()),
                            table: "wishlist".to_owned(),
                            column: vec!["wishlist_id".to_owned(),],
                        }),
                },
                Column {
                    name: column::price_momentary.to_owned(),
                    data_type: Type::F64,
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::product_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::added_to_cart.to_owned(),
                    data_type: Type::Bool,
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: Some(Operand::Value(Value::String("'false'".to_owned()))),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::wishlist_line_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
            ],
            is_view: false,
        }
    }
}
// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static organization_id: &'static str = "wishlist_line.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "wishlist_line.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "wishlist_line.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "wishlist_line.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "wishlist_line.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "wishlist_line.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "wishlist_line.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "wishlist_line.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "wishlist_line.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "wishlist_line.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "wishlist_line.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static wishlist_id: &'static str = "wishlist_line.wishlist_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static price_momentary: &'static str = "wishlist_line.price_momentary";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static product_id: &'static str = "wishlist_line.product_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static added_to_cart: &'static str = "wishlist_line.added_to_cart";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static wishlist_line_id: &'static str = "wishlist_line.wishlist_line_id";
