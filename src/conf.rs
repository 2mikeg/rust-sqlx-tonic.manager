use std::env;

pub struct Conf {
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
}

pub fn load_env() -> Conf {
    let db_host = env::var("DB_HOST").expect("DB_HOST var not found");
    let db_port = env::var("DB_PORT").expect("DB_PORT var not found");
    let db_user = env::var("DB_USER").expect("DB_USER var not found");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD var not found");
    let db_name = env::var("DB_NAME").expect("DB_NAME var not found");

    let conf = Conf {
        db_host,
        db_port,
        db_user,
        db_password,
        db_name,
    };

    conf
}
