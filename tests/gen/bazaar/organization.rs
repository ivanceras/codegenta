//! WARNING: This file is generated, derived from table bazaar.organization, DO NOT EDIT

use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Table;
use rustorm::table::Foreign;
use rustc_serialize::json::ToJson;
use rustc_serialize::json::Json;



#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Organization {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// --inherited-- 
    /// db data type: uuid
    pub organization_id: Uuid,
    /// db data type: uuid
    pub address_id: Option<Uuid>,
    /// db data type: character varying
    pub landmark: Option<String>,
    /// db data type: uuid
    pub parent_organization_id: Option<Uuid>,
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

    /// has one, self referential
    pub parent: Option<Box<Organization>>,
    /// has many
    pub organization: Vec<Organization>,
}



impl IsDao for Organization{
    fn from_dao(dao:&Dao)->Self{
        Organization{
            organization_id: dao.get("organization_id"),
            client_id: dao.get_opt("client_id"),
            created: dao.get("created"),
            created_by: dao.get_opt("created_by"),
            updated: dao.get("updated"),
            updated_by: dao.get_opt("updated_by"),
            priority: dao.get_opt("priority"),
            name: dao.get_opt("name"),
            description: dao.get_opt("description"),
            help: dao.get_opt("help"),
            active: dao.get("active"),
            parent_organization_id: dao.get_opt("parent_organization_id"),
            address_id: dao.get_opt("address_id"),
            landmark: dao.get_opt("landmark"),
            parent: None,
            organization: vec![],
        }
    }

    fn to_dao(&self)->Dao{
        let mut dao = Dao::new();
        dao.set("organization_id", &self.organization_id);
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
        match self.name{
            Some(ref _value) => dao.set("name", _value),
            None => dao.set_null("name")
        }
        match self.description{
            Some(ref _value) => dao.set("description", _value),
            None => dao.set_null("description")
        }
        match self.help{
            Some(ref _value) => dao.set("help", _value),
            None => dao.set_null("help")
        }
        dao.set("active", &self.active);
        match self.parent_organization_id{
            Some(ref _value) => dao.set("parent_organization_id", _value),
            None => dao.set_null("parent_organization_id")
        }
        match self.address_id{
            Some(ref _value) => dao.set("address_id", _value),
            None => dao.set_null("address_id")
        }
        match self.landmark{
            Some(ref _value) => dao.set("landmark", _value),
            None => dao.set_null("landmark")
        }
        dao
    }
}

impl ToJson for Organization{

    fn to_json(&self)->Json{
        self.to_dao().to_json()
    }
}

impl IsTable for Organization{

    fn table()->Table{
    
        Table{
            schema:"bazaar".to_string(),
            name:"organization".to_string(),
            parent_table:Some("record".to_string()),
            sub_table:vec![],
            comment:None,
            columns:
            vec![
                Column{
                    name:"organization_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:true, is_unique:false, not_null:true, is_inherited:true, 
                    default:Some("uuid_generate_v4()".to_string()),
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
                    name:"name".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"description".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"help".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"text".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"active".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:true, 
                    default:Some("true".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"parent_organization_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:Some(
                        Foreign{
                            schema:"bazaar".to_string(),
                            table:"organization".to_string(),
                            column:"organization_id".to_string(),
                        }),
                },
                Column{
                    name:"address_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"landmark".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
            ],
            is_view: false
        }
    }
}
// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static organization_id: &'static str = "organization.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "organization.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "organization.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "organization.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "organization.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "organization.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "organization.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "organization.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "organization.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "organization.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "organization.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static parent_organization_id: &'static str = "organization.parent_organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static address_id: &'static str = "organization.address_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static landmark: &'static str = "organization.landmark";
