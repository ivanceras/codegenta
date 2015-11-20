//! WARNING: This file is generated, derived from table bazaar.orders, DO NOT EDIT

use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use gen::bazaar::OrderLine;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use gen::schema;
use gen::table;
use gen::column;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Table;
use rustc_serialize::json::ToJson;
use rustc_serialize::json::Json;



#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Orders {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub order_id: Uuid,
    /// db data type: numeric
    pub amount_refunded: Option<f64>,
    /// db data type: numeric
    pub amount_tendered: Option<f64>,
    /// The cart from which this order was created from
    /// db data type: uuid
    pub cart_id: Option<Uuid>,
    /// default: 0.00
    /// db data type: numeric
    pub charges_amount: Option<f64>,
    /// For recognization purposes, this is the name shown to the seller
    /// db data type: character varying
    pub customer_name: Option<String>,
    /// db data type: timestamp with time zone
    pub date_approved: Option<DateTime<UTC>>,
    /// db data type: timestamp with time zone
    pub date_invoiced: Option<DateTime<UTC>>,
    /// default: now()
    /// db data type: timestamp with time zone
    pub date_ordered: Option<DateTime<UTC>>,
    /// db data type: numeric
    pub grand_total_amount: Option<f64>,
    /// if the order from the buyer is approved by the seller
    /// default: false
    /// db data type: boolean
    pub is_approved: Option<bool>,
    /// determined whether the order has been confirmed by the person who ordered it
    /// default: false
    /// db data type: boolean
    pub is_confirmed: Option<bool>,
    /// default: false
    /// db data type: boolean
    pub is_invoiced: Option<bool>,
    /// default: true
    /// db data type: boolean
    pub is_tax_included: Option<bool>,
    /// default: false
    /// db data type: boolean
    pub processed: Option<bool>,
    /// default: false
    /// db data type: boolean
    pub processing: Option<bool>,
    /// db data type: integer
    pub total_items: Option<i32>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    /// db data type: boolean
    pub active: bool,
    /// --inherited-- 
    /// db data type: uuid
    pub client_id: Option<Uuid>,
    /// default: now()
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
    /// db data type: numeric
    pub priority: Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    /// db data type: timestamp with time zone
    pub updated: DateTime<UTC>,
    /// --inherited-- 
    /// db data type: uuid
    pub updated_by: Option<Uuid>,

    /// has many
    pub order_line: Vec<OrderLine>,
}



impl IsDao for Orders {
    fn from_dao(dao: &Dao) -> Self {
        Orders {
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
            order_id: dao.get(column::order_id),
            customer_name: dao.get_opt(column::customer_name),
            total_items: dao.get_opt(column::total_items),
            grand_total_amount: dao.get_opt(column::grand_total_amount),
            charges_amount: dao.get_opt(column::charges_amount),
            processing: dao.get_opt(column::processing),
            processed: dao.get_opt(column::processed),
            is_confirmed: dao.get_opt(column::is_confirmed),
            is_tax_included: dao.get_opt(column::is_tax_included),
            date_ordered: dao.get_opt(column::date_ordered),
            is_invoiced: dao.get_opt(column::is_invoiced),
            date_invoiced: dao.get_opt(column::date_invoiced),
            is_approved: dao.get_opt(column::is_approved),
            date_approved: dao.get_opt(column::date_approved),
            amount_tendered: dao.get_opt(column::amount_tendered),
            amount_refunded: dao.get_opt(column::amount_refunded),
            cart_id: dao.get_opt(column::cart_id),
            order_line: vec![],
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
        dao.set(column::order_id, &self.order_id);
        match self.customer_name {
            Some(ref _value) => dao.set(column::customer_name, _value),
            None => dao.set_null(column::customer_name)
        }
        match self.total_items {
            Some(ref _value) => dao.set(column::total_items, _value),
            None => dao.set_null(column::total_items)
        }
        match self.grand_total_amount {
            Some(ref _value) => dao.set(column::grand_total_amount, _value),
            None => dao.set_null(column::grand_total_amount)
        }
        match self.charges_amount {
            Some(ref _value) => dao.set(column::charges_amount, _value),
            None => dao.set_null(column::charges_amount)
        }
        match self.processing {
            Some(ref _value) => dao.set(column::processing, _value),
            None => dao.set_null(column::processing)
        }
        match self.processed {
            Some(ref _value) => dao.set(column::processed, _value),
            None => dao.set_null(column::processed)
        }
        match self.is_confirmed {
            Some(ref _value) => dao.set(column::is_confirmed, _value),
            None => dao.set_null(column::is_confirmed)
        }
        match self.is_tax_included {
            Some(ref _value) => dao.set(column::is_tax_included, _value),
            None => dao.set_null(column::is_tax_included)
        }
        match self.date_ordered {
            Some(ref _value) => dao.set(column::date_ordered, _value),
            None => dao.set_null(column::date_ordered)
        }
        match self.is_invoiced {
            Some(ref _value) => dao.set(column::is_invoiced, _value),
            None => dao.set_null(column::is_invoiced)
        }
        match self.date_invoiced {
            Some(ref _value) => dao.set(column::date_invoiced, _value),
            None => dao.set_null(column::date_invoiced)
        }
        match self.is_approved {
            Some(ref _value) => dao.set(column::is_approved, _value),
            None => dao.set_null(column::is_approved)
        }
        match self.date_approved {
            Some(ref _value) => dao.set(column::date_approved, _value),
            None => dao.set_null(column::date_approved)
        }
        match self.amount_tendered {
            Some(ref _value) => dao.set(column::amount_tendered, _value),
            None => dao.set_null(column::amount_tendered)
        }
        match self.amount_refunded {
            Some(ref _value) => dao.set(column::amount_refunded, _value),
            None => dao.set_null(column::amount_refunded)
        }
        match self.cart_id {
            Some(ref _value) => dao.set(column::cart_id, _value),
            None => dao.set_null(column::cart_id)
        }
        dao
    }
}

impl ToJson for Orders {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for Orders {

    fn default() -> Self {
        Orders{
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
            order_id: Default::default(),
            customer_name: Default::default(),
            total_items: Default::default(),
            grand_total_amount: Default::default(),
            charges_amount: Default::default(),
            processing: Default::default(),
            processed: Default::default(),
            is_confirmed: Default::default(),
            is_tax_included: Default::default(),
            date_ordered: Default::default(),
            is_invoiced: Default::default(),
            date_invoiced: Default::default(),
            is_approved: Default::default(),
            date_approved: Default::default(),
            amount_tendered: Default::default(),
            amount_refunded: Default::default(),
            cart_id: Default::default(),
            order_line: Default::default(),
        }
    }
}

impl IsTable for Orders {

    fn table() -> Table {
        Table {
            schema: schema::bazaar.to_owned(),
            name: table::orders.to_owned(),
            parent_table: Some(table::record.to_owned()),
            sub_table: vec![],
            comment: None,
            columns: vec![
                Column {
                    name: column::organization_id.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::client_id.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::created.to_owned(),
                    data_type: "DateTime<UTC>".to_owned(),
                    db_data_type: "timestamp with time zone".to_owned(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: true,
                    default: Some("now()".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::created_by.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::updated.to_owned(),
                    data_type: "DateTime<UTC>".to_owned(),
                    db_data_type: "timestamp with time zone".to_owned(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: true,
                    default: Some("now()".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::updated_by.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::priority.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "numeric".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::name.to_owned(),
                    data_type: "String".to_owned(),
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::description.to_owned(),
                    data_type: "String".to_owned(),
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::help.to_owned(),
                    data_type: "String".to_owned(),
                    db_data_type: "text".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::active.to_owned(),
                    data_type: "bool".to_owned(),
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: true,
                    default: Some("true".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::order_id.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: Some("uuid_generate_v4()".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::customer_name.to_owned(),
                    data_type: "String".to_owned(),
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("For recognization purposes, this is the name shown to the seller".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::total_items.to_owned(),
                    data_type: "i32".to_owned(),
                    db_data_type: "integer".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::grand_total_amount.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "numeric".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::charges_amount.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "numeric".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: Some("0.00".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::processing.to_owned(),
                    data_type: "bool".to_owned(),
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: Some("false".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::processed.to_owned(),
                    data_type: "bool".to_owned(),
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: Some("false".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::is_confirmed.to_owned(),
                    data_type: "bool".to_owned(),
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: Some("false".to_owned()),
                    comment: Some("determined whether the order has been confirmed by the person who ordered it".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::is_tax_included.to_owned(),
                    data_type: "bool".to_owned(),
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: Some("true".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::date_ordered.to_owned(),
                    data_type: "DateTime<UTC>".to_owned(),
                    db_data_type: "timestamp with time zone".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: Some("now()".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::is_invoiced.to_owned(),
                    data_type: "bool".to_owned(),
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: Some("false".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::date_invoiced.to_owned(),
                    data_type: "DateTime<UTC>".to_owned(),
                    db_data_type: "timestamp with time zone".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::is_approved.to_owned(),
                    data_type: "bool".to_owned(),
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: Some("false".to_owned()),
                    comment: Some("if the order from the buyer is approved by the seller".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::date_approved.to_owned(),
                    data_type: "DateTime<UTC>".to_owned(),
                    db_data_type: "timestamp with time zone".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::amount_tendered.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "numeric".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::amount_refunded.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "numeric".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::cart_id.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("The cart from which this order was created from".to_owned()),
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
pub static organization_id: &'static str = "orders.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "orders.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "orders.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "orders.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "orders.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "orders.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "orders.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "orders.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "orders.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "orders.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "orders.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static order_id: &'static str = "orders.order_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static customer_name: &'static str = "orders.customer_name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static total_items: &'static str = "orders.total_items";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static grand_total_amount: &'static str = "orders.grand_total_amount";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static charges_amount: &'static str = "orders.charges_amount";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static processing: &'static str = "orders.processing";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static processed: &'static str = "orders.processed";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static is_confirmed: &'static str = "orders.is_confirmed";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static is_tax_included: &'static str = "orders.is_tax_included";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static date_ordered: &'static str = "orders.date_ordered";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static is_invoiced: &'static str = "orders.is_invoiced";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static date_invoiced: &'static str = "orders.date_invoiced";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static is_approved: &'static str = "orders.is_approved";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static date_approved: &'static str = "orders.date_approved";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static amount_tendered: &'static str = "orders.amount_tendered";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static amount_refunded: &'static str = "orders.amount_refunded";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static cart_id: &'static str = "orders.cart_id";
