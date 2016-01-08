//! WARNING: This file is generated, derived from table bazaar.product_availability, DO NOT EDIT

use chrono::datetime::DateTime;
use chrono::naive::time::NaiveTime;
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



#[derive(RustcEncodable)]
#[derive(Debug, Clone)]
pub struct ProductAvailability {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub product_id: Uuid,
    /// db data type: boolean
    pub always_available: Option<bool>,
    /// db data type: boolean
    pub available: Option<bool>,
    /// {"Mon", "Tue", "Wed", "Thur", "Fri", "Sat", "Sun"}
    /// db data type: json
    pub available_day: Option<Json>,
    /// db data type: timestamp with time zone
    pub available_from: Option<DateTime<UTC>>,
    /// db data type: timestamp with time zone
    pub available_until: Option<DateTime<UTC>>,
    /// db data type: time with time zone
    pub close_time: Option<NaiveTime>,
    /// db data type: time with time zone
    pub open_time: Option<NaiveTime>,
    /// default: '1'
    /// db data type: double precision
    pub stocks: Option<f64>,
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

}



impl IsDao for ProductAvailability {
    fn from_dao(dao: &Dao) -> Self {
        ProductAvailability {
            organization_id: dao.get_opt(column::organization_id),
            client_id: dao.get_opt(column::client_id),
            created: dao.get(column::created),
            created_by: dao.get_opt(column::created_by),
            updated: dao.get(column::updated),
            updated_by: dao.get_opt(column::updated_by),
            priority: dao.get_opt(column::priority),
            product_id: dao.get(column::product_id),
            available: dao.get_opt(column::available),
            always_available: dao.get_opt(column::always_available),
            stocks: dao.get_opt(column::stocks),
            available_from: dao.get_opt(column::available_from),
            available_until: dao.get_opt(column::available_until),
            available_day: dao.get_opt(column::available_day),
            open_time: dao.get_opt(column::open_time),
            close_time: dao.get_opt(column::close_time),
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
        dao.set(column::product_id, &self.product_id);
        match self.available {
            Some(ref _value) => dao.set(column::available, _value),
            None => dao.set_null(column::available)
        }
        match self.always_available {
            Some(ref _value) => dao.set(column::always_available, _value),
            None => dao.set_null(column::always_available)
        }
        match self.stocks {
            Some(ref _value) => dao.set(column::stocks, _value),
            None => dao.set_null(column::stocks)
        }
        match self.available_from {
            Some(ref _value) => dao.set(column::available_from, _value),
            None => dao.set_null(column::available_from)
        }
        match self.available_until {
            Some(ref _value) => dao.set(column::available_until, _value),
            None => dao.set_null(column::available_until)
        }
        match self.available_day {
            Some(ref _value) => dao.set(column::available_day, _value),
            None => dao.set_null(column::available_day)
        }
        match self.open_time {
            Some(ref _value) => dao.set(column::open_time, _value),
            None => dao.set_null(column::open_time)
        }
        match self.close_time {
            Some(ref _value) => dao.set(column::close_time, _value),
            None => dao.set_null(column::close_time)
        }
        dao
    }
}

impl ToJson for ProductAvailability {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for ProductAvailability {

    fn default() -> Self {
        ProductAvailability{
            organization_id: Default::default(),
            client_id: Default::default(),
            created: UTC::now(),
            created_by: Default::default(),
            updated: UTC::now(),
            updated_by: Default::default(),
            priority: Default::default(),
            product_id: Default::default(),
            available: Default::default(),
            always_available: Default::default(),
            stocks: Default::default(),
            available_from: Default::default(),
            available_until: Default::default(),
            available_day: Default::default(),
            open_time: Default::default(),
            close_time: Default::default(),
        }
    }
}

impl IsTable for ProductAvailability {

    fn table() -> Table {
        Table {
            schema: Some(schema::bazaar.to_owned()),
            name: table::product_availability.to_owned(),
            parent_table: Some(table::base.to_owned()),
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
                    name: column::product_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: Some(
                        Foreign {
                            schema: Some("bazaar".to_owned()),
                            table: "product".to_owned(),
                            column: "product_id".to_owned(),
                        }),
                },
                Column {
                    name: column::available.to_owned(),
                    data_type: Type::Bool,
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::always_available.to_owned(),
                    data_type: Type::Bool,
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::stocks.to_owned(),
                    data_type: Type::F64,
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: Some(Operand::Value(Value::String("'1'".to_owned()))),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::available_from.to_owned(),
                    data_type: Type::DateTime,
                    db_data_type: "timestamp with time zone".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::available_until.to_owned(),
                    data_type: Type::DateTime,
                    db_data_type: "timestamp with time zone".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::available_day.to_owned(),
                    data_type: Type::Json,
                    db_data_type: "json".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("{\"Mon\", \"Tue\", \"Wed\", \"Thur\", \"Fri\", \"Sat\", \"Sun\"}".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::open_time.to_owned(),
                    data_type: Type::NaiveTime,
                    db_data_type: "time with time zone".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::close_time.to_owned(),
                    data_type: Type::NaiveTime,
                    db_data_type: "time with time zone".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
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
pub static organization_id: &'static str = "product_availability.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "product_availability.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "product_availability.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "product_availability.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "product_availability.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "product_availability.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "product_availability.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static product_id: &'static str = "product_availability.product_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static available: &'static str = "product_availability.available";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static always_available: &'static str = "product_availability.always_available";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static stocks: &'static str = "product_availability.stocks";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static available_from: &'static str = "product_availability.available_from";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static available_until: &'static str = "product_availability.available_until";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static available_day: &'static str = "product_availability.available_day";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static open_time: &'static str = "product_availability.open_time";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static close_time: &'static str = "product_availability.close_time";
