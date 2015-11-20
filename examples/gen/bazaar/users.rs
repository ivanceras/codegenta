//! WARNING: This file is generated, derived from table bazaar.users, DO NOT EDIT

use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use gen::column;
use gen::schema;
use gen::table;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use rustorm::table::Column;
use rustorm::table::IsTable;
use rustorm::table::Table;
use uuid::Uuid;
use gen::bazaar::ApiKey;
use gen::bazaar::Product;
use gen::bazaar::Review;
use gen::bazaar::Settings;
use gen::bazaar::UserInfo;
use gen::bazaar::UserLocation;



///
/// This are @Users, will be used for @Login

///

#[derive(Debug, Clone)]
pub struct Users {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub user_id: Uuid,
    /// @Email
    /// db data type: character varying
    pub email: Option<String>,
    /// The users' @Password will be check against the value, while you can also specify hashing alogrithm used of the value @Hash(SHA256), or just @SHA256.
    /// 
    /// SHA512, CLEAR_TEXT, MD5 can also be used.
    /// @Length(8-50)
    /// @DisplayLength(20)
    /// db data type: character varying
    pub password: Option<String>,
    /// @Username
    /// @DisplayLength(20)
    /// @Length(2-100)
    /// db data type: character varying
    pub username: Option<String>,
    /// @Active
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
    /// db data type: double precision
    pub priority: Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    /// db data type: timestamp with time zone
    pub updated: DateTime<UTC>,
    /// --inherited-- 
    /// db data type: uuid
    pub updated_by: Option<Uuid>,

    /// has one, extension table
    pub info: Option<Box<UserInfo>>,
    /// has one, extension table
    pub location: Option<Box<UserLocation>>,
    /// has many
    pub api_key: Vec<ApiKey>,
    /// has many
    pub product: Vec<Product>,
    /// has many
    pub settings: Vec<Settings>,
    /// has many, indirect
    pub review: Vec<Review>,
}



impl IsDao for Users {
    fn from_dao(dao: &Dao) -> Self {
        Users {
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
            username: dao.get_opt(column::username),
            password: dao.get_opt(column::password),
            email: dao.get_opt(column::email),
            info: None,
            location: None,
            api_key: vec![],
            product: vec![],
            settings: vec![],
            review: vec![],
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
        match self.username {
            Some(ref _value) => dao.set(column::username, _value),
            None => dao.set_null(column::username)
        }
        match self.password {
            Some(ref _value) => dao.set(column::password, _value),
            None => dao.set_null(column::password)
        }
        match self.email {
            Some(ref _value) => dao.set(column::email, _value),
            None => dao.set_null(column::email)
        }
        dao
    }
}

impl ToJson for Users {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for Users {

    fn default() -> Self {
        Users{
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
            username: Default::default(),
            password: Default::default(),
            email: Default::default(),
            info: Default::default(),
            location: Default::default(),
            api_key: Default::default(),
            product: Default::default(),
            settings: Default::default(),
            review: Default::default(),
        }
    }
}

impl IsTable for Users {

    fn table() -> Table {
        Table {
            schema: schema::bazaar.to_owned(),
            name: table::users.to_owned(),
            parent_table: Some(table::record.to_owned()),
            sub_table: vec![],
            comment: Some("This are @Users, will be used for @Login".to_owned()),
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
                    db_data_type: "double precision".to_owned(),
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
                    comment: Some("@Active".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::user_id.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: Some("uuid_generate_v4()".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::username.to_owned(),
                    data_type: "String".to_owned(),
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("@Username\n@DisplayLength(20)\n@Length(2-100)".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::password.to_owned(),
                    data_type: "String".to_owned(),
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("The users' @Password will be check against the value, while you can also specify hashing alogrithm used of the value @Hash(SHA256), or just @SHA256.\n\nSHA512, CLEAR_TEXT, MD5 can also be used.\n@Length(8-50)\n@DisplayLength(20)".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::email.to_owned(),
                    data_type: "String".to_owned(),
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("@Email".to_owned()),
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
pub static organization_id: &'static str = "users.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "users.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "users.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "users.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "users.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "users.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "users.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "users.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "users.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "users.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "users.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static user_id: &'static str = "users.user_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static username: &'static str = "users.username";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static password: &'static str = "users.password";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static email: &'static str = "users.email";
