pub mod country;
pub mod currency;
pub mod exchange_rate;
pub use self::country::Country;
pub use self::currency::Currency;
pub use self::exchange_rate::ExchangeRate;
use rustorm::table::Table;
use rustorm::table::IsTable;


pub fn country()->Table{
    Country::table()
}

pub fn currency()->Table{
    Currency::table()
}

pub fn exchange_rate()->Table{
    ExchangeRate::table()
}
