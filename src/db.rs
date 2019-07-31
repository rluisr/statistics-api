use mysql;
use std::env;
use std::process;

pub fn connect() -> mysql::Pool {
    let db_url = match env::var("DB_URL") {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, "DB_URL");
            process::exit(1);
        },
    };

    return mysql::Pool::new(db_url).unwrap();
}