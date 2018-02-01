#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate tera;
#[macro_use] extern crate serde_derive;

/*#[cfg(test)]
mod tests;
*/
use std::{cmp};
use std::path::{Path, PathBuf};

use rocket_contrib::Template;
use rocket::request::{Form};
use rocket::response::NamedFile;
use tera::Context;

mod db;

//forms
#[derive(Debug, FromForm )]
struct FormInput {
    #[form(field = "sql_to_run")]
    sql_to_run: String,
}

static Q0_SQL :&'static str = "
select
 age, sum(weight) as total_weight
from cats group by age having sum(weight) > 12;";

static Q1_SQL :&'static str = "select name, sum(weight) 
over (order by name) as running_total_weight 
from cats order by name";

static Q2_SQL :&'static str = "
select name, breed, 
sum(weight) over (partition by breed order by name) as running_total_weight
from cats ";

static Q3_SQL :&'static str = "
select ROW_NUMBER() 
over (partition by breed order by name) as num_cats_of_breed, 
name ,breed from cats order by name";

static Q4_SQL :&'static str = "
select 
rank() over (partition by breed order by weight DESC) as ranking,
name, breed, weight
from cats order by ranking, weight DESC";

static Q5_SQL :&'static str = "
select
 name, weight, NTILE(4) over ( order by weight) as weight_quartile
       from  cats 
       ";

static Q6_SQL :&'static str = "
select 
DENSE_RANK() over (order by age DESC) as r, name,age
 from cats order by r";

static Q7_SQL :&'static str = "
select name, weight, 
      lag(weight, 1) over (order by weight) - weight as target_weight
      from cats order by weight";

static Q8_SQL :&'static str = "
    select name, breed, weight,
lag(weight, 1) over (partition by breed order by weight) - weight as target_weight
from cats order by weight ";

static Q9_SQL :&'static str = "
select name, weight, 
       ntile(2) over ntile_window as by_half,
       ntile(3) over ntile_window as thirds,
       ntile(4) over ntile_window as quart
              from cats
              WINDOW ntile_window AS
                       ( ORDER BY weight)
     order by weight";

static Q10_SQL :&'static str = "
select name, color,
first_value(weight) over (partition by color order by weight) as lowest_weight_by_color
from cats ";


fn _get_sql_for_q(s: &String) -> (&str, &str) {
    match s.as_ref() {
        "q0" => (Q0_SQL, "group by"),
        "q1" => (Q1_SQL, "over"),
        "q2" => (Q2_SQL, "partition by"),
        "q3" => (Q3_SQL, "row_number"),
        "q4" => (Q4_SQL, "rank"),
        "q5" => (Q5_SQL, "ntile"),
        "q6" => (Q6_SQL, "dense_rank"),
        "q7" => (Q7_SQL, "lag"),
        "q8" => (Q8_SQL, "lag"),
        "q9" => (Q9_SQL, "window"),
        "q10" => (Q10_SQL, "first_value"),
        _ => ("select 1 from cats", "")
    }
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
    sql_result: Vec<Vec<String>>,
    sql_answer: Vec<Vec<String>>,
    next_q: String,
    prev_q: String,
}

fn _context_builder(conn: &db::DbConn, question: &String, sql_result: Vec<Vec<String>>, sql_to_run: String) -> TemplateContext {
    let (sql_correct, keyword) = _get_sql_for_q(question);
    let correct_result = _run_sql(conn, sql_correct);
    let (prev, next) = _get_next_and_prev(question);
    TemplateContext {
        query_requires: keyword.to_string(),
        sql_correct:  sql_correct.to_string(),
        sql_answer: correct_result,
        sql_result: sql_result,
        sql_to_run: sql_to_run,
        next_q: next,
        prev_q: prev,
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
                        "varchar" => {
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
    let result1 = _run_sql(&conn, base_sql);
    Template::render(question.clone(), &_context_builder(&conn, &question, result1, base_sql.to_string()))
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

