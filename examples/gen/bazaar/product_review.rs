//! WARNING: This file is generated, derived from table bazaar.product_review, DO NOT EDIT

use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Table;
use rustorm::table::Foreign;



#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct ProductReview {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub product_id: Uuid,
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub review_id: Uuid,
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

}



impl IsDao for ProductReview{
    fn from_dao(dao:&Dao)->Self{
        ProductReview{
            organization_id: dao.get_opt("organization_id"),
            client_id: dao.get_opt("client_id"),
            created: dao.get("created"),
            created_by: dao.get_opt("created_by"),
            updated: dao.get("updated"),
            updated_by: dao.get_opt("updated_by"),
            priority: dao.get_opt("priority"),
            product_id: dao.get("product_id"),
            review_id: dao.get("review_id"),
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
        dao.set("product_id", &self.product_id);
        dao.set("review_id", &self.review_id);
        dao
    }
}

impl IsTable for ProductReview{

    fn table()->Table{
    
        Table{
            schema:"bazaar".to_string(),
            name:"product_review".to_string(),
            parent_table:Some("base".to_string()),
            sub_table:vec![],
            comment:None,
            columns:
            vec![
                Column{
                    name:"organization_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"client_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"created".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:true, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"created_by".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updated".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:true, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updated_by".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"priority".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"product_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:true, is_unique:false, not_null:true, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:Some(
                        Foreign{
                            schema:"bazaar".to_string(),
                            table:"product".to_string(),
                            column:"product_id".to_string(),
                        }),
                },
                Column{
                    name:"review_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:true, is_unique:false, not_null:true, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:Some(
                        Foreign{
                            schema:"bazaar".to_string(),
                            table:"review".to_string(),
                            column:"review_id".to_string(),
                        }),
                },
            ],
        }
    }
}
// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static organization_id: &'static str = "product_review.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "product_review.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "product_review.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "product_review.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "product_review.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "product_review.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "product_review.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static product_id: &'static str = "product_review.product_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static review_id: &'static str = "product_review.review_id";
