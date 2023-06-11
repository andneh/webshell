use rusqlite::{Connection, Result};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}
fn create_table_if_not_exists(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn insert_user(conn: &Connection, name: &str, email: &str) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        &[name, email],
    )?;

    Ok(())
}

fn get_all_users(conn: &Connection) -> Result<Vec<User>> {
    let mut stmt = conn.prepare("SELECT id, name, email FROM users")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    })?;

    let mut users = Vec::new();
    for user_result in user_iter {
        users.push(user_result?);
    }

    Ok(users)
}

fn main() -> Result<()> {
    let conn = Connection::open("../db.sqlite")?;

    create_table_if_not_exists(&conn)?;

    insert_user(&conn, "Alice", "alice@example.com")?;
    insert_user(&conn, "Bob", "bob@example.com")?;

    let users = get_all_users(&conn)?;
    for user in users {
        println!("{:?}", user);
    }

    Ok(())
}
