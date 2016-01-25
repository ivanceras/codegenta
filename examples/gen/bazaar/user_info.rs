//! WARNING: This file is generated, derived from table bazaar.user_info, DO NOT EDIT

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
use gen::bazaar::Address;
use gen::bazaar::Photo;



#[derive(RustcEncodable)]
#[derive(Debug, Clone)]
pub struct UserInfo {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub user_id: Uuid,
    /// db data type: uuid
    pub address_id: Option<Uuid>,
    /// db data type: character varying
    pub current_location: Option<String>,
    /// db data type: character varying
    pub displayname: Option<String>,
    /// db data type: uuid
    pub photo_id: Option<Uuid>,
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
    pub address: Option<Address>,
    /// has one
    pub photo: Option<Photo>,
}



impl IsDao for UserInfo {
    fn from_dao(dao: &Dao) -> Self {
        UserInfo {
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
            user_id: dao.get(column::user_id),
            address_id: dao.get_opt(column::address_id),
            current_location: dao.get_opt(column::current_location),
            displayname: dao.get_opt(column::displayname),
            photo_id: dao.get_opt(column::photo_id),
            address: None,
            photo: None,
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
        dao.set(column::user_id, &self.user_id);
        match self.address_id {
            Some(ref _value) => dao.set(column::address_id, _value),
            None => dao.set_null(column::address_id)
        }
        match self.current_location {
            Some(ref _value) => dao.set(column::current_location, _value),
            None => dao.set_null(column::current_location)
        }
        match self.displayname {
            Some(ref _value) => dao.set(column::displayname, _value),
            None => dao.set_null(column::displayname)
        }
        match self.photo_id {
            Some(ref _value) => dao.set(column::photo_id, _value),
            None => dao.set_null(column::photo_id)
        }
        dao
    }
}

impl ToJson for UserInfo {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for UserInfo {

    fn default() -> Self {
        UserInfo{
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
            user_id: Default::default(),
            address_id: Default::default(),
            current_location: Default::default(),
            displayname: Default::default(),
            photo_id: Default::default(),
            address: Default::default(),
            photo: Default::default(),
        }
    }
}

impl IsTable for UserInfo {

    fn table() -> Table {
        Table {
            schema: Some(schema::bazaar.to_owned()),
            name: table::user_info.to_owned(),
            parent_table: Some(table::record.to_owned()),
            sub_table: vec![],
            comment: None,
            columns: vec![
                organization_id(),
                client_id(),
                created(),
                created_by(),
                updated(),
                updated_by(),
                priority(),
                name(),
                description(),
                help(),
                active(),
                user_id(),
                address_id(),
                current_location(),
                displayname(),
                photo_id(),
            ],
            is_view: false,
        }
    }
}
// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

pub fn organization_id()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::organization_id.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn client_id()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::client_id.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn created()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::created.to_owned(),
        data_type: Type::DateTime,
        db_data_type: "timestamp with time zone".to_owned(),
        is_primary: false, is_unique: false, not_null: true, is_inherited: true,
        default: Some(Operand::Value(Value::String("'now()'".to_owned()))),
        comment: None,
        foreign: None,
    }
}

pub fn created_by()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::created_by.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn updated()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::updated.to_owned(),
        data_type: Type::DateTime,
        db_data_type: "timestamp with time zone".to_owned(),
        is_primary: false, is_unique: false, not_null: true, is_inherited: true,
        default: Some(Operand::Value(Value::String("'now()'".to_owned()))),
        comment: None,
        foreign: None,
    }
}

pub fn updated_by()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::updated_by.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn priority()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::priority.to_owned(),
        data_type: Type::F64,
        db_data_type: "double precision".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn name()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::name.to_owned(),
        data_type: Type::String,
        db_data_type: "character varying".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn description()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::description.to_owned(),
        data_type: Type::String,
        db_data_type: "character varying".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn help()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::help.to_owned(),
        data_type: Type::String,
        db_data_type: "text".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn active()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::active.to_owned(),
        data_type: Type::Bool,
        db_data_type: "boolean".to_owned(),
        is_primary: false, is_unique: false, not_null: true, is_inherited: true,
        default: Some(Operand::Value(Value::String("'true'".to_owned()))),
        comment: None,
        foreign: None,
    }
}

pub fn user_id()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::user_id.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: true, is_unique: false, not_null: true, is_inherited: false,
        default: None,
        comment: None,
        foreign: Some(
                        Foreign {
                            schema: Some("bazaar".to_owned()),
                            table: "users".to_owned(),
                            column: "user_id".to_owned(),
                        }),
    }
}

pub fn address_id()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::address_id.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: false,
        default: None,
        comment: None,
        foreign: Some(
                        Foreign {
                            schema: Some("bazaar".to_owned()),
                            table: "address".to_owned(),
                            column: "address_id".to_owned(),
                        }),
    }
}

pub fn current_location()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::current_location.to_owned(),
        data_type: Type::String,
        db_data_type: "character varying".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: false,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn displayname()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::displayname.to_owned(),
        data_type: Type::String,
        db_data_type: "character varying".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: false,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn photo_id()->Column{
    Column {
        table: Some(table::user_info.to_owned()),
        name: column::photo_id.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: false,
        default: None,
        comment: None,
        foreign: Some(
                        Foreign {
                            schema: Some("bazaar".to_owned()),
                            table: "photo".to_owned(),
                            column: "photo_id".to_owned(),
                        }),
    }
}
