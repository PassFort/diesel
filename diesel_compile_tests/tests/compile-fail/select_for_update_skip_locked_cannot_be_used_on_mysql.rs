#[macro_use]
extern crate diesel;

use diesel::*;
use diesel::mysql::MysqlConnection;

table! {
    users {
        id -> Integer,
    }
}

fn main() {
    let conn = MysqlConnection::establish("").unwrap();
    users::table
        .for_update()
        .skip_locked()
        .load(&conn)
        //~^ ERROR: E0271
        .unwrap();
}
