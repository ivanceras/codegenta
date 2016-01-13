//! WARNING: This file is generated, derived from table bazaar.product, DO NOT EDIT

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
use gen::bazaar::Category;
use gen::bazaar::Photo;
use gen::bazaar::ProductAvailability;
use gen::bazaar::Review;
use gen::bazaar::Users;
use gen::payment::Currency;


pub fn product() -> Table {
	Table {
		schema: Some(schema::bazaar.to_owned()),
		name: table::product.to_owned(),
		parent_table: Some(table::record.to_owned()),
		sub_table: vec![],
		comment: Some("This will be exposed as an @Api, including @Table(users, category, product_availability, photo)".to_owned()),
		columns: vec![
			Column {
				name: column::organization_id.to_owned(),
				data_type: Type::Uuid,
				db_data_type: "uuid".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: true,
				default: None,
				comment: Some("@Value(users.user_id) , which means the value will be set with the users.user_id value\n\n@Where(users.active=true)".to_owned()),
				foreign: None,
			},
			Column {
				name: column::client_id.to_owned(),
				data_type: Type::Uuid,
				db_data_type: "uuid".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: true,
				default: None,
				comment: Some("@Value(users.client_id) The client_id of the user creating this records".to_owned()),
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
				comment: Some("@Value(users.user_id)".to_owned()),
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
				comment: Some("@Value(users.user_id)".to_owned()),
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
				comment: Some("This is @Required it has @DisplayLength(50) - 50 character in display length a @MinLength(1) and @MaxLength(100) - Do not go over 100 characters or else the system will throw a ValueTooLong exception\ncan also be express with @Length(1-100)".to_owned()),
				foreign: None,
			},
			Column {
				name: column::description.to_owned(),
				data_type: Type::String,
				db_data_type: "character varying".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: true,
				default: None,
				comment: Some("@DisplayLength(100) When building a UI for this field\n@MaxLength(200) Do not go over 200 character on this one".to_owned()),
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
				comment: Some("@Active".to_owned()),
				foreign: None,
			},
			Column {
				name: column::product_id.to_owned(),
				data_type: Type::Uuid,
				db_data_type: "uuid".to_owned(),
				is_primary: true, is_unique: false, not_null: true, is_inherited: false,
				default: Some(Operand::Value(Value::String("'uuid_generate_v4()'".to_owned()))),
				comment: None,
				foreign: None,
			},
			Column {
				name: column::parent_product_id.to_owned(),
				data_type: Type::Uuid,
				db_data_type: "uuid".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: false,
				default: None,
				comment: None,
				foreign: None,
			},
			Column {
				name: column::is_service.to_owned(),
				data_type: Type::Bool,
				db_data_type: "boolean".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: false,
				default: Some(Operand::Value(Value::String("'false'".to_owned()))),
				comment: None,
				foreign: None,
			},
			Column {
				name: column::price.to_owned(),
				data_type: Type::F64,
				db_data_type: "double precision".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: false,
				default: None,
				comment: None,
				foreign: None,
			},
			Column {
				name: column::use_parent_price.to_owned(),
				data_type: Type::Bool,
				db_data_type: "boolean".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: false,
				default: Some(Operand::Value(Value::String("'false'".to_owned()))),
				comment: None,
				foreign: None,
			},
			Column {
				name: column::unit.to_owned(),
				data_type: Type::String,
				db_data_type: "character varying".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: false,
				default: None,
				comment: None,
				foreign: None,
			},
			Column {
				name: column::tags.to_owned(),
				data_type: Type::Json,
				db_data_type: "json".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: false,
				default: None,
				comment: None,
				foreign: None,
			},
			Column {
				name: column::info.to_owned(),
				data_type: Type::Json,
				db_data_type: "json".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: false,
				default: None,
				comment: Some("{color:\"red\",\ndimension:\"10x20x30\",\ndimensionUnit:\"mm\",\nweight:\"4\",\nweightUnit:\"kg\"\n}".to_owned()),
				foreign: None,
			},
			Column {
				name: column::seq_no.to_owned(),
				data_type: Type::I32,
				db_data_type: "integer".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: false,
				default: None,
				comment: Some("@Sequence can be used to do alternate ordering of the values, when alphetical or time can not be used".to_owned()),
				foreign: None,
			},
			Column {
				name: column::upfront_fee.to_owned(),
				data_type: Type::F64,
				db_data_type: "double precision".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: false,
				default: Some(Operand::Value(Value::String("'0.00'".to_owned()))),
				comment: Some("Applicable to services, usually services has an upfront fee".to_owned()),
				foreign: None,
			},
			Column {
				name: column::barcode.to_owned(),
				data_type: Type::String,
				db_data_type: "character varying".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: false,
				default: None,
				comment: Some("barcode if scanning the product, conflict can happen, expect to return matching list of products using the barcode".to_owned()),
				foreign: None,
			},
			Column {
				name: column::owner_id.to_owned(),
				data_type: Type::Uuid,
				db_data_type: "uuid".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: false,
				default: None,
				comment: Some("Whom this product belongs, since created_by can be someone else create the product list in behalf of the owner of the product".to_owned()),
				foreign: Some(
					Foreign {
						schema: Some("bazaar".to_owned()),
						table: "users".to_owned(),
						column: "user_id".to_owned(),
					}),
			},
			Column {
				name: column::currency_id.to_owned(),
				data_type: Type::Uuid,
				db_data_type: "uuid".to_owned(),
				is_primary: false, is_unique: false, not_null: false, is_inherited: false,
				default: None,
				comment: None,
				foreign: Some(
					Foreign {
						schema: Some("payment".to_owned()),
						table: "currency".to_owned(),
						column: "currency_id".to_owned(),
					}),
			},
		],
		is_view: false,
	}
}

///
/// This will be exposed as an @Api, including @Table(users, category, product_availability, photo)

///
#[derive(RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Product {
    /// primary
    /// default: 'uuid_generate_v4()'
    /// not nullable 
    /// db data type: uuid
    pub product_id: Uuid,
    /// barcode if scanning the product, conflict can happen, expect to return matching list of products using the barcode
    /// db data type: character varying
    pub barcode: Option<String>,
    /// db data type: uuid
    pub currency_id: Option<Uuid>,
    /// {color:"red",
    /// dimension:"10x20x30",
    /// dimensionUnit:"mm",
    /// weight:"4",
    /// weightUnit:"kg"
    /// }
    /// db data type: json
    pub info: Option<Json>,
    /// default: 'false'
    /// db data type: boolean
    pub is_service: Option<bool>,
    /// Whom this product belongs, since created_by can be someone else create the product list in behalf of the owner of the product
    /// db data type: uuid
    pub owner_id: Option<Uuid>,
    /// db data type: uuid
    pub parent_product_id: Option<Uuid>,
    /// db data type: double precision
    pub price: Option<f64>,
    /// @Sequence can be used to do alternate ordering of the values, when alphetical or time can not be used
    /// db data type: integer
    pub seq_no: Option<i32>,
    /// db data type: json
    pub tags: Option<Json>,
    /// db data type: character varying
    pub unit: Option<String>,
    /// Applicable to services, usually services has an upfront fee
    /// default: '0.00'
    /// db data type: double precision
    pub upfront_fee: Option<f64>,
    /// default: 'false'
    /// db data type: boolean
    pub use_parent_price: Option<bool>,
    /// @Active
    /// default: 'true'
    /// not nullable 
    /// --inherited-- 
    /// db data type: boolean
    pub active: bool,
    /// @Value(users.client_id) The client_id of the user creating this records
    /// --inherited-- 
    /// db data type: uuid
    pub client_id: Option<Uuid>,
    /// default: 'now()'
    /// not nullable 
    /// --inherited-- 
    /// db data type: timestamp with time zone
    pub created: DateTime<UTC>,
    /// @Value(users.user_id)
    /// --inherited-- 
    /// db data type: uuid
    pub created_by: Option<Uuid>,
    /// @DisplayLength(100) When building a UI for this field
    /// @MaxLength(200) Do not go over 200 character on this one
    /// --inherited-- 
    /// db data type: character varying
    pub description: Option<String>,
    /// --inherited-- 
    /// db data type: text
    pub help: Option<String>,
    /// This is @Required it has @DisplayLength(50) - 50 character in display length a @MinLength(1) and @MaxLength(100) - Do not go over 100 characters or else the system will throw a ValueTooLong exception
    /// can also be express with @Length(1-100)
    /// --inherited-- 
    /// db data type: character varying
    pub name: Option<String>,
    /// @Value(users.user_id) , which means the value will be set with the users.user_id value
    /// 
    /// @Where(users.active=true)
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
    /// @Value(users.user_id)
    /// --inherited-- 
    /// db data type: uuid
    pub updated_by: Option<Uuid>,

    /// has one
    pub owner: Option<Users>,
    /// has one
    pub currency: Option<Currency>,
    /// has one, extension table
    pub availability: Option<Box<ProductAvailability>>,
    /// has many, indirect
    pub category: Vec<Category>,
    /// has many, indirect
    pub photo: Vec<Photo>,
    /// has many, indirect
    pub review: Vec<Review>,
}



impl IsDao for Product {
    fn from_dao(dao: &Dao) -> Self {
        Product {
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
            product_id: dao.get(column::product_id),
            parent_product_id: dao.get_opt(column::parent_product_id),
            is_service: dao.get_opt(column::is_service),
            price: dao.get_opt(column::price),
            use_parent_price: dao.get_opt(column::use_parent_price),
            unit: dao.get_opt(column::unit),
            tags: dao.get_opt(column::tags),
            info: dao.get_opt(column::info),
            seq_no: dao.get_opt(column::seq_no),
            upfront_fee: dao.get_opt(column::upfront_fee),
            barcode: dao.get_opt(column::barcode),
            owner_id: dao.get_opt(column::owner_id),
            currency_id: dao.get_opt(column::currency_id),
            owner: None,
            currency: None,
            availability: None,
            category: vec![],
            photo: vec![],
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
        dao.set(column::product_id, &self.product_id);
        match self.parent_product_id {
            Some(ref _value) => dao.set(column::parent_product_id, _value),
            None => dao.set_null(column::parent_product_id)
        }
        match self.is_service {
            Some(ref _value) => dao.set(column::is_service, _value),
            None => dao.set_null(column::is_service)
        }
        match self.price {
            Some(ref _value) => dao.set(column::price, _value),
            None => dao.set_null(column::price)
        }
        match self.use_parent_price {
            Some(ref _value) => dao.set(column::use_parent_price, _value),
            None => dao.set_null(column::use_parent_price)
        }
        match self.unit {
            Some(ref _value) => dao.set(column::unit, _value),
            None => dao.set_null(column::unit)
        }
        match self.tags {
            Some(ref _value) => dao.set(column::tags, _value),
            None => dao.set_null(column::tags)
        }
        match self.info {
            Some(ref _value) => dao.set(column::info, _value),
            None => dao.set_null(column::info)
        }
        match self.seq_no {
            Some(ref _value) => dao.set(column::seq_no, _value),
            None => dao.set_null(column::seq_no)
        }
        match self.upfront_fee {
            Some(ref _value) => dao.set(column::upfront_fee, _value),
            None => dao.set_null(column::upfront_fee)
        }
        match self.barcode {
            Some(ref _value) => dao.set(column::barcode, _value),
            None => dao.set_null(column::barcode)
        }
        match self.owner_id {
            Some(ref _value) => dao.set(column::owner_id, _value),
            None => dao.set_null(column::owner_id)
        }
        match self.currency_id {
            Some(ref _value) => dao.set(column::currency_id, _value),
            None => dao.set_null(column::currency_id)
        }
        dao
    }
}

impl ToJson for Product {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for Product {

    fn default() -> Self {
        Product{
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
            product_id: Default::default(),
            parent_product_id: Default::default(),
            is_service: Default::default(),
            price: Default::default(),
            use_parent_price: Default::default(),
            unit: Default::default(),
            tags: Default::default(),
            info: Default::default(),
            seq_no: Default::default(),
            upfront_fee: Default::default(),
            barcode: Default::default(),
            owner_id: Default::default(),
            currency_id: Default::default(),
            owner: Default::default(),
            currency: Default::default(),
            availability: Default::default(),
            category: Default::default(),
            photo: Default::default(),
            review: Default::default(),
        }
    }
}

impl IsTable for Product {

    fn table() -> Table {
        Table {
            schema: Some(schema::bazaar.to_owned()),
            name: table::product.to_owned(),
            parent_table: Some(table::record.to_owned()),
            sub_table: vec![],
            comment: Some("This will be exposed as an @Api, including @Table(users, category, product_availability, photo)".to_owned()),
            columns: vec![
                Column {
                    name: column::organization_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: Some("@Value(users.user_id) , which means the value will be set with the users.user_id value\n\n@Where(users.active=true)".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::client_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: Some("@Value(users.client_id) The client_id of the user creating this records".to_owned()),
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
                    comment: Some("@Value(users.user_id)".to_owned()),
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
                    comment: Some("@Value(users.user_id)".to_owned()),
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
                    comment: Some("This is @Required it has @DisplayLength(50) - 50 character in display length a @MinLength(1) and @MaxLength(100) - Do not go over 100 characters or else the system will throw a ValueTooLong exception\ncan also be express with @Length(1-100)".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::description.to_owned(),
                    data_type: Type::String,
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: true,
                    default: None,
                    comment: Some("@DisplayLength(100) When building a UI for this field\n@MaxLength(200) Do not go over 200 character on this one".to_owned()),
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
                    comment: Some("@Active".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::product_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: Some(Operand::Value(Value::String("'uuid_generate_v4()'".to_owned()))),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::parent_product_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::is_service.to_owned(),
                    data_type: Type::Bool,
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: Some(Operand::Value(Value::String("'false'".to_owned()))),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::price.to_owned(),
                    data_type: Type::F64,
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::use_parent_price.to_owned(),
                    data_type: Type::Bool,
                    db_data_type: "boolean".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: Some(Operand::Value(Value::String("'false'".to_owned()))),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::unit.to_owned(),
                    data_type: Type::String,
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::tags.to_owned(),
                    data_type: Type::Json,
                    db_data_type: "json".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::info.to_owned(),
                    data_type: Type::Json,
                    db_data_type: "json".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("{color:\"red\",\ndimension:\"10x20x30\",\ndimensionUnit:\"mm\",\nweight:\"4\",\nweightUnit:\"kg\"\n}".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::seq_no.to_owned(),
                    data_type: Type::I32,
                    db_data_type: "integer".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("@Sequence can be used to do alternate ordering of the values, when alphetical or time can not be used".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::upfront_fee.to_owned(),
                    data_type: Type::F64,
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: Some(Operand::Value(Value::String("'0.00'".to_owned()))),
                    comment: Some("Applicable to services, usually services has an upfront fee".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::barcode.to_owned(),
                    data_type: Type::String,
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("barcode if scanning the product, conflict can happen, expect to return matching list of products using the barcode".to_owned()),
                    foreign: None,
                },
                Column {
                    name: column::owner_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: Some("Whom this product belongs, since created_by can be someone else create the product list in behalf of the owner of the product".to_owned()),
                    foreign: Some(
                        Foreign {
                            schema: Some("bazaar".to_owned()),
                            table: "users".to_owned(),
                            column: "user_id".to_owned(),
                        }),
                },
                Column {
                    name: column::currency_id.to_owned(),
                    data_type: Type::Uuid,
                    db_data_type: "uuid".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: Some(
                        Foreign {
                            schema: Some("payment".to_owned()),
                            table: "currency".to_owned(),
                            column: "currency_id".to_owned(),
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
pub static organization_id: &'static str = "product.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "product.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "product.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "product.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "product.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "product.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "product.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "product.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "product.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "product.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "product.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static product_id: &'static str = "product.product_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static parent_product_id: &'static str = "product.parent_product_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static is_service: &'static str = "product.is_service";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static price: &'static str = "product.price";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static use_parent_price: &'static str = "product.use_parent_price";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static unit: &'static str = "product.unit";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static tags: &'static str = "product.tags";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static info: &'static str = "product.info";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static seq_no: &'static str = "product.seq_no";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static upfront_fee: &'static str = "product.upfront_fee";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static barcode: &'static str = "product.barcode";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static owner_id: &'static str = "product.owner_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static currency_id: &'static str = "product.currency_id";
