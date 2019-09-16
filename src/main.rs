#![feature(plugin, decl_macro)]
#![feature(proc_macro_hygiene)]
#![feature(nll)]
// #![plugin(rocket_codegen)]

extern crate r2d2;
extern crate r2d2_postgres;
extern crate regex;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate tera;

use std::cmp;
use std::path::{Path, PathBuf};

use rocket_contrib::templates::Template;
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{Form, FromRequest, Outcome, Request};
use rocket::response::NamedFile;
use tera::Context;

mod db;
mod sql;

macro_rules! regex {
     ($e:expr) => (regex::Regex::new($e).unwrap())
}

//forms
#[derive(Debug)]
struct FormInput {
    sql_to_run: String,
}

fn _get_next_and_prev(cat: &str, id: &str) -> (String, String) {
    let i = id.parse::<i32>().unwrap_or(-1);
    let prev = {
        if i > 0 {
            format!("{}/{}", cat, cmp::max(i - 1, 0))
        } else if i == 0 {
            cat.to_string() + "/"
        } else if i == -1 {
            let prev_cat = sql::get_prev(cat);
            if prev_cat == "" {
                "".to_string()
            } else {
                format!(
                    "{}/{:?}",
                    prev_cat.to_string(),
                    sql::get_titles_for(prev_cat).len() - 1
                )
            }
        } else {
            panic!("Impossible number given {}", i);
        }
    };
    let next = {
        let next_id = (i + 1).to_string();
        // Nasty hack: Indicates we are at the end of our questions:
        if cat == "other" && i == 2 {
            "".to_string()
        } else if sql::get_sql_for_q(cat, next_id.as_ref()).is_some() {
            format!("{}/{}", cat, next_id)
        } else {
            format!("{}/", sql::get_next(cat))
        }
    };
    (prev, next)
}

#[derive(Serialize)]
struct TemplateContext {
    keyword: String,
    keyword_help_link: String,
    heading: String,
    sql_correct: String,
    sql_to_run: String,
    sql_correct_result: Vec<Vec<String>>,
    sql_to_run_result: Vec<Vec<String>>,
    next_q: String,
    prev_q: String,
    category: String,
    is_correct: bool,
    used_correct_word: bool,
}

struct TemplateDetails {
    id: String,
    category: String,
    sql: String,
    help_link: String,
    title: String,
    keywords: Vec<String>,
}

#[derive(Serialize)]
struct TemplateContextHeading<'a> {
    titles: Vec<&'a str>,
    next_q: String,
    prev_q: String,
    category: String,
}

impl TemplateDetails {
    fn get_path(&self) -> String {
        self.category.to_string() + "/" + self.id.as_ref()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for TemplateDetails {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let template_category = request.get_param::<String>(0).unwrap_or_else(|_| "".into());
        let template_id = request.get_param::<String>(1).unwrap_or_else(|_| "".into());

        match sql::get_sql_for_q(template_category.as_ref(), template_id.as_ref()) {
            Some((sql, help_link, title, keywords)) => Success(TemplateDetails {
                id: template_id.to_string(),
                category: template_category.to_string(), //idea: move these to str s
                sql: sql.to_string(),
                help_link: help_link.to_string(),
                title: title.to_string(),
                keywords: keywords.into_iter().map(|s| s.to_string()).collect(),
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
    let (prev_q, next_q) = _get_next_and_prev(t.category.as_ref(), t.id.as_ref());
    let is_correct = sql_result[1..] == sql_correct_result[1..];
    let used_correct_word = t.keywords
        .iter()
        .any(|k| sql_to_run.to_lowercase().contains(k));

    TemplateContext {
        keyword: t.keywords[0].to_string(),
        keyword_help_link: t.help_link.to_string(),
        sql_correct: t.sql.to_string(),
        sql_to_run,
        sql_correct_result,
        sql_to_run_result: sql_result,
        heading: t.title.to_string(),
        next_q,
        prev_q,
        category: t.category.to_string(),
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
                        "_varchar" => {
                            let temp: Option<Vec<String>> = row.get(i);
                            match temp {
                                None => "Null".to_string(),
                                Some(x) => x.join(","),
                            }
                        }
                        x => {
                            println!("Got unknown type: {:?}", x);
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

fn _verify_then_run_sql<'a>(s: &'a str, conn: &db::DbConn) -> Vec<Vec<String>> {
    if regex!(r###"[^\w]pg_"###).is_match(s) {
        vec![vec!["Do not use pg_".into()]]
    } else if regex!(r###"[^\w]statement_timeout"###).is_match(s) {
        vec![vec!["Do not use statement_timeout".into()]]
    } else if regex!(r###"[^\w]version[^\w]"###).is_match(s) {
        vec![vec!["Do not use version".into()]]
    } else {
        _run_sql(conn, s)
    }
}

#[post("/questions/<_type>/<_question>", data = "<sink>")]
fn post_db(
    _type: String,
    _question: String,
    template: TemplateDetails,
    conn: db::DbConn,
    sink: Result<Form<FormInput>, Option<String>>,
) -> Template {
    let (sql_command, result) = match sink {
        Ok(form) => {
            let sql_command = form.get().sql_to_run.to_string();
            let result = _verify_then_run_sql(sql_command.as_ref(), &conn);
            (sql_command, result)
        }
        Err(Some(f)) => ("".into(), vec![vec![f.to_string()]]),
        Err(None) => ("".into(), vec![vec!["".to_string()]]),
    };

    // log sql to stdout so we can see how people break it.
    println!("query: {:?}", sql_command.replace("\r\n", " "));
    let c = &_context_builder(&conn, &template, result, sql_command);
    Template::render(template.get_path(), &c)
}

#[get("/questions/<_type>/<_question>")]
fn get_db(
    _type: String,
    _question: String,
    template: TemplateDetails,
    conn: db::DbConn,
) -> Template {
    let sql = "select \n*\n from cats ";
    // Forcing an empty result encourages people to click run the first time to help engagement.
    let result = vec![vec![]];
    let c = _context_builder(&conn, &template, result, sql.to_string());
    Template::render(template.get_path(), &c)
}

#[get("/questions/<category>")]
fn old_question_link(category: String) -> Template {
    let real_cat = sql::check_category(category.as_ref());
    let titles = sql::get_titles_for(real_cat);
    let (prev_q, next_q) = _get_next_and_prev(real_cat, "");
    let context = TemplateContextHeading {
        titles,
        next_q: next_q,
        prev_q: prev_q,
        category: real_cat.to_string(),
    };
    Template::render(real_cat.to_string() + "/index", context)
}

#[get("/favicon.ico")]
fn get_favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).ok()
}

#[get("/robots.txt")]
fn get_robots() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/robots.txt")).ok()
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
                get_robots,
                get_home,
                get_about,
                post_db,
                get_db,
                old_question_link,
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
        _get_next_and_prev("grouping", "1"),
        (String::from("grouping/0"), String::from("grouping/2"))
    );
    assert_eq!(
        _get_next_and_prev("over", "0"),
        (String::from("over/"), String::from("over/1"))
    );
    assert_eq!(
        _get_next_and_prev("intro", "0"),
        (String::from("intro/"), String::from("over/"))
    );
    assert_eq!(
        _get_next_and_prev("over", ""),
        (String::from("intro/0"), String::from("over/0"))
    );
    // check it doesn't crash
    _get_next_and_prev("qa", "");
    _get_next_and_prev("", "3");
}
