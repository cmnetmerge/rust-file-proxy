use actix_web::{App, HttpServer, middleware, web};
use actix_web::http::ContentEncoding;
use deadpool_postgres::{Manager, Pool};
use serde_json::from_str;
use tokio_postgres::{Config, NoTls};
use crate::config::IConfig;

mod models;
mod config;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let cfg: config::Config = config::Config {};
    let mut postgres_cfg = Config::new();
    postgres_cfg.host(cfg.get_config_with_key("DATABASE_HOST").as_str());
    postgres_cfg.user(cfg.get_config_with_key("DATABASE_USER").as_str());
    postgres_cfg.password(cfg.get_config_with_key("DATABASE_PASSWORD").as_str());
    postgres_cfg.port(from_str(cfg.get_config_with_key("DATABASE_PORT").as_str()).unwrap());
    postgres_cfg.dbname(cfg.get_config_with_key("DATABASE_DBNAME").as_str());

    let mgr = Manager::new(postgres_cfg, NoTls);
    let pool = Pool::new(mgr, 15);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .wrap(middleware::Logger::default())
            .service(web::scope("/token"))
    })
        .bind("127.0.0.1:8090")?
        .run()
        .await
}
