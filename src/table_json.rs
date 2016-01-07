//! Converts json to Table
//! and Table to json format

use rustorm::table::{Table,Column,Foreign};
use rustorm::dao::{Value,Type};
use rustc_serialize::json::Json;
use std::error::Error;
use std::fmt;
use rustorm::platform::postgres::Postgres;
use rustorm::database::DatabaseDev;
use rustorm::query::Operand;

trait TableJson{
	
	fn from_str(s: &str)->Result<Self, ParseError>;

	fn extract_column(json_column: &Json)->Result<Column, ParseError>;

	fn to_json(&self)->String;
}

#[derive(Debug)]
struct ParseError{
	description: String,
}

impl ParseError{

	fn new(s: &str)->Self{
		ParseError{
			description: s.to_owned()
		}
	}
}

impl Error for ParseError{
	
	fn description(&self) -> &str{
		&self.description
	}
	fn cause(&self)->Option<&Error> {
		None	
	}
}

impl fmt::Display for ParseError{
	
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}",self.description())
	}
}

impl TableJson for Table{

	fn from_str(s: &str)->Result<Self, ParseError>{
		println!("json: {:?}",s);
		let json: Json = Json::from_str(s).unwrap();
		println!("Json: {:#?}", json);
		match json{
			Json::Object(btree) => {
				let table_name = match btree.get("table"){
					Some(json_table) => {
						match json_table{
							&Json::String(ref json_table) => json_table.to_owned(),
							_ => return Err(ParseError::new("Expecting a string on table name")),
						}
					},
					None => return Err(ParseError::new("Expecting a table name")),
				};
				let inherits = match btree.get("inherits"){
					Some(inherits) => match inherits{
						&Json::String(ref inherits) => Some(inherits.to_owned()),
						_ => return Err(ParseError::new("Expecting a string on inherits"))
					},
					None => None
				};
				let table_comment = match btree.get("comment"){
					Some(comment) => match comment{
						&Json::String(ref comment) => Some(comment.to_owned()),
						_ => return Err(ParseError::new("Expecting a String on comment"))
					},
					None => None
				};
				let mut columns = vec![];
				match btree.get("columns"){
					Some(json_columns) => match json_columns{
							&Json::Array(ref json_columns) => {
								for json_column in json_columns{
									let column =  try!(Self::extract_column(&json_column));
									columns.push(column);
								}
							},
							_ => return Err(ParseError::new("Expecting an array for columns")),
					},
					None => println!("no columns!"),
				};

				Ok(Table{
					name: table_name,
					parent_table: inherits,
					columns: columns,
					comment: table_comment,
					..Default::default()
				})
			},
			_ => return Err(ParseError::new("expecting an object")),
		}
	}

	fn extract_column(json_column: &Json)->Result<Column, ParseError>{
		match json_column{
			&Json::Object(ref json_column) => {
				let column_name = match json_column.get("column"){
					Some(json_column) => match json_column{
						&Json::String(ref json_column) => json_column.to_owned(),
						_ => return Err(ParseError::new("Expecting a string on column name"))
					},
					None => return Err(ParseError::new("Expecting a column"))
				};
				let data_type = match json_column.get("data_type"){
					Some(data_type) => match data_type{
						&Json::String(ref data_type) => data_type,
						_ => return Err(ParseError::new("Expecting a string on data_type value"))
					},
					None => return Err(ParseError::new("Expecting data_type"))
				};
				let primary = match json_column.get("primary"){
					Some(primary) => match primary{
						&Json::Boolean(primary) => primary,
						_ => return Err(ParseError::new("Expecting Boolean value on primary"))
					},
					None => false
				};
				let unique = match json_column.get("unique"){
					Some(unique) => match unique{
						&Json::Boolean(unique) => unique,
						_ => return Err(ParseError::new("Expecting Boolean value on unique"))
					},
					None => false
				};
				let nullable = match json_column.get("nullable"){
					Some(nullable) => match nullable{
						&Json::Boolean(nullable) => nullable,
						_ => return Err(ParseError::new("Expecting Boolean value on nullable"))
					},
					None => true
				};
				let comment = match json_column.get("comment"){
					Some(comment) => match comment{
						&Json::String(ref comment) => Some(comment.to_owned()),
						_ => return Err(ParseError::new("Expecting String value on comment"))
					},
					None => None
				};
				let default = match json_column.get("default"){
					Some(default) => match default{
						&Json::I64(default) => Some(Operand::Value(Value::I64(default))),
						&Json::U64(default) => Some(Operand::Value(Value::U64(default))),
						&Json::F64(default) => Some(Operand::Value(Value::F64(default))),
						&Json::String(ref default) => Some(Operand::Value(Value::String(default.to_owned()))),//TODO detect functions here
						&Json::Boolean(default) => Some(Operand::Value(Value::Bool(default))),
						&Json::Null => Some(Operand::Value(Value::None(Type::Json))),
						_ => return Err(ParseError::new("Expecting i64, u64, f64, String, Boolean or null for default"))
					},
					None => None
				};
				let foreign = match json_column.get("foreign"){
					Some(foreign) => match foreign{
						&Json::String(ref foreign) => {
							let foreign = Foreign::from_str(foreign);
							Some(foreign)
						},
						_ => return Err(ParseError::new("Expecting String value on foreign"))
					},
					None => None
				};
				let pg = Postgres::new();
				let (_, rtype) = pg.dbtype_to_rust_type(&data_type);
				Ok(Column{
					name: column_name,
					data_type: rtype,
					db_data_type: format!("{}",data_type),
					is_primary: primary,
					is_unique: unique,
					default: default,
					comment: comment,
					not_null: !nullable,
					foreign: foreign,
					is_inherited: false,
				})
			},
			_ => return Err(ParseError::new("expecting an object")),
		}
	}

	fn to_json(&self)->String{
		panic!("soon");	
	}
}


#[test]
fn test1(){
	let json = r#"
{
  "table": "bazaar.product",
  "inherits": "system.record",
  "columns": [
    {
      "column": "product_id",
      "data_type": "uuid",
      "primary": true,
      "nullable": false,
      "default": "uuid_generate_v4()"
    },
    {
      "column": "parent_product_id",
      "data_type": "uuid"
    },
    {
      "column": "is_service",
      "data_type": "boolean",
      "default": false,
      "comment": "Determines if this product is a service"
    },
    {
      "column": "price",
      "data_type": "double precision"
    },
    {
      "column": "user_parent_price",
      "data_type": "boolean",
      "default": false
    },
    {
      "column": "unit",
      "data_type": "character varying"
    },
    {
      "column": "tags",
      "data_type": "json"
    },
    {
      "column": "info",
      "data_type": "json"
    },
    {
      "column": "upfront_fee",
      "data_type": "double precision",
      "default": 0.0
    },
    {
      "column": "owner_id",
      "data_type": "uuid",
      "foreign": "bazaar.user",
      "comment": "The owner of this product"
    },
    {
      "column": "currency_id",
      "data_type": "uuid",
      "foreign": "payment.currency"
    }
  ]
}
	"#;
	let table = Table::from_str(json);
	println!("parsed table {:#?}", table);
	assert_eq!(json, table.unwrap().to_json());
}
