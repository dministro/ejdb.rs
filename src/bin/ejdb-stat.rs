#[cfg(feature = "bson_0_13")]
pub extern crate dep_bson_0_13 as bson;
#[cfg(feature = "bson_1_2")]
pub extern crate dep_bson_1_2 as bson;
extern crate ejdb;

use std::env;
use std::io::Write;

use ejdb::Database;

macro_rules! abort {
    ($code:expr, $($args:tt)*) => {{
        let _ = writeln!(&mut ::std::io::stderr(), $($args)*);
        ::std::process::exit($code);
    }}
}

fn main() {
    let db_path = env::args()
        .nth(1)
        .unwrap_or_else(|| abort!(1, "Usage: ejdb-stat <database>"));

    let db = Database::open(db_path).unwrap_or_else(|e| abort!(1, "Error opening database: {}", e));

    println!("Metadata:");
    let meta = db
        .get_metadata()
        .unwrap_or_else(|e| abort!(1, "Error loading metadata: {}", e));
    println!("{}", bson::Bson::Document(meta.into_inner()));
}
