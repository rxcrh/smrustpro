use rusqlite::{Connection, Result};
use std::{io, println};
use dirs;
use fallible_iterator::FallibleIterator;


fn id_exists(id: &str, conn: &Connection) -> Result<bool, Box<dyn std::error::Error>> {
    let mut ids = get_ids(&conn)?;
    ids.retain(|&x| x == id.parse().unwrap());

    if ids.is_empty() {
        return Ok(false);
    }

    Ok(true)
}

fn get_ids(conn: &Connection) -> Result<Vec<usize>, Box<dyn std::error::Error>> {

    let mut stmt = conn.prepare("SELECT id FROM notes")?;
    let rows = stmt.query(rusqlite::params![])?;

    let ids = rows.map(|row| row.get(0)).collect::<Vec<usize>>()?;
    Ok(ids)
}

fn get_entrys(conn: &Connection) -> Result<Vec<String>, Box<dyn std::error::Error>> {

    let mut stmt = conn.prepare("SELECT body FROM notes")?;
    let rows = stmt.query(rusqlite::params![])?;

    let entrys = rows.map(|row| row.get(0)).collect::<Vec<String>>()?;
    Ok(entrys)
}

fn try_to_delete(msg: &str, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {

    if !id_exists(msg, &conn)? {
        println!("Could not delete entry, because it doesn't exist!");
        return Ok(());
    }

    conn.execute("DELETE FROM notes WHERE id = (?1)", [msg])?;

    Ok(())
}

fn try_to_update(msg: &str, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {

    let msg_split = match msg.split_once(" ") {
        Some(msg_split) => msg_split,
        None => {
            println!("Your entry is not valid!");
            return Ok(());
        }
    };

    let id = msg_split.0;
    let body = msg_split.1;

    if !id_exists(id, &conn)? {
        println!("The given id does not exist!");
        return Ok(());
    }

    conn.execute("UPDATE notes SET body = (?1) WHERE id = (?2)", [body, id])?;
    Ok(())
}

fn try_to_list(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {

    let ids = get_ids(&conn)?;
    let bodys = get_entrys(&conn)?;

    for (id, entry) in ids.iter().zip(bodys) {
        println!("{}: {}", id, entry);
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

    println!("/help for all the commands");

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
        } else if cmd == "/help" || cmd == "/h" {
            println!("\t /del [entry] for deleting an entry");
            println!("\t /edit [id] [entry] for editing an entry");
            println!("\t /list for listing all entrys");
            println!("\t Hint: /d, /e and /l also work");
        } else {
            try_to_create(trimmed_body, &conn)?;
        }
    }

    Ok(())

}


