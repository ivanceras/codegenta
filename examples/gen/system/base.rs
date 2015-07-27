//! WARNING: This file is generated, derived from table system.base, DO NOT EDIT

use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Table;
use rustc_serialize::json::ToJson;
use rustc_serialize::json::Json;



///
/// Base table contains the creation and modification status of a record
///
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Base {
    /// db data type: uuid
    pub client_id: Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// db data type: timestamp with time zone
    pub created: DateTime<UTC>,
    /// db data type: uuid
    pub created_by: Option<Uuid>,
    /// db data type: uuid
    pub organization_id: Option<Uuid>,
    /// priority of saving data and eviction
    /// db data type: numeric
    pub priority: Option<f64>,
    /// default: now()
    /// not nullable 
    /// db data type: timestamp with time zone
    pub updated: DateTime<UTC>,
    /// db data type: uuid
    pub updated_by: Option<Uuid>,

}



impl IsDao for Base{
    fn from_dao(dao:&Dao)->Self{
        Base{
            organization_id: dao.get_opt("organization_id"),
            client_id: dao.get_opt("client_id"),
            created: dao.get("created"),
            created_by: dao.get_opt("created_by"),
            updated: dao.get("updated"),
            updated_by: dao.get_opt("updated_by"),
            priority: dao.get_opt("priority"),
        }
    }

    fn to_dao(&self)->Dao{
        let mut dao = Dao::new();
        match self.organization_id{
            Some(ref _value) => dao.set("organization_id", _value),
            None => dao.set_null("organization_id")
        }
        match self.client_id{
            Some(ref _value) => dao.set("client_id", _value),
            None => dao.set_null("client_id")
        }
        dao.set("created", &self.created);
        match self.created_by{
            Some(ref _value) => dao.set("created_by", _value),
            None => dao.set_null("created_by")
        }
        dao.set("updated", &self.updated);
        match self.updated_by{
            Some(ref _value) => dao.set("updated_by", _value),
            None => dao.set_null("updated_by")
        }
        match self.priority{
            Some(ref _value) => dao.set("priority", _value),
            None => dao.set_null("priority")
        }
        dao
    }
}

impl ToJson for Base{

    fn to_json(&self)->Json{
        self.to_dao().to_json()
    }
}

impl IsTable for Base{

    fn table()->Table{
    
        Table{
            schema:"system".to_string(),
            name:"base".to_string(),
            parent_table:None,
            sub_table:vec!["record".to_string(),"product_availability".to_string(),"product_category".to_string(),"product_photo".to_string(),"product_review".to_string(),],
            comment:Some("Base table contains the creation and modification status of a record".to_string()),
            columns:
            vec![
                Column{
                    name:"organization_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"client_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"created".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:false, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"created_by".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updated".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:false, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updated_by".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"priority".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:Some("priority of saving data and eviction".to_string()),
                    foreign:None,
                },
            ],
        }
    }
}
// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static organization_id: &'static str = "base.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "base.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "base.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "base.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "base.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "base.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "base.priority";
