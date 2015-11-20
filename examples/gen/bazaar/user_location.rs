//! WARNING: This file is generated, derived from table bazaar.user_location, DO NOT EDIT

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
use rustorm::table::Foreign;
use rustorm::table::IsTable;
use rustorm::table::Table;
use uuid::Uuid;




#[derive(Debug, Clone)]
pub struct UserLocation {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub user_id: Uuid,
    /// db data type: double precision
    pub accuracy: Option<f64>,
    /// user can anonymize their location by setting loose accuracy
    /// db data type: double precision
    pub set_accuracy: Option<f64>,
    /// db data type: double precision
    pub set_latitude: Option<f64>,
    /// db data type: double precision
    pub set_longitude: Option<f64>,
    /// db data type: double precision
    pub true_latitude: Option<f64>,
    /// db data type: double precision
    pub true_longitude: Option<f64>,
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

}



impl IsDao for UserLocation {
    fn from_dao(dao: &Dao) -> Self {
        UserLocation {
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
            true_latitude: dao.get_opt(column::true_latitude),
            true_longitude: dao.get_opt(column::true_longitude),
            set_latitude: dao.get_opt(column::set_latitude),
            set_longitude: dao.get_opt(column::set_longitude),
            accuracy: dao.get_opt(column::accuracy),
            set_accuracy: dao.get_opt(column::set_accuracy),
            user_id: dao.get(column::user_id),
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
        match self.true_latitude {
            Some(ref _value) => dao.set(column::true_latitude, _value),
            None => dao.set_null(column::true_latitude)
        }
        match self.true_longitude {
            Some(ref _value) => dao.set(column::true_longitude, _value),
            None => dao.set_null(column::true_longitude)
        }
        match self.set_latitude {
            Some(ref _value) => dao.set(column::set_latitude, _value),
            None => dao.set_null(column::set_latitude)
        }
        match self.set_longitude {
            Some(ref _value) => dao.set(column::set_longitude, _value),
            None => dao.set_null(column::set_longitude)
        }
        match self.accuracy {
            Some(ref _value) => dao.set(column::accuracy, _value),
            None => dao.set_null(column::accuracy)
        }
        match self.set_accuracy {
            Some(ref _value) => dao.set(column::set_accuracy, _value),
            None => dao.set_null(column::set_accuracy)
        }
        dao.set(column::user_id, &self.user_id);
        dao
    }
}

impl ToJson for UserLocation {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for UserLocation {

    fn default() -> Self {
        UserLocation{
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
            true_latitude: Default::default(),
            true_longitude: Default::default(),
            set_latitude: Default::default(),
            set_longitude: Default::default(),
            accuracy: Default::default(),
            set_accuracy: Default::default(),
            user_id: Default::default(),
        }
    }
}

impl IsTable for UserLocation {

    fn table() -> Table {
        Table {
            schema: schema::bazaar.to_owned(),
            name: table::user_location.to_owned(),
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
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::true_latitude.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::true_longitude.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::set_latitude.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::set_longitude.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::accuracy.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::set_accuracy.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("user can anonymize their location by setting loose accuracy".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::user_id.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: Some(
                        Foreign {
                            schema: "bazaar".to_owned(),
                            table: "users".to_owned(),
                            column: "user_id".to_owned(),
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
pub static organization_id: &'static str = "user_location.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "user_location.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "user_location.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "user_location.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "user_location.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "user_location.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "user_location.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "user_location.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "user_location.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "user_location.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "user_location.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static true_latitude: &'static str = "user_location.true_latitude";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static true_longitude: &'static str = "user_location.true_longitude";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static set_latitude: &'static str = "user_location.set_latitude";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static set_longitude: &'static str = "user_location.set_longitude";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static accuracy: &'static str = "user_location.accuracy";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static set_accuracy: &'static str = "user_location.set_accuracy";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static user_id: &'static str = "user_location.user_id";
