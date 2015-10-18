use writer::Writer;
use rustorm::database::DatabaseDev;

use rustorm::table::{Column, Table, Foreign};

pub trait MetaCode{
    /// meta code describes the meta table and produce a Table instance definition
    fn meta_code(&self) -> (Vec<String>, String);
}


impl MetaCode for Foreign{

    fn meta_code(&self) -> (Vec<String>, String) {

        let mut w = Writer::new();
        w.ln();
        w.tabs(6);
        w.append("Foreign {");
        w.ln();
        w.tabs(7);
        w.append("schema: ");
        w.append(&format!("\"{}\".to_owned(),", self.schema));
        w.ln();
        w.tabs(7);
        w.append("table: ");
        w.append(&format!("\"{}\".to_owned(),", self.table));
        w.ln();
        w.tabs(7);
        w.append("column: ");
        w.append(&format!("\"{}\".to_owned(),", self.column));
        w.ln();
        w.tabs(6);
        w.append("}");
        (vec!["rustorm::table::Foreign".to_owned()], w.src)

    }

}

impl MetaCode for Column{

    fn meta_code(&self) -> (Vec<String>, String) {
        let mut imports = vec![];
        let mut w = Writer::new();
        w.ln();
        w.tabs(4);
        w.append("Column {");
        w.ln();
        w.tabs(5);
        w.append("name: ");
        w.append(&format!("column::{}.to_owned(),", self.corrected_name()));
        w.ln();
        w.tabs(5);
        w.append("data_type: ");
        w.append(&format!("\"{}\".to_owned(),", self.data_type));
        w.ln();
        w.tabs(5);
        w.append("db_data_type: ");
        w.append(&format!("\"{}\".to_owned(),", self.db_data_type));
        w.ln();
        w.tabs(5);
        w.append("is_primary: ");
        w.append(&format!("{}, ", self.is_primary));
        w.append("is_unique: ");
        w.append(&format!("{}, ", self.is_unique));
        w.append("not_null: ");
        w.append(&format!("{}, ", self.not_null));
        w.append("is_inherited: ");
        w.append(&format!("{},", self.is_inherited));
        w.ln();
        w.tabs(5);
        w.append("default: ");
        if self.default.is_some() {
            w.append(&format!("Some(\"{}\".to_owned()),",
                              &self.default.clone().unwrap()));
        } else {
            w.append("None,");
        }
        w.ln();
        w.tabs(5);
        w.append("comment: ");
        if self.comment.is_some() {
            w.append(&format!("Some(\"{}\".to_owned()),",
                              &self.comment
                                   .clone()
                                   .unwrap()
                                   .replace("\"", "\\\"")
                                   .replace("\n", "\\n")));
        } else {
            w.append("None,");
        }
        w.ln();
        w.tabs(5);
        w.append("foreign: ");
        if self.foreign.is_some() {
            let (foreign_imports, foreign_src) = self.foreign.as_ref().unwrap().meta_code();
            for imp in foreign_imports {
                imports.push(imp);
            }
            w.append(&format!("Some({}),", foreign_src));
        } else {
            w.append("None,");
        }
        w.ln();
        w.tabs(4);
        w.append("}");
        imports.sort_by(|a, b| a.cmp(&b));
        imports.dedup();
        (imports, w.src)
    }

}

impl MetaCode for Table{

    /// build a source code which express it self as a table object
    /// which is a meta definition of the struct itself
    fn meta_code(&self) -> (Vec<String>, String) {
        let mut imports = vec![];
        let mut w = Writer::new();
        w.ln();
        w.tabs(2);
        w.append("Table {");
        w.ln();
        w.tabs(3);
        w.append("schema: ");
        w.append(&format!("schema::{}.to_owned(),", self.schema));
        w.ln();
        w.tabs(3);
        w.append("name: ");
        w.append(&format!("table::{}.to_owned(),", self.name));
        w.ln();
        w.tabs(3);
        w.append("parent_table: ");
        if self.parent_table.is_some() {
            w.append(&format!("Some(table::{}.to_owned()),",
                              &self.parent_table.clone().unwrap()));
        } else {
            w.append("None,");
        }
        w.ln();
        w.tabs(3);
        w.append("sub_table: ");
        if !self.sub_table.is_empty() {
            let sub_table = self.sub_table.clone();
            w.append("vec![");
            for s in sub_table {
                w.append(&format!("table::{}.to_owned(),", s));
            }
            w.append("],");
        } else {
            w.append("vec![],");
        }
        w.ln();
        w.tabs(3);
        w.append("comment: ");
        if self.comment.is_some() {
            // TODO: use r# for raw formatting
            w.append(&format!("Some(\"{}\".to_owned()),",
                              &self.comment
                                   .clone()
                                   .unwrap()
                                   .replace("\"", "\\\"")
                                   .replace("\n", "\\n")));
        } else {
            w.append("None,");
        }
        w.ln();
        w.tabs(3);
        w.append("columns: vec![");
        for c in &self.columns {
            let (column_imports, column_src) = c.meta_code();
            for imp in column_imports {
                imports.push(imp);
            }
            w.append(&column_src);
            w.append(",");
        }
        w.ln();
        w.tabs(3);
        w.append("],");
        w.ln();
        w.tabs(3);
        w.append(&format!("is_view: {},", self.is_view));
        w.ln();
        w.tabs(2);
        w.append("}");
        imports.sort_by(|a, b| a.cmp(&b));
        imports.dedup();
        (imports, w.src)
    }

}


pub trait StructCode{
    /// the struct code/module code of the table, which can easily be used as struct of the project
    fn struct_code<'a>(&'a self,
                       db: &DatabaseDev,
                       all_tables: &'a Vec<Table>)
                       -> (Vec<String>, Vec<&'a Table>, String);
    /// auxilliary function to write the column of tables
    fn write_column(w: &mut Writer, c: &Column);
}

impl StructCode for Table{
    /// build a source code for the struct defined by this table
    ///(imports, imported_tables, source code)
    fn struct_code<'a>(&'a self,
                       db: &DatabaseDev,
                       all_tables: &'a Vec<Table>)
                       -> (Vec<String>, Vec<&'a Table>, String) {
        let mut w = Writer::new();
        //imported tables needed since we are partitioning the tables in schemas
        let mut imported_tables = Vec::new();
        //imports
        let mut imports: Vec<String> = Vec::new();
        for c in &self.columns {
            let (package, _) = db.dbtype_to_rust_type(&c.db_data_type);
            if !package.is_empty() {
                for i in package {
                    imports.push(i);
                }
            }
        }
        imports.sort_by(|a, b| a.cmp(b));
        imports.dedup();

        //struct
        let struct_name = self.struct_name();
        w.ln();
        if self.comment.is_some() {
            w.append("///");
            w.ln();
            w.doc_comment(&self.comment.clone().unwrap());
            w.ln();
            w.append("///");
            w.ln();
        }
        w.append("#[derive(RustcDecodable, RustcEncodable)]");
        w.ln();
        w.append("#[derive(Debug, Clone)]");
        w.ln();
        w.append("pub struct ").append(&struct_name).appendln(" {");

        let mut included_columns = Vec::new();
        //primary columns
        for p in self.primary_columns() {
            if !included_columns.contains(&p.name) {
                w.tab();
                w.append("/// primary");
                w.ln();
                Self::write_column(&mut w, p);
                included_columns.push(p.name.clone());
            }
        }
        //unique columns
        for u in self.unique_columns() {
            if !included_columns.contains(&u.name) {
                w.tab();
                w.append("/// unique");
                w.ln();
                Self::write_column(&mut w, u);
                included_columns.push(u.name.clone());
            }
        }

        // uninherited columns
        for uc in &self.uninherited_columns() {
            if !included_columns.contains(&uc.name) {
                Self::write_column(&mut w, uc);
                included_columns.push(uc.name.clone());
            }
        }

        // inherited columns
        for ic in &self.inherited_columns() {
            if !included_columns.contains(&ic.name) {
                Self::write_column(&mut w, ic);
                included_columns.push(ic.name.clone());
            }
        }
        let referenced_table = self.get_all_applicable_reference(all_tables);
        for ref_table in referenced_table {
            w.ln_tab();
            w.append("/// ");
            let comment = if ref_table.is_has_one {
                if ref_table.table != self {
                    "has one"
                } else {
                    "has one, self referential"
                }
            } else if ref_table.is_ext {
                "has one, extension table"
            } else if ref_table.is_has_many {
                if ref_table.is_direct {
                    "has many"
                } else {
                    "has many, indirect"
                }
            } else {
                unreachable!();
            };
            w.append(comment);
            w.ln_tab();
            let member_name = ref_table.member_name(self);
            w.append("pub ");
            w.append(&member_name);
            w.append(": ");
            if ref_table.is_has_one {
                if ref_table.table != self {
                    w.append("Option<");
                    w.append(&ref_table.table.struct_name());
                    w.append(">");
                } else {
                    w.append("Option<Box<");
                    w.append(&ref_table.table.struct_name());
                    w.append(">>");
                }
            }
            if ref_table.is_ext {
                w.append("Option<Box<");
                w.append(&ref_table.table.struct_name());
                w.append(">>");
            }
            if ref_table.is_has_many {
                w.append("Vec<");//put it inside the box to get rid of illegal recursive struct
                w.append(&ref_table.table.struct_name());
                w.append(">");
            }
            w.comma();
            imported_tables.push(ref_table.table);
        }
        w.ln();
        w.append("}");
        w.ln();
        imported_tables.sort_by(|a, b| (a.complete_name().cmp(&b.complete_name())));
        imported_tables.dedup();

        (imports, imported_tables, w.src)
    }

    fn write_column(w: &mut Writer, c: &Column) {
        if c.comment.is_some() {
            let comment = &c.comment.clone().unwrap();
            for split in comment.split("\n") {
                w.tab();
                w.append("/// ");
                w.append(split);
                w.ln();
            }
        }
        if c.default.is_some() {
            let default = &c.default.clone().unwrap();
            for split in default.split("\n") {
                w.tab();
                w.append("/// default: ");
                w.append(split);
                w.ln();
            }
        }
        if c.not_null {
            w.tab();
            w.append("/// not nullable ");
            w.ln();
        }
        if c.is_inherited {
            w.tab();
            w.append("/// --inherited-- ");
            w.ln();
        }
        w.tab();
        w.append("/// db data type: ");
        w.append(&c.db_data_type);
        w.ln();

        w.tab();
        w.append("pub ");
        w.append(&c.corrected_name());
        w.append(": ");
        if c.not_null {
            w.append(&c.data_type);
        } else {
            w.append("Option<");
            w.append(&c.data_type);
            w.append(">");
        }
        w.comma();
        w.ln();
    }

}
