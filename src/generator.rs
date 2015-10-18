use writer::Writer;
use rustorm::database::DatabaseDev;
use std::fs::File;
use std::fs;
use std::io::Write;

use rustorm::table::Table;
use meta::{MetaCode, StructCode};



/// configuration for generating code

pub struct Config {
    /// the module name to be used when it is possible to unify all tables in 1 module
    /// likely the project name
    pub base_module: Option<String>,
    /// include the has_one, has_many, extension Models
    pub include_table_references: bool,
    ///use the condense name of the has_many else, use the table name referred
    pub use_condensed_name: bool,

    ///generate the is table definition for each table
    pub generate_table_meta: bool,

    /// base directory for the generated content
    pub base_dir: String,

    /// whether to include generating views
    pub include_views: bool,
}

impl Config{

    pub fn default() -> Self {
        Config {
            base_module: Some("gen".to_owned()),
            include_table_references: true,
            use_condensed_name: true,
            generate_table_meta: true,
            base_dir: "./src".to_owned(),
            include_views: true,
        }
    }

    fn table_module(&self, table: &Table) -> String {
        let parent = self.module(&table.schema);
        format!("{}::{}", parent, table.struct_name())
    }

    fn module_dir(&self, schema: &str) -> String {
        let base_module = self.base_module.clone();
        match base_module {
            Some(x) => format!("{}/{}/{}", self.base_dir, x, schema),
            None => format!("{}/{}", self.base_dir, schema),
        }
    }
    fn module_base_dir(&self) -> String {
        let base_module = self.base_module.clone();
        match base_module {
            Some(x) => format!("{}/{}", self.base_dir, x),
            None => format!("./{}", self.base_dir),
        }
    }

    fn module(&self, schema: &str) -> String {
        let base_module = self.base_module.clone();
        match base_module {
            Some(x) => format!("{}::{}", x, schema),
            None => format!("{}", schema),
        }
    }

}

///
/// retrieve all the table definition in the database
///
pub fn get_all_tables(db_dev: &DatabaseDev) -> Vec<Table> {
    let all_tables_names = db_dev.get_all_tables();
    let mut all_table_def: Vec<Table> = Vec::new();
    for (schema, table, is_view) in all_tables_names {
        println!("Extracted {}.{}", schema, table);
        let meta = db_dev.get_table_metadata(&schema, &table, is_view);
        all_table_def.push(meta);
    }
    all_table_def
}


/// get the database schema
pub fn get_schemas(all_table: &Vec<Table>) -> Vec<String> {
    let mut schema_names = Vec::new();
    for t in all_table {
        if !schema_names.contains(&t.schema) {
            schema_names.push(t.schema.clone());
        }
    }
    schema_names.sort_by(|a, b| a.cmp(b));
    schema_names
}

/// get all tables with schema name
pub fn get_tables_in_schema<'a>(schema: &str, all_table: &'a Vec<Table>) -> Vec<&'a Table> {
    let mut tables = Vec::new();
    for t in all_table {
        if &t.schema == schema {
            tables.push(t);//cloned the table here
        }
    }
    tables.sort_by(|a, b| a.name.cmp(&b.name));
    tables
}
/// this is the default config generation

pub fn generate_all(db_dev: &DatabaseDev, config: &Config) {
    generate_tables(db_dev, vec![], config)
}

/// if only_tables is empty vec, then generate_all
pub fn generate_tables(db_dev: &DatabaseDev, only_tables: Vec<String>, config: &Config) {
    let all_tables: Vec<Table> = get_all_tables(db_dev);
    let mut tables = vec![];
    if only_tables.is_empty() {
        for table in all_tables {
            tables.push(table);
        }
    } else {
        for table in all_tables {
            if only_tables.contains(&table.name) {
                tables.push(table);
            }
        }
    }
    for table in &tables {
        generate_table(db_dev, config, table, &tables);
    }
    generate_mod_per_schema(&config, &tables);
    generate_mod_rs(&config, &tables);
    generate_mod_table_names(&config, &tables);
    generate_mod_column_names(&config, &tables);
    generate_mod_schema_names(&config, &tables);
}

/// the gernaration of tables should be placed on their respective directory
/// base_mod::schema::table.rs
fn generate_table(db_dev: &DatabaseDev, config: &Config, table: &Table, all_tables: &Vec<Table>) {
    let mut w = Writer::new();
    let (struct_imports, imported_tables, struct_src) = table.struct_code(db_dev, all_tables);
    let (dao_imports, dao_src) = generate_dao_conversion_code(config, table, all_tables);
    let (meta_imports, meta_src) = generate_meta_code(table);
    let (json_imports, json_src) = generate_to_json_code(table);
    let static_columns = generate_static_column_names(table);
    let warning = format!(" WARNING: This file is generated, derived from table {}, DO NOT EDIT",
                          table.complete_name());
    w.inner_doc_comment(&warning);
    w.ln();
    w.ln();
    for i in struct_imports {
        w.appendln(&format!("use {};", i));
    }

    for it in imported_tables {
        if it != table {
            w.appendln(&format!("use {};", config.table_module(it)));
        }
    }
    for i in dao_imports {
        w.appendln(&format!("use {};", i));
    }
    for i in meta_imports {
        w.appendln(&format!("use {};", i));
    }
    for i in json_imports {
        w.appendln(&format!("use {};", i));
    }
    w.ln();
    w.ln();
    w.appendln(&struct_src);
    w.ln();
    w.appendln(&dao_src);
    w.ln();
    w.appendln(&json_src);
    w.ln();
    w.append(&meta_src);
    w.ln();
    w.appendln(&static_columns);

    let module_dir = config.module_dir(&table.schema);
    match fs::create_dir_all(&module_dir) {
        Ok(_) => {
            let file = format!("{}/{}.rs", &module_dir, table.name);
            save_to_file(&file, &w.src);
        }
        Err(e) => println!("There was an error creating directory {}", e),
    }
}


fn generate_mod_per_schema(config: &Config, all_tables: &Vec<Table>) {
    let schemas = get_schemas(all_tables);
    for schema in schemas {
        let mut w = Writer::new();
        let module_dir = config.module_dir(&schema);
        let tables = get_tables_in_schema(&schema, all_tables);
        for table in &tables {
            w.appendln(&format!("pub mod {};", table.name));
        }
        for table in &tables {
            //re-export structs
            w.appendln(&format!("pub use self::{}::{};",
                                table.name,
                                table.struct_name()));
        }
        let mod_file = format!("{}/mod.rs", module_dir);
        save_to_file(&mod_file, &w.src);
    }
}

/// listing down schemas in the database
fn generate_mod_schema_names(config: &Config, all_tables: &Vec<Table>) {
    let schemas = get_schemas(all_tables);
    let mut w = Writer::new();
    for schema in schemas {
        w.ln();
        w.appendln("#[allow(non_upper_case_globals)]");
        w.appendln(&format!("pub const {}: &'static str = \"{}\";", schema, schema));
    }
    let module_dir = config.module_base_dir();
    let mod_file = format!("{}/schema.rs", module_dir);
    save_to_file(&mod_file, &w.src);
}
/// listing of all table names in the system
fn generate_mod_table_names(config: &Config, all_tables: &Vec<Table>) {
    let mut unique_table = vec![];
    for table in all_tables {
        unique_table.push(table.name.to_owned());
    }
    unique_table.sort_by(|a, b| a.cmp(b));
    unique_table.dedup();

    let mut w = Writer::new();
    for table_name in unique_table {
        w.ln();
        w.appendln("#[allow(non_upper_case_globals)]");
        w.appendln(&format!("pub const {}: &'static str = \"{}\";",
                            table_name,
                            table_name));
    }
    let module_dir = config.module_base_dir();
    let mod_file = format!("{}/table.rs", module_dir);
    save_to_file(&mod_file, &w.src);
}

/// listing of all column names in the system
fn generate_mod_column_names(config: &Config, all_tables: &Vec<Table>) {
    let mut unique_columns = vec![];
    for table in all_tables {
        for column in &table.columns {
            unique_columns.push((column.corrected_name(), column.name.to_owned()));
        }
    }
    unique_columns.sort_by(|a, b| a.cmp(b));
    unique_columns.dedup();

    let mut w = Writer::new();
    for (corrected_name, column_name) in unique_columns {
        w.ln();
        w.appendln("#[allow(non_upper_case_globals)]");
        w.appendln(&format!("pub const {}: &'static str = \"{}\";",
                            corrected_name,
                            column_name));
    }
    let module_dir = config.module_base_dir();
    let mod_file = format!("{}/column.rs", module_dir);
    save_to_file(&mod_file, &w.src);
}


fn generate_mod_rs(config: &Config, all_tables: &Vec<Table>) {
    let mut w = Writer::new();
    let schemas = get_schemas(&all_tables);
    for schema in &schemas {
        w.append("pub mod ");
        w.append(schema);
        w.appendln(";");
    }
    w.ln();
    w.appendln("pub mod schema;");
    w.appendln("pub mod table;");
    w.appendln("pub mod column;");
    w.ln();
    w.appendln("use rustorm::table::Table;");
    w.appendln("use rustorm::table::IsTable;");
    for table in all_tables {
        let table_mod = format!("use {};", config.table_module(table));
        w.appendln(&table_mod);
    }
    let mod_file = format!("{}/mod.rs", config.module_base_dir());
    let all_table_fn = &generate_fn_get_all_tables(&all_tables);
    w.append(all_table_fn);
    save_to_file(&mod_file, &w.src);

}



fn generate_meta_code(table: &Table) -> (Vec<String>, String) {
    let mut w = Writer::new();
    let mut imports = Vec::new();
    imports.push("rustorm::table::IsTable".to_owned());
    imports.push("rustorm::table::Column".to_owned());
    imports.push("rustorm::table::Table".to_owned());

    w.append("impl IsTable for ");
    w.append(&table.struct_name());
    w.append("{");
    w.ln();
    w.ln();
    w.tab();
    w.append("fn table()->Table{");
    w.ln();
    w.tab();
    let (table_imports, table_src) = table.meta_code();
    for imp in table_imports {
        imports.push(imp);
    }
    w.append(&table_src);
    w.ln();
    w.tab();
    w.append("}");
    w.ln();
    w.append("}");
    (imports, w.src)
}

fn generate_to_json_code(table: &Table) -> (Vec<String>, String) {
    let mut w = Writer::new();
    let mut imports = Vec::new();
    imports.push("rustc_serialize::json::ToJson".to_owned());
    imports.push("rustc_serialize::json::Json".to_owned());

    w.append("impl ToJson for ");
    w.append(&table.struct_name());
    w.append("{");
    w.ln();
    w.ln();
    w.tab();
    w.append("fn to_json(&self)->Json{");
    w.ln();
    w.tabs(2);
    w.append("self.to_dao().to_json()");
    w.ln();
    w.tab();
    w.append("}");
    w.ln();
    w.append("}");
    (imports, w.src)
}

fn generate_static_column_names(table: &Table) -> String {
    let mut w = Writer::new();
    w.comment(" Generated columns for easier development of dynamic queries without sacrificing \
               wrong spelling of column names");
    for column in &table.columns {
        w.ln();
        w.ln();
        w.appendln("#[allow(non_upper_case_globals)]");
        w.appendln("#[allow(dead_code)]");
        w.append("pub static ");
        w.append(&column.corrected_name());
        w.append(": &'static str = ");
        w.append(&format!("\"{}.{}\"", table.name, column.name));
        w.append(";");
    }
    w.src
}

/// TODO: if column names begins with the tablename_, then put this value to the column name hash map
/// example: product_name, value will be copied to name
/// test if product_name is not a column, split with `_`, then check if first splinter is the tablename, then the check if the 2nd splinter if 
/// a column of this table, then set that column with the value of the original name 
fn generate_dao_conversion_code(config: &Config,
                                table: &Table,
                                all_tables: &Vec<Table>)
                                -> (Vec<String>, String) {
    let mut w = Writer::new();
    let mut imports = Vec::new();
    imports.push("rustorm::dao::Dao".to_owned());
    imports.push("rustorm::dao::IsDao".to_owned());
    imports.push(format!("{}::schema", config.base_module.as_ref().unwrap()));
    imports.push(format!("{}::table", config.base_module.as_ref().unwrap()));
    imports.push(format!("{}::column", config.base_module.as_ref().unwrap()));

    w.ln();
    w.append("impl IsDao for ");
    w.append(&table.struct_name());
    w.append("{");
    w.ln_tab();
    w.append("fn from_dao(dao:&Dao)->Self{");
    w.ln_tabs(2);
    w.append(&table.struct_name());
    w.append("{");
    for c in &table.columns {
        w.ln_tabs(3);
        let column_name = format!("column::{}", &c.corrected_name());
        w.append(&c.corrected_name());
        w.append(": ");
        if c.not_null {
            w.append("dao.get");
        } else {
            w.append("dao.get_opt");
        }
        w.append("(");
        w.append(&column_name);
        w.append(")");
        w.comma();
    }
    let referenced_tables = table.get_all_applicable_reference(all_tables);
    for ref_table in referenced_tables {
        let member_name = ref_table.member_name(table);
        w.ln_tabs(3);
        w.append(&member_name);
        w.append(": ");
        if ref_table.is_has_one {
            w.append("None");
        }
        if ref_table.is_ext {
            w.append("None");
        }
        if ref_table.is_has_many {
            w.append("vec![]");
        }
        w.comma();
    }
    w.ln_tabs(2);
    w.append("}");
    w.ln_tab();
    w.append("}");
    w.ln();
    w.ln_tab();
    w.append("fn to_dao(&self)->Dao{");
    w.ln_tabs(2);
    w.append("let mut dao = Dao::new();");
    for c in &table.columns {
        w.ln_tabs(2);
        let column_name = format!("column::{}", &c.corrected_name());
        if c.not_null {
            w.append("dao.set(");
            w.append(&column_name);
            w.append(", &self.");
            w.append(&c.corrected_name());
            w.append(");");
        } else {
            w.append("match self.");
            w.append(&c.corrected_name());
            w.append("{");
            w.ln_tabs(3);
            w.append("Some(ref _value) => dao.set(");
            w.append(&column_name);
            w.append(", _value),");
            w.ln_tabs(3);
            w.append("None => dao.set_null(");
            w.append(&column_name);
            w.append(")");
            w.ln_tabs(2);
            w.append("}");
        }
    }
    w.ln_tabs(2);
    w.append("dao");
    w.ln_tab();
    w.append("}");
    w.ln();
    w.append("}");
    (imports, w.src)
}

fn generate_fn_get_all_tables(tables: &Vec<Table>) -> String {
    let mut w = Writer::new();
    w.ln();
    w.ln();
    w.append("pub fn get_all_tables()->Vec<Table>{");
    w.ln();
    w.tab();
    w.append("vec![");
    for t in tables {
        w.ln();
        w.tabs(2);
        w.append(&t.struct_name());
        w.append("::table(),");
    }
    w.ln();
    w.tab();
    w.append("]");
    w.ln();
    w.append("}");
    w.src
}



fn save_to_file(filename: &str, content: &str) {
    match File::create(filename) {
        Err(_) => panic!("couldn't create file {}", filename),
        Ok(mut file) => {
            match file.write_all(content.as_bytes()) {
                Ok(_) => {
                    println!("Saved to {}", filename);
                }
                Err(_) => {
                    println!("There was error saving to file: {}", filename)
                }
            };
        }
    };

}
