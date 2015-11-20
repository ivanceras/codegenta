//! WARNING: This file is generated, derived from table bazaar.photo, DO NOT EDIT

use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use gen::bazaar::PhotoSizes;
use gen::bazaar::Product;
use gen::bazaar::UserInfo;
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
pub struct Photo {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub photo_id: Uuid,
    /// The base64 encoding of the image, which can be stored in the database
    /// db data type: character varying
    pub data: Option<String>,
    /// db data type: integer
    pub seq_no: Option<i32>,
    /// The online version of the photo, could be hosted in cdn somewhere else, to avoid payloads in the system. The online photo can be cached by creating a base64 encoding, then storing it in the local db
    /// db data type: character varying
    pub url: Option<String>,
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
    pub photo_sizes: Vec<PhotoSizes>,
    /// has many
    pub user_info: Vec<UserInfo>,
    /// has many, indirect
    pub product: Vec<Product>,
}



impl IsDao for Photo {
    fn from_dao(dao: &Dao) -> Self {
        Photo {
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
            photo_id: dao.get(column::photo_id),
            url: dao.get_opt(column::url),
            data: dao.get_opt(column::data),
            seq_no: dao.get_opt(column::seq_no),
            photo_sizes: vec![],
            user_info: vec![],
            product: vec![],
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
        dao.set(column::photo_id, &self.photo_id);
        match self.url {
            Some(ref _value) => dao.set(column::url, _value),
            None => dao.set_null(column::url)
        }
        match self.data {
            Some(ref _value) => dao.set(column::data, _value),
            None => dao.set_null(column::data)
        }
        match self.seq_no {
            Some(ref _value) => dao.set(column::seq_no, _value),
            None => dao.set_null(column::seq_no)
        }
        dao
    }
}

impl ToJson for Photo {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for Photo {

    fn default() -> Self {
        Photo{
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
            photo_id: Default::default(),
            url: Default::default(),
            data: Default::default(),
            seq_no: Default::default(),
            photo_sizes: Default::default(),
            user_info: Default::default(),
            product: Default::default(),
        }
    }
}

impl IsTable for Photo {

    fn table() -> Table {
        Table {
            schema: schema::bazaar.to_owned(),
            name: table::photo.to_owned(),
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
                    name: column::photo_id.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: Some("uuid_generate_v4()".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::url.to_owned(),
                    data_type: "String".to_owned(),
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("The online version of the photo, could be hosted in cdn somewhere else, to avoid payloads in the system. The online photo can be cached by creating a base64 encoding, then storing it in the local db".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::data.to_owned(),
                    data_type: "String".to_owned(),
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("The base64 encoding of the image, which can be stored in the database".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::seq_no.to_owned(),
                    data_type: "i32".to_owned(),
                    db_data_type: "integer".to_owned(),
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
pub static organization_id: &'static str = "photo.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "photo.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "photo.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "photo.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "photo.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "photo.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "photo.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "photo.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "photo.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "photo.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "photo.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static photo_id: &'static str = "photo.photo_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static url: &'static str = "photo.url";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static data: &'static str = "photo.data";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static seq_no: &'static str = "photo.seq_no";
