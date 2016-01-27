pub mod base;
pub mod record;
pub use self::base::Base;
pub use self::record::Record;
use rustorm::table::Table;
use rustorm::table::IsTable;


pub fn base()->Table{
    Base::table()
}

pub fn record()->Table{
    Record::table()
}
