use rusqlite::{Connection, Result, params};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use crypto::symmetriccipher;
use crypto::blockmodes::NoPadding;
use crypto::aessafe::AesSafe128Encryptor;
use crypto::aessafe::AesSafe128Decryptor;
use crypto::aes::KeySize::KeySize128;

fn select_cert(conn: &Connection, name: &str) -> Result<Vec<u8>> {
    let mut stmt = conn.prepare("SELECT cert FROM certs WHERE name = ?")?;
    let cert: Vec<u8> = stmt.query_row(params![name], |row| row.get(0))?;
    Ok(cert)
}


fn select_cert_and_encrypt_input(conn: &Connection, user_input: &str) -> Result<()> {
    let cert =  select_cert(conn, "CrypterClient").unwrap();




    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open("../data.sqlite")?;

    let cert = select_cert(&conn, "CrypterClient")?;
    println!("Cert: {:?}", cert);

    // let user_input = "Hello, World!";

    // select_cert_and_encrypt_input(&conn, user_input)?;

    Ok(())
}
