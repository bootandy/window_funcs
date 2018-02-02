#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate tera;
#[macro_use] extern crate serde_derive;

use std::{cmp};
use std::path::{Path, PathBuf};

use rocket_contrib::Template;
use rocket::request::{Form};
use rocket::response::NamedFile;
use tera::Context;

mod db;
mod sql;

//forms
#[derive(Debug, FromForm )]
struct FormInput {
    #[form(field = "sql_to_run")]
    sql_to_run: String,
}

fn _get_next_and_prev(s: &str) -> (String, String) {
    let i = s[1..].parse::<i32>().unwrap();
    let prev = cmp::max(i - 1, 0);
    let next = cmp::min(i + 1, 10);
    return (format!("q{}", prev), (format!("q{}", next)))
}

#[derive(Serialize)]
struct TemplateContext {
    query_requires: String, 
    sql_correct: String, 
    sql_to_run : String, 
    sql_to_run_result: Vec<Vec<String>>,
    sql_correct_result: Vec<Vec<String>>,
    next_q: String,
    prev_q: String,
    is_correct: bool,
    used_correct_word: bool,
}

fn _context_builder(conn: &db::DbConn, question: &String, sql_result: Vec<Vec<String>>, sql_to_run: String) -> TemplateContext {
    let (sql_correct, keyword) = sql::get_sql_for_q(question);
    let correct_result = _run_sql(conn, sql_correct);
    let (prev, next) = _get_next_and_prev(question);
    let is_correct = sql_result[1..] == correct_result[1..];
    let used_correct_word = sql_to_run.to_lowercase().contains(keyword);
        
    TemplateContext {
        query_requires: keyword.to_string(),
        sql_correct:  sql_correct.to_string(),
        sql_correct_result: correct_result,
        sql_to_run: sql_to_run,
        sql_to_run_result: sql_result,
        next_q: next,
        prev_q: prev,
        is_correct: is_correct,
        used_correct_word: used_correct_word,
    }
}

fn _format_type<T: ToString>(t: Option<T>) -> String {
    match t {
        None => "Null".to_string(),
        Some(x) => x.to_string()
    }
}

fn _run_sql(conn: &db::DbConn, sql_command: &str) -> Vec<Vec<String>> {
    let mut result = vec![];
    let query_result = conn.query(sql_command, &[]);
    match query_result {
        Ok(query_results) => {
            result.push(query_results.columns().into_iter().map(|c| {c.name().to_string()}).collect());

            for row in query_results.into_iter() { 
                let result_row = row.columns().into_iter().enumerate().map(|(i, col)| {
                    match col.type_().name() {
                        "int8" => {
                            let temp: Option<i64> = row.get(i);
                            _format_type(temp)
                        },
                        "int4" => {
                            let temp: Option<i32> = row.get(i);
                            _format_type(temp)
                        },
                        "float8" => {
                            let temp: Option<f64> = row.get(i);
                            match temp {
                                None => "Null".to_string(),
                                Some(x) => format!("{:.1}", x)
                            }
                        },
                        "varchar" | "text" => {
                            let temp: Option<String> = row.get(i);
                            _format_type(temp)
                        },
                        x => {
                            println!("BOOM! {:?}", x);
                            format!("Add conversion for {:?}", x)
                        }
                    }
                }).collect();
                result.push(result_row);
            }
        },
        Err(error) => {
            println!(">>> {:?}", error);
            result.push(vec![error.to_string()])
        }
    }
    result
}

#[post("/questions/<question>", data = "<sink>")]
fn post_db(question: String, conn: db::DbConn, sink: Result<Form<FormInput>, Option<String>>) -> Template {
    let (sql_command, result) = match sink {
        Ok(form) => {
            let sql_command1 = form.get().sql_to_run.to_string();
            let result1 = _run_sql(&conn, sql_command1.as_ref());
            (sql_command1, result1)
        },
        Err(Some(f)) => {
            let sql_command1 = "".to_string();
            let result1 = vec![vec![f.to_string()]];
            (sql_command1, result1)
        },
        Err(None) => {
            ("".to_string(), vec![])
        }
    };
    Template::render(question.clone(), &_context_builder(&conn, &question, result, sql_command))
}


#[get("/questions/<question>")]
fn get_db(question : String, conn: db::DbConn) ->  Template {
    let base_sql = "select \n*\n from cats ";
    let result = _run_sql(&conn, base_sql);
    Template::render(question.clone(), &_context_builder(&conn, &question, result, base_sql.to_string()))
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
            .mount("/", routes![static_files, get_favicon, get_home, get_about, post_db, get_db ])
            .attach(Template::fairing())
}

fn main() {
        rocket().launch();
}


#[test]
fn test_get_next_and_prev() {
    assert_eq!(_get_next_and_prev("q4"), (String::from("q3"), String::from("q5")));
    assert_eq!(_get_next_and_prev("q0"), (String::from("q0"), String::from("q1")));
}

