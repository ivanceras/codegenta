//! WARNING: This file is generated, derived from table bazaar.address, DO NOT EDIT

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
use rustorm::table::IsTable;
use rustorm::table::Table;
use uuid::Uuid;
use gen::bazaar::UserInfo;



///
/// @Address

///
#[derive(RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Address {
    /// primary
    /// default: 'uuid_generate_v4()'
    /// not nullable 
    /// db data type: uuid
    pub address_id: Uuid,
    /// distance is computed on the fly using the formula in sql, this is here to provide a property on the Models to store the value
    /// db data type: double precision
    pub distance: Option<f64>,
    /// db data type: double precision
    pub latitude: Option<f64>,
    /// db data type: double precision
    pub longitude: Option<f64>,
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

    /// has many
    pub user_info: Vec<UserInfo>,
}



impl IsDao for Address {
    fn from_dao(dao: &Dao) -> Self {
        Address {
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
            address_id: dao.get(column::address_id),
            latitude: dao.get_opt(column::latitude),
            longitude: dao.get_opt(column::longitude),
            distance: dao.get_opt(column::distance),
            user_info: vec![],
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
        dao.set(column::address_id, &self.address_id);
        match self.latitude {
            Some(ref _value) => dao.set(column::latitude, _value),
            None => dao.set_null(column::latitude)
        }
        match self.longitude {
            Some(ref _value) => dao.set(column::longitude, _value),
            None => dao.set_null(column::longitude)
        }
        match self.distance {
            Some(ref _value) => dao.set(column::distance, _value),
            None => dao.set_null(column::distance)
        }
        dao
    }
}

impl ToJson for Address {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for Address {

    fn default() -> Self {
        Address{
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
            address_id: Default::default(),
            latitude: Default::default(),
            longitude: Default::default(),
            distance: Default::default(),
            user_info: Default::default(),
        }
    }
}

impl IsTable for Address {

    fn table() -> Table {
        Table {
            schema: Some(schema::bazaar.to_owned()),
            name: table::address.to_owned(),
            parent_table: Some(table::record.to_owned()),
            sub_table: vec![],
            comment: Some("@Address".to_owned()),
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
                    name: column::address_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: Some(Operand::Value(Value::String("'uuid_generate_v4()'".to_owned()))),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::latitude.to_owned(),
                    data_type: Type::F64,
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::longitude.to_owned(),
                    data_type: Type::F64,
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::distance.to_owned(),
                    data_type: Type::F64,
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("distance is computed on the fly using the formula in sql, this is here to provide a property on the Models to store the value".to_owned()),
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
pub static organization_id: &'static str = "address.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "address.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "address.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "address.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "address.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "address.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "address.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "address.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "address.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "address.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "address.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static address_id: &'static str = "address.address_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static latitude: &'static str = "address.latitude";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static longitude: &'static str = "address.longitude";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static distance: &'static str = "address.distance";
