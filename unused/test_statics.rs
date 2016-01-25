extern crate rustorm;
use rustorm::query::{Filter, Equality, Operand, ColumnName};
use std::ops::{Add, Sub};
use std::cmp::PartialEq;


pub struct Storage{
    filters: Vec<Filter>
}
    
impl Storage{
        
    fn store(filter: Filter){
        println!("storing.. {:?}",filter)
    }
}

#[derive(Debug)]
pub struct StaticColumn{
    pub name: &'static str,
    pub table: Option<&'static str>,
}

impl StaticColumn{
    
    pub fn complete_name(&self)->String{
        format!("{}.{}", self.name.to_string(), self.table.as_ref().unwrap().to_string())
    }
    
    pub fn __eq__(&self, other: &StaticColumn)->Filter{
        Filter::bare_new(
            Operand::ColumnName(ColumnName::from_str(&self.complete_name())), 
            Equality::EQ, 
            Operand::ColumnName(ColumnName::from_str(&other.complete_name())))
    }
    
    fn __ne__(&self, other: &StaticColumn) -> Filter {
        Filter::bare_new(
            Operand::ColumnName(ColumnName::from_str(&self.complete_name())), 
            Equality::NEQ, 
            Operand::ColumnName(ColumnName::from_str(&other.complete_name())))
    }
    
    pub fn __ge__(&self, other: &StaticColumn)->Filter{
        Filter::bare_new(
            Operand::ColumnName(ColumnName::from_str(&self.complete_name())), 
            Equality::GTE, 
            Operand::ColumnName(ColumnName::from_str(&other.complete_name())))
    }
}

impl <'a> Add for &'a StaticColumn {
    type Output = Filter;

    fn add(self, other: &'a StaticColumn) -> Filter {
        Filter::bare_new(
            Operand::ColumnName(ColumnName::from_str(&self.complete_name())), 
            Equality::EQ, 
            Operand::ColumnName(ColumnName::from_str(&other.complete_name())))
    }
}


impl <'a> Sub for &'a StaticColumn {
    type Output = Filter;

    fn sub(self, other: &'a StaticColumn) -> Filter {
        Filter::bare_new(
            Operand::ColumnName(ColumnName::from_str(&self.complete_name())), 
            Equality::NEQ, 
            Operand::ColumnName(ColumnName::from_str(&other.complete_name())))
    }
}



fn add_to_filter(filter: Filter){
    println!("--->>> Adding to filter: {:?}", filter);
    Storage::store(filter);
}

impl PartialEq for StaticColumn{
    
    
    fn eq(&self, other: &StaticColumn) -> bool{
        let filter = Filter::bare_new(
            Operand::ColumnName(ColumnName::from_str(&self.complete_name())), 
            Equality::EQ, 
            Operand::ColumnName(ColumnName::from_str(&other.complete_name())));
        add_to_filter(filter);
        true
    }

    fn ne(&self, other: &StaticColumn) -> bool {
        Filter::bare_new(
            Operand::ColumnName(ColumnName::from_str(&self.complete_name())), 
            Equality::NEQ, 
            Operand::ColumnName(ColumnName::from_str(&other.complete_name())));
        false
    }
}


#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static hello: &'static StaticColumn = &StaticColumn{name: "hello", table: Some("bazaar")};
pub static hi: &'static StaticColumn = &StaticColumn{name: "hi", table: Some("bazaar")};

fn main(){
    println!("name_column: {:?}", hello);
    println!("complete_name: {:?}", hello.complete_name());
    let filter_add: Filter = hello + hi;
    println!("Filter Add: {:?}", filter_add);
    let filter_sub: Filter = hello - hi;
    println!("Filter Sub: {:?}", filter_sub);
    
    hello == hi;
    
    let filter_eq = hello.__eq__(hi);
    println!("filter_eq : {:?} ", filter_eq);
    let filter_neq = hello.__ne__(hi);
    println!("filter_neq : {:?} ", filter_neq);
    
    let filter_gte = hello.__ge__(hi);
    println!("filter_gte : {:?} ", filter_gte);
}