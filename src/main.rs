use actix_files::Files;
use actix_web::{App, HttpServer, main, middleware::Logger};
use clap::Parser;

use env_logger::Builder;
use log::LevelFilter;
use log::info;

use crate::routers::home_route;
mod components;
mod layouts;
mod routers;
mod utils;
mod views;

#[derive(Debug, clap::Parser)]
#[command(name = "mage_gojo_blog", version, about = "个人博客")]
struct Cli {
    #[arg(short = 'H', long = "host", default_value = "127.0.0.1")]
    host: String,
    #[arg(short = 'p', long = "port", default_value_t = 8888)]
    port: u16,
}

fn init_logger() {
    Builder::new()
        // 不依赖 RUST_LOG，直接写死级别
        .filter_level(LevelFilter::Info)
        .init();
}

#[main]
async fn main() -> std::io::Result<()> {
    init_logger();
    let cli = Cli::parse();
    let bind_addr = format!("{}:{}", cli.host, cli.port);
    //  初始化日志
    info!("启动成功！");
    info!("服务器监听地址: http://{}", bind_addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(r#"%a "%{User-Agent}i" %m %U %Dms"#))
            .service(home_route)
            .service(Files::new("/assets", "./assets"))
    })
    .bind(bind_addr)?
    .run()
    .await
}
