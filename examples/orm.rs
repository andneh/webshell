#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

mod schema {
    table! {
        users {
            id -> Integer,
            user_name -> Text,
            email -> Text,
        }
    }
}

use schema::users;

#[derive(Queryable, Insertable)]
#[table_name = "users"]
struct User {
    id: Option<i32>,
    user_name: String,
    email: String,
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn create_user(conn: &SqliteConnection, name: &str, email: &str) -> User {
    use schema::users::dsl::*;

    let new_user = User {
        id: None,
        user_name: name.to_owned(),
        email: email.to_owned(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Error inserting user");

    users.order(id.desc()).first(conn).unwrap()
}

fn main() {
    let conn = establish_connection();

    let user = create_user(&conn, "John Doe", "john@example.com");
    println!("Created user: {:?}", user);
}
