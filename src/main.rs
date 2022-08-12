mod config {
    use serde::Deserialize;
    #[derive(Debug, Default, Deserialize)]
    pub struct ExampleConfig {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
}

use deadpool_postgres::Pool;
use sql::{get_next_page, get_prev_page, get_question_data_map, get_sql_for_q};
use std::{collections::HashMap, path::Path};

use crate::config::ExampleConfig;
use ::config::Config;
use actix_files::NamedFile;
use actix_web::{
    error, get, middleware,
    web::{self},
    App, Error, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use postgres::NoTls;
use serde::Deserialize;
use tera::Tera;

mod db;
mod errors;
mod sql;

use db::{run_sql, verify_then_run_sql};
use errors::MyError;

#[derive(Debug, Deserialize)]
struct FormInput {
    sql_to_run: String,
}

fn _build_simple_context(
    data_mp: &web::Data<HashMap<&str, Vec<(&str, &str, &str, Vec<&str>)>>>,
    type_as_str: &str,
    num_as_str: &str,
) -> tera::Context {
    let mut ctx = tera::Context::new();
    ctx.insert("prev_q", &get_prev_page(data_mp, type_as_str, num_as_str));
    ctx.insert("next_q", &get_next_page(data_mp, type_as_str, num_as_str));
    ctx.insert("category", &type_as_str);
    ctx
}

async fn build_full_context(
    db_pool: &web::Data<Pool>,
    data_mp: &web::Data<HashMap<&str, Vec<(&str, &str, &str, Vec<&str>)>>>,
    sql_from_user: &str,
    type_as_str: &str,
    num_as_str: &str,
) -> Result<tera::Context, MyError> {
    let client = db_pool.get().await.map_err(|x| MyError::DBPoolError(x))?;

    let mut ctx = _build_simple_context(data_mp, type_as_str, num_as_str);

    let sql_user_result = verify_then_run_sql(&client, sql_from_user.as_ref()).await;

    let data = get_sql_for_q(data_mp, type_as_str, num_as_str);
    match data {
        Some((sql, help_link, title, keywords)) => {
            let keys: String = keywords.iter().map(|s| s.to_string()).collect();
            ctx.insert("sql_correct", sql);
            ctx.insert("heading", title);
            ctx.insert("keyword_help_link", help_link);
            ctx.insert("keyword", &keys);
            let used_correct_word = keywords
                .iter()
                .any(|k| sql_from_user.to_lowercase().contains(k));
            ctx.insert("used_correct_word", &used_correct_word);
            let sql_correct_result = run_sql(&client, sql.as_ref()).await;
            ctx.insert("sql_correct_result", &sql_correct_result);

            let is_correct = if !sql_user_result.is_empty() {
                sql_user_result[1..] == sql_correct_result[1..]
            } else {
                false
            };
            ctx.insert("is_correct", &is_correct);
        }
        None => {
            return Err(MyError::BadPath);
        }
    }
    ctx.insert("sql_to_run", &sql_from_user);
    ctx.insert("sql_to_run_result", &sql_user_result);
    Ok(ctx)
}

async fn question_page_get(
    db_pool: web::Data<Pool>,
    params: web::Path<(String, String)>,
    template: web::Data<tera::Tera>,
    data_mp: web::Data<HashMap<&str, Vec<(&str, &str, &str, Vec<&str>)>>>,
) -> Result<HttpResponse, Error> {
    let sql_from_user = "select \n*\n from cats ";
    let type_as_str = params.0.to_owned();
    let num_as_str = params.1.to_owned();

    let mut ctx =
        build_full_context(&db_pool, &data_mp, sql_from_user, &type_as_str, &num_as_str).await?;
    // Forcing an empty result encourages people to click run the first time to help engagement.
    let empty: Vec<Vec<String>> = vec![vec![]];
    ctx.insert("sql_to_run_result", &empty);

    let path = type_as_str + "/" + &num_as_str + ".html.tera";
    _render_template(path, ctx, template)
}

async fn question_page_post(
    db_pool: web::Data<Pool>,
    form: web::Form<FormInput>,
    params: web::Path<(String, String)>,
    template: web::Data<tera::Tera>,
    data_mp: web::Data<HashMap<&str, Vec<(&str, &str, &str, Vec<&str>)>>>,
) -> Result<HttpResponse, Error> {
    let sql_from_user = &form.sql_to_run;
    let type_as_str = params.0.to_owned();
    let num_as_str = params.1.to_owned();

    println!("SQL: {}/{}:\n{}", type_as_str, num_as_str, sql_from_user);
    let ctx =
        build_full_context(&db_pool, &data_mp, sql_from_user, &type_as_str, &num_as_str).await?;

    let path = type_as_str + "/" + &num_as_str + ".html.tera";
    _render_template(path, ctx, template)
}

async fn get_intro_page(
    _type: web::Path<(String,)>,
    template: web::Data<tera::Tera>,
    data_mp: web::Data<HashMap<&str, Vec<(&str, &str, &str, Vec<&str>)>>>,
) -> Result<HttpResponse, Error> {
    let type_as_str = _type.0.to_owned();
    let ctx = _build_simple_context(&data_mp, &type_as_str, "");
    let path = type_as_str + "/index.html.tera";
    _render_template(path, ctx, template)
}

fn _render_template(
    path: String,
    ctx: tera::Context,
    template: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    // TODO: if template invalid return a 301 ?

    let built_template = template
        .render(&path, &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error {path} {e}")))?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(built_template))
}

#[get("/favicon.ico")]
async fn get_favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).ok()
}

#[get("/robots.txt")]
async fn get_robots() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/robots.txt")).ok()
}

#[get("/static/{file}")]
async fn static_files(filename: web::Path<(String,)>) -> Option<NamedFile> {
    let p: String = "static/".to_string() + &filename.into_inner().0;
    NamedFile::open(Path::new(&p)).ok()
}

async fn home(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("home.html.tera", &tera::Context::new())
        .map_err(|e| error::ErrorInternalServerError(format!("Template error Home {e}")))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn about(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("about.html.tera", &tera::Context::new())
        .map_err(|e| error::ErrorInternalServerError(format!("Template error About {e}")))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    std::env::set_var("RUST_LOG", "actix_web=warn");
    env_logger::init();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    HttpServer::new(move || {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**");
        let tera = Tera::new(path).unwrap();
        let mp = get_question_data_map();

        App::new()
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(mp))
            .wrap(middleware::Logger::default()) // enable logger
            .service(get_favicon)
            .service(get_robots)
            .service(static_files)
            .service(web::resource("/").route(web::get().to(home)))
            .service(web::resource("/about").route(web::get().to(about)))
            .service(
                web::resource("/questions/{cat}/{num}")
                    .route(web::get().to(question_page_get))
                    .route(web::post().to(question_page_post)),
            )
            .service(web::resource("/questions/{cat}/").route(web::get().to(get_intro_page)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
