use mysql;

static DB_URL: &'static str = env!("DB_URL");

pub fn connect() -> mysql::Pool {
    mysql::Pool::new(DB_URL).unwrap();
}