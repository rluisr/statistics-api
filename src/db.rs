use mysql;

static DB_URL: &'static str = env!("DB_URL");

pub fn connect() -> mysql::Pool {
    return mysql::Pool::new(DB_URL).unwrap();
}