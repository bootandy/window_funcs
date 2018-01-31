#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

use rocket_contrib::Template;
use rocket::request::{Form};
use rocket::response::NamedFile;


mod db;

//forms
#[derive(Debug, FromForm )]
struct FormInput {
    #[form(field = "sql_to_run")]
    sql_to_run: String,
}

static Q1_SQL :&'static str = "select name, sum(weight) 
over (order by name) as running_total_weight 
from cats order by name";

static Q2_SQL :&'static str = "
select name, breed, 
sum(weight) over (partition by breed order by name) from cats ";

fn _get_sql_for_q(s: &String) -> &str {
    match s.as_ref() {
        "q1" => Q1_SQL,
        "q2" => Q2_SQL,
        _ => "select 1 from cats"
    }
}


#[derive(Serialize)]
struct TemplateContext {
    sql_to_run : String, 
    sql_result: Vec<Vec<String>>,
    sql_answer: Vec<Vec<String>>,
}

fn _context_builder(correct_result: Vec<Vec<String>>, sql_result: Vec<Vec<String>>, sql_to_run: String) -> TemplateContext {
    TemplateContext {
        sql_answer: correct_result,
        sql_result: sql_result,
        sql_to_run: sql_to_run
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
                            let temp: i64 = row.get(i);
                            temp.to_string()
                        },
                        "int4" => {
                            let temp: i32 = row.get(i);
                            temp.to_string()
                        },
                        "float8" => {
                            let temp: f64 = row.get(i);
                            format!("{:.1}", temp)
                        },
                        "varchar" => {
                            row.get(i)
                        },
                        x => format!("Add conversion for {:?}", x)
                    }
                }).collect();
                result.push(result_row);
            }
        },
        Err(error) => {
            result.push(vec![error.to_string()])
        }
    }
    result
}

#[post("/<question>", data = "<sink>")]
fn post_db(question: String, conn: db::DbConn, sink: Result<Form<FormInput>, Option<String>>) -> Template {
    let correct_result = _run_sql(&conn, _get_sql_for_q(&question));
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
    Template::render(question.clone(), &_context_builder(correct_result, result, sql_command))
}


#[get("/<question>")]
fn get_db(question : String, conn: db::DbConn) ->  Template {
    let correct_result = _run_sql(&conn, _get_sql_for_q(&question));
    Template::render(question.clone(), &_context_builder(correct_result, vec![], "select * from cats ".to_string()))
}


#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
        rocket::ignite()
            .manage(db::init_pool())
            .mount("/", routes![static_files, post_db, get_db ])
            .attach(Template::fairing())
}

fn main() {
        rocket().launch();
}


