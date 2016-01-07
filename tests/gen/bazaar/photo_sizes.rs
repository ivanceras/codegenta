//! WARNING: This file is generated, derived from table bazaar.photo_sizes, DO NOT EDIT

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
use gen::bazaar::Photo;



#[derive(RustcEncodable)]
#[derive(Debug, Clone)]
pub struct PhotoSizes {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub photo_id: Uuid,
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub photo_size_id: Uuid,
    /// The base64 encoding of this photo, optimized to this size
    /// db data type: character varying
    pub data: Option<String>,
    /// db data type: integer
    pub height: Option<i32>,
    /// db data type: character varying
    pub url: Option<String>,
    /// db data type: integer
    pub width: Option<i32>,
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
    pub photo: Option<Photo>,
}



impl IsDao for PhotoSizes {
    fn from_dao(dao: &Dao) -> Self {
        PhotoSizes {
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
            width: dao.get_opt(column::width),
            height: dao.get_opt(column::height),
            data: dao.get_opt(column::data),
            url: dao.get_opt(column::url),
            photo_id: dao.get(column::photo_id),
            photo_size_id: dao.get(column::photo_size_id),
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
        match self.width {
            Some(ref _value) => dao.set(column::width, _value),
            None => dao.set_null(column::width)
        }
        match self.height {
            Some(ref _value) => dao.set(column::height, _value),
            None => dao.set_null(column::height)
        }
        match self.data {
            Some(ref _value) => dao.set(column::data, _value),
            None => dao.set_null(column::data)
        }
        match self.url {
            Some(ref _value) => dao.set(column::url, _value),
            None => dao.set_null(column::url)
        }
        dao.set(column::photo_id, &self.photo_id);
        dao.set(column::photo_size_id, &self.photo_size_id);
        dao
    }
}

impl ToJson for PhotoSizes {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for PhotoSizes {

    fn default() -> Self {
        PhotoSizes{
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
            width: Default::default(),
            height: Default::default(),
            data: Default::default(),
            url: Default::default(),
            photo_id: Default::default(),
            photo_size_id: Default::default(),
            photo: Default::default(),
        }
    }
}

impl IsTable for PhotoSizes {

    fn table() -> Table {
        Table {
            schema: Some(schema::bazaar.to_owned()),
            name: table::photo_sizes.to_owned(),
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
                    name: column::width.to_owned(),
                    data_type: Type::I32,
                    db_data_type: "integer".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::height.to_owned(),
                    data_type: Type::I32,
                    db_data_type: "integer".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::data.to_owned(),
                    data_type: Type::String,
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("The base64 encoding of this photo, optimized to this size".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::url.to_owned(),
                    data_type: Type::String,
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::photo_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: Some(
                        Foreign {
                            schema: Some("bazaar".to_owned()),
                            table: "photo".to_owned(),
                            column: vec!["photo_id".to_owned(),],
                        }),
                },
                Column {
                    name: column::photo_size_id.to_owned(),
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
pub static organization_id: &'static str = "photo_sizes.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "photo_sizes.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "photo_sizes.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "photo_sizes.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "photo_sizes.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "photo_sizes.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "photo_sizes.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "photo_sizes.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "photo_sizes.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "photo_sizes.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "photo_sizes.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static width: &'static str = "photo_sizes.width";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static height: &'static str = "photo_sizes.height";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static data: &'static str = "photo_sizes.data";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static url: &'static str = "photo_sizes.url";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static photo_id: &'static str = "photo_sizes.photo_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static photo_size_id: &'static str = "photo_sizes.photo_size_id";
