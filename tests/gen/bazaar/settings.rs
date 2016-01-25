//! WARNING: This file is generated, derived from table bazaar.settings, DO NOT EDIT

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
use gen::bazaar::Users;



#[derive(RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Settings {
    /// primary
    /// default: 'uuid_generate_v4()'
    /// not nullable 
    /// db data type: uuid
    pub settings_id: Uuid,
    /// Use metric system as unit, if false, use english system
    /// default: 'true'
    /// db data type: boolean
    pub use_metric: Option<bool>,
    /// db data type: uuid
    pub user_id: Option<Uuid>,
    /// db data type: json
    pub value: Option<Json>,
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
    pub user: Option<Users>,
}



impl IsDao for Settings {
    fn from_dao(dao: &Dao) -> Self {
        Settings {
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
            user_id: dao.get_opt(column::user_id),
            value: dao.get_opt(column::value),
            settings_id: dao.get(column::settings_id),
            use_metric: dao.get_opt(column::use_metric),
            user: None,
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
        match self.user_id {
            Some(ref _value) => dao.set(column::user_id, _value),
            None => dao.set_null(column::user_id)
        }
        match self.value {
            Some(ref _value) => dao.set(column::value, _value),
            None => dao.set_null(column::value)
        }
        dao.set(column::settings_id, &self.settings_id);
        match self.use_metric {
            Some(ref _value) => dao.set(column::use_metric, _value),
            None => dao.set_null(column::use_metric)
        }
        dao
    }
}

impl ToJson for Settings {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for Settings {

    fn default() -> Self {
        Settings{
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
            value: Default::default(),
            settings_id: Default::default(),
            use_metric: Default::default(),
            user: Default::default(),
        }
    }
}

impl IsTable for Settings {

    fn table() -> Table {
        Table {
            schema: Some(schema::bazaar.to_owned()),
            name: table::settings.to_owned(),
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
                value(),
                settings_id(),
                use_metric(),
            ],
            is_view: false,
        }
    }
}
// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(dead_code)]
pub fn organization_id()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::organization_id.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }}

#[allow(dead_code)]
pub fn client_id()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::client_id.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }}

#[allow(dead_code)]
pub fn created()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::created.to_owned(),
        data_type: Type::DateTime,
        db_data_type: "timestamp with time zone".to_owned(),
        is_primary: false, is_unique: false, not_null: true, is_inherited: true,
        default: Some(Operand::Value(Value::String("'now()'".to_owned()))),
        comment: None,
        foreign: None,
    }}

#[allow(dead_code)]
pub fn created_by()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::created_by.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }}

#[allow(dead_code)]
pub fn updated()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::updated.to_owned(),
        data_type: Type::DateTime,
        db_data_type: "timestamp with time zone".to_owned(),
        is_primary: false, is_unique: false, not_null: true, is_inherited: true,
        default: Some(Operand::Value(Value::String("'now()'".to_owned()))),
        comment: None,
        foreign: None,
    }}

#[allow(dead_code)]
pub fn updated_by()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::updated_by.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }}

#[allow(dead_code)]
pub fn priority()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::priority.to_owned(),
        data_type: Type::F64,
        db_data_type: "double precision".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }}

#[allow(dead_code)]
pub fn name()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::name.to_owned(),
        data_type: Type::String,
        db_data_type: "character varying".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }}

#[allow(dead_code)]
pub fn description()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::description.to_owned(),
        data_type: Type::String,
        db_data_type: "character varying".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }}

#[allow(dead_code)]
pub fn help()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::help.to_owned(),
        data_type: Type::String,
        db_data_type: "text".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: true,
        default: None,
        comment: None,
        foreign: None,
    }}

#[allow(dead_code)]
pub fn active()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::active.to_owned(),
        data_type: Type::Bool,
        db_data_type: "boolean".to_owned(),
        is_primary: false, is_unique: false, not_null: true, is_inherited: true,
        default: Some(Operand::Value(Value::String("'true'".to_owned()))),
        comment: None,
        foreign: None,
    }}

#[allow(dead_code)]
pub fn user_id()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::user_id.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: false,
        default: None,
        comment: None,
        foreign: Some(
                        Foreign {
                            schema: Some("bazaar".to_owned()),
                            table: "users".to_owned(),
                            column: "user_id".to_owned(),
                        }),
    }}

#[allow(dead_code)]
pub fn value()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::value.to_owned(),
        data_type: Type::Json,
        db_data_type: "json".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: false,
        default: None,
        comment: None,
        foreign: None,
    }}

#[allow(dead_code)]
pub fn settings_id()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::settings_id.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: true, is_unique: false, not_null: true, is_inherited: false,
        default: Some(Operand::Value(Value::String("'uuid_generate_v4()'".to_owned()))),
        comment: None,
        foreign: None,
    }}

#[allow(dead_code)]
pub fn use_metric()->Column{
    Column {
        table: Some("settings".to_owned()),
        name: column::use_metric.to_owned(),
        data_type: Type::Bool,
        db_data_type: "boolean".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: false,
        default: Some(Operand::Value(Value::String("'true'".to_owned()))),
        comment: Some(r#"Use metric system as unit, if false, use english system"#.to_owned()),
        foreign: None,
    }}
