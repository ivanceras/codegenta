pub mod address;
pub mod api_key;
pub mod cart;
pub mod cart_line;
pub mod category;
pub mod client;
pub mod invoice;
pub mod order_line;
pub mod orders;
pub mod organization;
pub mod photo;
pub mod photo_sizes;
pub mod product;
pub mod product_availability;
pub mod product_category;
pub mod product_photo;
pub mod product_review;
pub mod review;
pub mod settings;
pub mod user_info;
pub mod user_location;
pub mod user_review;
pub mod users;
pub mod wishlist;
pub mod wishlist_line;
pub use self::address::Address;
pub use self::api_key::ApiKey;
pub use self::cart::Cart;
pub use self::cart_line::CartLine;
pub use self::category::Category;
pub use self::client::Client;
pub use self::invoice::Invoice;
pub use self::order_line::OrderLine;
pub use self::orders::Orders;
pub use self::organization::Organization;
pub use self::photo::Photo;
pub use self::photo_sizes::PhotoSizes;
pub use self::product::Product;
pub use self::product_availability::ProductAvailability;
pub use self::product_category::ProductCategory;
pub use self::product_photo::ProductPhoto;
pub use self::product_review::ProductReview;
pub use self::review::Review;
pub use self::settings::Settings;
pub use self::user_info::UserInfo;
pub use self::user_location::UserLocation;
pub use self::user_review::UserReview;
pub use self::users::Users;
pub use self::wishlist::Wishlist;
pub use self::wishlist_line::WishlistLine;
use rustorm::table::Table;
use rustorm::table::IsTable;


pub fn address()->Table{
    Address::table()
}

pub fn api_key()->Table{
    ApiKey::table()
}

pub fn cart()->Table{
    Cart::table()
}

pub fn cart_line()->Table{
    CartLine::table()
}

pub fn category()->Table{
    Category::table()
}

pub fn client()->Table{
    Client::table()
}

pub fn invoice()->Table{
    Invoice::table()
}

pub fn order_line()->Table{
    OrderLine::table()
}

pub fn orders()->Table{
    Orders::table()
}

pub fn organization()->Table{
    Organization::table()
}

pub fn photo()->Table{
    Photo::table()
}

pub fn photo_sizes()->Table{
    PhotoSizes::table()
}

pub fn product()->Table{
    Product::table()
}

pub fn product_availability()->Table{
    ProductAvailability::table()
}

pub fn product_category()->Table{
    ProductCategory::table()
}

pub fn product_photo()->Table{
    ProductPhoto::table()
}

pub fn product_review()->Table{
    ProductReview::table()
}

pub fn review()->Table{
    Review::table()
}

pub fn settings()->Table{
    Settings::table()
}

pub fn user_info()->Table{
    UserInfo::table()
}

pub fn user_location()->Table{
    UserLocation::table()
}

pub fn user_review()->Table{
    UserReview::table()
}

pub fn users()->Table{
    Users::table()
}

pub fn wishlist()->Table{
    Wishlist::table()
}

pub fn wishlist_line()->Table{
    WishlistLine::table()
}
