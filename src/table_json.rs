//! Converts json to Table
//! and Table to json format

use rustorm::table::{Table,Column,Foreign};
use rustorm::dao::{Value,Type};
use rustc_serialize::json::{self,Json};
use std::error::Error;
use std::fmt;
use rustorm::platform::postgres::Postgres;
use rustorm::database::DatabaseDev;
use rustorm::query::Operand;
use std::collections::BTreeMap;
use rustc_serialize::json::ToJson;

trait TableJson{
	
	fn from_str(s: &str)->Result<Self, ParseError>;

	fn extract_column(json_column: &Json)->Result<Column, ParseError>;

	fn to_json(&self)->String;

	fn column_to_btree(column: &Column)->Json;
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
							&Json::String(ref json_table) => json_table,
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
				let mut table = Table::with_name(table_name);
				table.parent_table = inherits;
				table.columns = columns;
				table.comment = table_comment;
				Ok(table)
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
						&Json::Object(ref foreign) => {
							let foreign_table = match foreign.get("table"){
								Some(foreign_table) => match foreign_table{ 
									&Json::String(ref foreign_table) => foreign_table,
									_ => return Err(ParseError::new("Expecting String value on foreign.table"))
								},
								None => return Err(ParseError::new("Expecting a table in foreign"))
							};
							let foreign_column = match foreign.get("column"){
								Some(foreign_column) => match foreign_column{
									&Json::String(ref foreign_column) => foreign_column,
									_ => return Err(ParseError::new("Expecting a String value on column"))
								},
								None => return Err(ParseError::new("Expecting a column in foreign"))
							};
							Some(Foreign::from_str(foreign_table, foreign_column))
						},
						_ => return Err(ParseError::new("Expecting Object value on foreign"))
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
		let mut map: BTreeMap<String, Json> = BTreeMap::new();
		map.insert("table".to_owned(),Json::String(self.complete_name()));
		match &self.parent_table{
			&Some(ref parent_table) => {
				map.insert("inherits".to_owned(), Json::String(parent_table.to_owned()));
			},
			&None => {}
		};
		match &self.comment{
			&Some(ref comment) => {
				map.insert("comment".to_owned(), Json::String(comment.to_owned()));
			},
			&None => {}
		};
		if !self.columns.is_empty(){
			let mut json_columns = vec![];
			for column in &self.columns{
				let json_column = Self::column_to_btree(&column);
				json_columns.push(json_column);
			}
			map.insert("columns".to_owned(), Json::Array(json_columns));
		}
		let json_object = Json::Object(map);
		format!("{}",json_object.pretty())
	}

	fn column_to_btree(column: &Column)->Json{
		let mut btree: BTreeMap<String, Json> = BTreeMap::new();
		btree.insert("column".to_owned(),Json::String(column.name.to_owned()));
		// use the db_data_type
		btree.insert("data_type".to_owned(),Json::String(column.db_data_type.to_owned()));
		btree.insert("primary".to_owned(), Json::Boolean(column.is_primary));
		btree.insert("unique".to_owned(), Json::Boolean(column.is_unique));
		btree.insert("nullable".to_owned(), Json::Boolean(column.nullable()));
		match &column.comment{
			&Some(ref comment) => { 
				btree.insert("comment".to_owned(), Json::String(comment.to_owned()));
			},
			&None => {}
		};
		match &column.default{
			&Some(ref operand) => match operand{
				&Operand::Value(ref value) => {
					btree.insert("default".to_owned(), value.to_json());
				},
				_ => panic!("only expecting Operand::Value for now {:?}", column.default)
			},
			&None => {}
		};
		match &column.foreign{
			&Some(ref foreign) => {
				let foreign_table = foreign.complete_table_name();
				let mut foreign_map: BTreeMap<String, Json> = BTreeMap::new();		
				foreign_map.insert("table".to_owned(), Json::String(foreign_table));
				foreign_map.insert("column".to_owned(), Json::String(foreign.column.to_owned()));
				btree.insert("foreign".to_owned(), Json::Object(foreign_map));
			},
			&None => {}
		}
		Json::Object(btree)
	}

}


#[test]
fn test1(){
	let json = r#"
{
  "table": "bazaar.product",
  "inherits": "system.record",
  "comment": "Store all products and services",
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
      "default": 0
    },
    {
      "column": "owner_id",
      "data_type": "uuid",
      "foreign": {
        "table": "bazaar.user",
        "column": "user_id"
      },
      "comment": "The owner of this product"
    },
    {
      "column": "currency_id",
      "data_type": "uuid",
      "foreign": {
        "table": "payment.currency",
        "column": "currency_id"
      }
    }
  ]
}	
	"#;

	let table = Table::from_str(json).unwrap();
	println!("parsed table {:#?}", table);
	let table_json = table.to_json();
	println!("table_json: {}", table_json);
	assert_eq!(table.schema, Some("bazaar".to_owned()));
	assert_eq!(table.name, "product".to_owned());
}
