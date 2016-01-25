//! WARNING: This file is generated, derived from table system.base, DO NOT EDIT

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



///
/// Base table contains the creation and modification status of a record

///
#[derive(RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Base {
    /// db data type: uuid
    pub client_id: Option<Uuid>,
    /// default: 'now()'
    /// not nullable 
    /// db data type: timestamp with time zone
    pub created: DateTime<UTC>,
    /// db data type: uuid
    pub created_by: Option<Uuid>,
    /// db data type: uuid
    pub organization_id: Option<Uuid>,
    /// priority of saving data and eviction
    /// db data type: double precision
    pub priority: Option<f64>,
    /// default: 'now()'
    /// not nullable 
    /// db data type: timestamp with time zone
    pub updated: DateTime<UTC>,
    /// db data type: uuid
    pub updated_by: Option<Uuid>,

}



impl IsDao for Base {
    fn from_dao(dao: &Dao) -> Self {
        Base {
            organization_id: dao.get_opt(column::organization_id),
            client_id: dao.get_opt(column::client_id),
            created: dao.get(column::created),
            created_by: dao.get_opt(column::created_by),
            updated: dao.get(column::updated),
            updated_by: dao.get_opt(column::updated_by),
            priority: dao.get_opt(column::priority),
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
        dao
    }
}

impl ToJson for Base {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for Base {

    fn default() -> Self {
        Base{
            organization_id: Default::default(),
            client_id: Default::default(),
            created: UTC::now(),
            created_by: Default::default(),
            updated: UTC::now(),
            updated_by: Default::default(),
            priority: Default::default(),
        }
    }
}

impl IsTable for Base {

    fn table() -> Table {
        Table {
            schema: Some(schema::system.to_owned()),
            name: table::base.to_owned(),
            parent_table: None,
            sub_table: vec![table::record.to_owned(),table::product_availability.to_owned(),table::product_category.to_owned(),table::product_photo.to_owned(),table::product_review.to_owned(),],
            comment: Some(r#"Base table contains the creation and modification status of a record"#.to_owned()),
            columns: vec![
                organization_id(),
                client_id(),
                created(),
                created_by(),
                updated(),
                updated_by(),
                priority(),
            ],
            is_view: false,
        }
    }
}
// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

pub fn organization_id()->Column{
    Column {
        table: Some(table::base.to_owned()),
        name: column::organization_id.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: false,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn client_id()->Column{
    Column {
        table: Some(table::base.to_owned()),
        name: column::client_id.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: false,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn created()->Column{
    Column {
        table: Some(table::base.to_owned()),
        name: column::created.to_owned(),
        data_type: Type::DateTime,
        db_data_type: "timestamp with time zone".to_owned(),
        is_primary: false, is_unique: false, not_null: true, is_inherited: false,
        default: Some(Operand::Value(Value::String("'now()'".to_owned()))),
        comment: None,
        foreign: None,
    }
}

pub fn created_by()->Column{
    Column {
        table: Some(table::base.to_owned()),
        name: column::created_by.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: false,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn updated()->Column{
    Column {
        table: Some(table::base.to_owned()),
        name: column::updated.to_owned(),
        data_type: Type::DateTime,
        db_data_type: "timestamp with time zone".to_owned(),
        is_primary: false, is_unique: false, not_null: true, is_inherited: false,
        default: Some(Operand::Value(Value::String("'now()'".to_owned()))),
        comment: None,
        foreign: None,
    }
}

pub fn updated_by()->Column{
    Column {
        table: Some(table::base.to_owned()),
        name: column::updated_by.to_owned(),
        data_type: Type::Uuid,
        db_data_type: "uuid".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: false,
        default: None,
        comment: None,
        foreign: None,
    }
}

pub fn priority()->Column{
    Column {
        table: Some(table::base.to_owned()),
        name: column::priority.to_owned(),
        data_type: Type::F64,
        db_data_type: "double precision".to_owned(),
        is_primary: false, is_unique: false, not_null: false, is_inherited: false,
        default: None,
        comment: Some(r#"priority of saving data and eviction"#.to_owned()),
        foreign: None,
    }
}
