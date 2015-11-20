//! WARNING: This file is generated, derived from table bazaar.review, DO NOT EDIT

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
use gen::bazaar::Product;
use gen::bazaar::Users;



///
/// Reviews of buyers from the sellers and the sellers' products

///

#[derive(Debug, Clone)]
pub struct Review {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub review_id: Uuid,
    /// db data type: boolean
    pub approved: Option<bool>,
    /// the user id who approves the review
    /// db data type: uuid
    pub approvedby: Option<Uuid>,
    /// The statement of the review
    /// db data type: character varying
    pub comment: Option<String>,
    /// rating 1 to 5, 5 is the highest
    /// db data type: integer
    pub rating: Option<i32>,
    /// db data type: uuid
    pub user_id: Option<Uuid>,
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

    /// has many, indirect
    pub product: Vec<Product>,
    /// has many, indirect
    pub users: Vec<Users>,
}



impl IsDao for Review {
    fn from_dao(dao: &Dao) -> Self {
        Review {
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
            rating: dao.get_opt(column::rating),
            comment: dao.get_opt(column::comment),
            review_id: dao.get(column::review_id),
            user_id: dao.get_opt(column::user_id),
            approved: dao.get_opt(column::approved),
            approvedby: dao.get_opt(column::approvedby),
            product: vec![],
            users: vec![],
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
        match self.rating {
            Some(ref _value) => dao.set(column::rating, _value),
            None => dao.set_null(column::rating)
        }
        match self.comment {
            Some(ref _value) => dao.set(column::comment, _value),
            None => dao.set_null(column::comment)
        }
        dao.set(column::review_id, &self.review_id);
        match self.user_id {
            Some(ref _value) => dao.set(column::user_id, _value),
            None => dao.set_null(column::user_id)
        }
        match self.approved {
            Some(ref _value) => dao.set(column::approved, _value),
            None => dao.set_null(column::approved)
        }
        match self.approvedby {
            Some(ref _value) => dao.set(column::approvedby, _value),
            None => dao.set_null(column::approvedby)
        }
        dao
    }
}

impl ToJson for Review {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for Review {

    fn default() -> Self {
        Review{
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
            rating: Default::default(),
            comment: Default::default(),
            review_id: Default::default(),
            user_id: Default::default(),
            approved: Default::default(),
            approvedby: Default::default(),
            product: Default::default(),
            users: Default::default(),
        }
    }
}

impl IsTable for Review {

    fn table() -> Table {
        Table {
            schema: schema::bazaar.to_owned(),
            name: table::review.to_owned(),
            parent_table: Some(table::record.to_owned()),
            sub_table: vec![],
            comment: Some("Reviews of buyers from the sellers and the sellers' products".to_owned()),
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
                    name: column::rating.to_owned(),
                    data_type: "i32".to_owned(),
                    db_data_type: "integer".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("rating 1 to 5, 5 is the highest".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::comment.to_owned(),
                    data_type: "String".to_owned(),
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("The statement of the review".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::review_id.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::user_id.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::approved.to_owned(),
                    data_type: "bool".to_owned(),
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::approvedby.to_owned(),
                    data_type: "Uuid".to_owned(),
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("the user id who approves the review".to_owned()),
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
pub static organization_id: &'static str = "review.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "review.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "review.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "review.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "review.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "review.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "review.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "review.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "review.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "review.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "review.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static rating: &'static str = "review.rating";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static comment: &'static str = "review.comment";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static review_id: &'static str = "review.review_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static user_id: &'static str = "review.user_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static approved: &'static str = "review.approved";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static approvedby: &'static str = "review.approvedby";
