#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate derivative;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod api_v1;
mod captcha;
mod config;
mod context;
mod db;
mod errors;
mod models;
mod schema;
mod utils;
mod views;

use rocket_contrib::{serve::StaticFiles, templates::Template};
use structopt::StructOpt;

use crate::captcha::Captcha;
use crate::config::Config;
use crate::context::Context;
use crate::db::DbConn;

#[derive(StructOpt)]
enum Opt {
    #[structopt(name = "generate-config")]
    GenerateConfig,

    #[structopt(name = "migrate")]
    Migrate,

    #[structopt(name = "runserver")]
    RunServer,
}

fn main() {
    let opt = Opt::from_args();
    if let Opt::GenerateConfig = opt {
        let config = Config::default();
        let output = toml::to_string_pretty(&config).expect("default config should not panic");
        println!("{}", output);
        return;
    }

    let config = Config::read().unwrap();
    let rocket = rocket::ignite()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .mount("/api/v1", api_v1::routes())
        .mount("/static", StaticFiles::from("static"))
        .mount("/", views::routes());

    match opt {
        Opt::Migrate => {
            let db = DbConn::get_one(&rocket).expect("failed to get db");
            db.migrate();
        }
        Opt::RunServer => {
            error!("{}", rocket.launch());
        }
        Opt::GenerateConfig => unreachable!(),
    }
}
