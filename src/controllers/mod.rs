use rusqlite::Connection;

pub mod cert;

trait ODA {
    fn get_oda(&self) -> String;
    fn create(conn: &Connection) -> Self;
    fn read(conn: &Connection) -> Self;
    fn update(conn: &Connection) -> Self;
    fn delete(conn: &Connection) -> Self;
}