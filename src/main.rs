#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate r2d2;
extern crate r2d2_postgres;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate tera;

use std::cmp;
use std::path::{Path, PathBuf};

use rocket_contrib::Template;
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{Form, FromRequest, Outcome, Request};
use rocket::response::NamedFile;
use tera::Context;

mod db;
mod sql;

//forms
#[derive(Debug, FromForm)]
struct FormInput {
    sql_to_run: String,
}

fn _get_next_and_prev(s: &str) -> (String, String) {
    let i = s.parse::<i32>().unwrap_or(0);
    let prev = cmp::max(i - 1, 0);
    let next = cmp::min(i + 1, 10);
    (format!("{}", prev), (format!("{}", next)))
}

#[derive(Serialize)]
struct TemplateContext {
    keyword: String,
    sql_correct: String,
    sql_to_run: String,
    sql_correct_result: Vec<Vec<String>>,
    sql_to_run_result: Vec<Vec<String>>,
    next_q: String,
    prev_q: String,
    is_correct: bool,
    used_correct_word: bool,
}

struct TemplateDetails {
    name: String,
    sql: String,
    keyword: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for TemplateDetails {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let template_name = request.get_param::<String>(0).unwrap_or("".into());

        match sql::get_sql_for_q(template_name.as_ref()) {
            Some((sql, keyword)) => Success(TemplateDetails {
                name: template_name.to_string(),
                sql: sql.to_string(),
                keyword: keyword.to_string(),
            }),
            None => Failure((Status::BadRequest, ())),
        }
    }
}

fn _context_builder(
    conn: &db::DbConn,
    t: &TemplateDetails,
    sql_result: Vec<Vec<String>>,
    sql_to_run: String,
) -> TemplateContext {
    let sql_correct_result = _run_sql(conn, t.sql.as_ref());
    let (prev_q, next_q) = _get_next_and_prev(t.name.as_ref());
    let is_correct = sql_result[1..] == sql_correct_result[1..];
    let key_ref: &str = t.keyword.as_ref();
    let used_correct_word: bool = sql_to_run.to_lowercase().contains(key_ref);

    TemplateContext {
        keyword: t.keyword.to_string(),
        sql_correct: t.sql.to_string(),
        sql_to_run,
        sql_correct_result,
        sql_to_run_result: sql_result,
        next_q,
        prev_q,
        is_correct,
        used_correct_word,
    }
}

fn _format_type<T: ToString>(t: Option<T>) -> String {
    match t {
        None => "Null".to_string(),
        Some(x) => x.to_string(),
    }
}

fn _run_sql(conn: &db::DbConn, sql_command: &str) -> Vec<Vec<String>> {
    let mut result = vec![];
    let query_result = conn.query(sql_command, &[]);
    match query_result {
        Ok(query_results) => {
            result.push(
                query_results
                    .columns()
                    .into_iter()
                    .map(|c| c.name().to_string())
                    .collect(),
            );

            for row in &query_results {
                let result_row = row.columns()
                    .into_iter()
                    .enumerate()
                    .map(|(i, col)| match col.type_().name() {
                        "int8" => {
                            let temp: Option<i64> = row.get(i);
                            _format_type(temp)
                        }
                        "int4" => {
                            let temp: Option<i32> = row.get(i);
                            _format_type(temp)
                        }
                        "float8" => {
                            let temp: Option<f64> = row.get(i);
                            match temp {
                                None => "Null".to_string(),
                                Some(x) => format!("{:.1}", x),
                            }
                        }
                        "varchar" | "text" => {
                            let temp: Option<String> = row.get(i);
                            _format_type(temp)
                        }
                        x => {
                            println!("BOOM! {:?}", x);
                            format!("Add conversion for {:?}", x)
                        }
                    })
                    .collect();
                result.push(result_row);
            }
        }
        Err(error) => {
            println!(">>> {:?}", error);
            result.push(vec![error.to_string()])
        }
    }
    result
}

#[post("/questions/<_question>", data = "<sink>")]
fn post_db(
    _question: String,
    template: TemplateDetails,
    conn: db::DbConn,
    sink: Result<Form<FormInput>, Option<String>>,
) -> Template {
    let (sql_command, result) = match sink {
        Ok(form) => {
            let sql_command1 = form.get().sql_to_run.to_string();
            let result1 = _run_sql(&conn, sql_command1.as_ref());
            (sql_command1, result1)
        }
        Err(Some(f)) => {
            let sql_command1 = "".to_string();
            let result1 = vec![vec![f.to_string()]];
            (sql_command1, result1)
        }
        Err(None) => ("".to_string(), vec![vec!["".to_string()]]),
    };
    let c = &_context_builder(&conn, &template, result, sql_command);
    Template::render(template.name, &c)
}

#[get("/questions/<_question>")]
fn get_db(_question: String, template: TemplateDetails, conn: db::DbConn) -> Template {
    let sql = "select \n*\n from cats ";
    // Forcing an empty result encourages people to click run the first time to help engagement.
    let result = vec![vec![]];
    let c = _context_builder(&conn, &template, result, sql.to_string());
    Template::render(template.name, &c)
}

#[get("/favicon.ico")]
fn get_favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).ok()
}

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/about")]
fn get_about() -> Template {
    Template::render("about", Context::new())
}

#[get("/")]
fn get_home() -> Template {
    Template::render("home", Context::new())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(db::init_pool())
        .mount(
            "/",
            routes![
                static_files,
                get_favicon,
                get_home,
                get_about,
                post_db,
                get_db
            ],
        )
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}

#[test]
fn test_get_next_and_prev() {
    assert_eq!(
        _get_next_and_prev("4"),
        (String::from("3"), String::from("5"))
    );
    assert_eq!(
        _get_next_and_prev("0"),
        (String::from("0"), String::from("1"))
    );
    // check it doesn't crash
    _get_next_and_prev("qa");
    _get_next_and_prev("");
}
