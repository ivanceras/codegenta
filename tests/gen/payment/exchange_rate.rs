//! WARNING: This file is generated, derived from table payment.exchange_rate, DO NOT EDIT

use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use gen::payment::Currency;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use gen::schema;
use gen::table;
use gen::column;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Table;
use rustorm::table::Foreign;
use rustc_serialize::json::ToJson;
use rustc_serialize::json::Json;



#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct ExchangeRate {
    /// primary
    /// this will be referred when processing payments with different currencies
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub exchange_rate_id: Uuid,
    /// db data type: numeric
    pub exchange_rate: Option<f64>,
    /// db data type: uuid
    pub from_currency: Option<Uuid>,
    /// db data type: uuid
    pub to_currency: Option<Uuid>,
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

    /// has one
    pub from: Option<Currency>,
    /// has one
    pub to: Option<Currency>,
}



impl IsDao for ExchangeRate {
    fn from_dao(dao: &Dao) -> Self {
        ExchangeRate {
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
            exchange_rate_id: dao.get(column::exchange_rate_id),
            from_currency: dao.get_opt(column::from_currency),
            exchange_rate: dao.get_opt(column::exchange_rate),
            to_currency: dao.get_opt(column::to_currency),
            from: None,
            to: None,
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
        dao.set(column::exchange_rate_id, &self.exchange_rate_id);
        match self.from_currency {
            Some(ref _value) => dao.set(column::from_currency, _value),
            None => dao.set_null(column::from_currency)
        }
        match self.exchange_rate {
            Some(ref _value) => dao.set(column::exchange_rate, _value),
            None => dao.set_null(column::exchange_rate)
        }
        match self.to_currency {
            Some(ref _value) => dao.set(column::to_currency, _value),
            None => dao.set_null(column::to_currency)
        }
        dao
    }
}

impl ToJson for ExchangeRate {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for ExchangeRate {

    fn default() -> Self {
        ExchangeRate{
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
            exchange_rate_id: Default::default(),
            from_currency: Default::default(),
            exchange_rate: Default::default(),
            to_currency: Default::default(),
            from: Default::default(),
            to: Default::default(),
        }
    }
}

impl IsTable for ExchangeRate {

    fn table() -> Table {
        Table {
            schema: schema::payment.to_owned(),
            name: table::exchange_rate.to_owned(),
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
                    name: column::exchange_rate_id.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: Some("uuid_generate_v4()".to_owned()),
                    comment: Some("this will be referred when processing payments with different currencies".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::from_currency.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: Some(
                        Foreign {
                            schema: "payment".to_owned(),
                            table: "currency".to_owned(),
                            column: "currency_id".to_owned(),
                        }),
                },
                Column {
                    name: column::exchange_rate.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "numeric".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::to_currency.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: Some(
                        Foreign {
                            schema: "payment".to_owned(),
                            table: "currency".to_owned(),
                            column: "currency_id".to_owned(),
                        }),
                },
            ],
            is_view: false,
        }
    }
}
// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static organization_id: &'static str = "exchange_rate.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "exchange_rate.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "exchange_rate.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "exchange_rate.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "exchange_rate.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "exchange_rate.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "exchange_rate.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "exchange_rate.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "exchange_rate.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "exchange_rate.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "exchange_rate.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static exchange_rate_id: &'static str = "exchange_rate.exchange_rate_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static from_currency: &'static str = "exchange_rate.from_currency";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static exchange_rate: &'static str = "exchange_rate.exchange_rate";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static to_currency: &'static str = "exchange_rate.to_currency";
