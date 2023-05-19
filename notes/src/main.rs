use rusqlite::{Connection, Result};
use std::io;
use dirs;
use fallible_iterator::FallibleIterator;

fn try_to_delete(msg: &str, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT id FROM notes")?;
    let rows = stmt.query(rusqlite::params![])?;

    let mut element_with_id = rows.map(|row| row.get(0)).collect::<Vec<usize>>()?;
    element_with_id.retain(|&x| x == msg.parse().unwrap());

    if element_with_id.is_empty() {
        println!{"Could not delete entry, because it doesn't exist"};
        return Ok(());
    }

    conn.execute("DELETE FROM notes WHERE id = (?1)", [msg])?;

    Ok(())
}

fn try_to_update(msg: &str, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {

    let msg_split = msg.split_once(" ").unwrap();
    let id = msg_split.0;
    let body = msg_split.1;

    conn.execute("UPDATE notes SET body = (?1) WHERE id = (?2)", [body, id])?;
    Ok(())
}

fn try_to_list(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT id, body FROM notes")?;
    let mut rows = stmt.query(rusqlite::params![])?;

    while let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let body: String = row.get(1)?;
        println!("{} {}", id, body.to_string());
    }

    Ok(())
}

fn try_to_create(msg: &str, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute("INSERT INTO notes (body) values (?1)", [msg])?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(dirs::home_dir().unwrap().to_str().unwrap().to_owned() + "/notes.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id integer primary key,
            body text not null
        )",
        [],
    )?;

    let mut running = true;
    while running == true {

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        let trimmed_body = buffer.trim();
        let cmd_split = trimmed_body.split_once(" ");

        let mut cmd = trimmed_body;
        let mut msg = " ";

        if cmd_split != None {

            cmd = cmd_split.unwrap().0;
            msg = cmd_split.unwrap().1;

        }

        if cmd == "" || cmd == "/exit" || cmd == "/quit" {
            running = false;
        } else if cmd == "/del" || cmd == "/d" {
            try_to_delete(msg, &conn)?;
        } else if cmd == "/edit" || cmd == "/e" {
            try_to_update(msg, &conn)?;
        } else if cmd == "/list" || cmd == "/l" {
            try_to_list(&conn)?;
        } else {
            try_to_create(cmd, &conn)?;
        }
    }

    Ok(())

}


