//! WARNING: This file is generated, derived from table public.exchangables, DO NOT EDIT

use uuid::Uuid;
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
pub struct Exchangables {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub photo_id: Uuid,
    /// db data type: character varying
    pub data: Option<String>,
    /// db data type: double precision
    pub price: Option<f64>,
    /// db data type: real
    pub priority: Option<f32>,
    /// db data type: integer
    pub seq_no: Option<i32>,
    /// db data type: character varying
    pub url: Option<String>,

}



impl IsDao for Exchangables {
    fn from_dao(dao: &Dao) -> Self {
        Exchangables {
            photo_id: dao.get(column::photo_id),
            url: dao.get_opt(column::url),
            data: dao.get_opt(column::data),
            seq_no: dao.get_opt(column::seq_no),
            price: dao.get_opt(column::price),
            priority: dao.get_opt(column::priority),
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
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
        match self.price {
            Some(ref _value) => dao.set(column::price, _value),
            None => dao.set_null(column::price)
        }
        match self.priority {
            Some(ref _value) => dao.set(column::priority, _value),
            None => dao.set_null(column::priority)
        }
        dao
    }
}

impl ToJson for Exchangables {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for Exchangables {

    fn default() -> Self {
        Exchangables{
            photo_id: Default::default(),
            url: Default::default(),
            data: Default::default(),
            seq_no: Default::default(),
            price: Default::default(),
            priority: Default::default(),
        }
    }
}

impl IsTable for Exchangables {

    fn table() -> Table {
        Table {
            schema: schema::public.to_owned(),
            name: table::exchangables.to_owned(),
            parent_table: None,
            sub_table: vec![],
            comment: None,
            columns: vec![
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
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::data.to_owned(),
                    data_type: "String".to_owned(),
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
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
                Column {
                    name: column::price.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::priority.to_owned(),
                    data_type: "f32".to_owned(),
                    db_data_type: "real".to_owned(),
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
pub static photo_id: &'static str = "exchangables.photo_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static url: &'static str = "exchangables.url";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static data: &'static str = "exchangables.data";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static seq_no: &'static str = "exchangables.seq_no";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static price: &'static str = "exchangables.price";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "exchangables.priority";
