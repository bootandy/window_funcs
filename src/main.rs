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

use rocket_contrib::Template;
use rocket::request::{Form};

mod db;

//forms
#[derive(Debug, FromForm )]
struct FormInput {
    #[form(field = "sql_to_run")]
    sql_to_run: String,
}

#[derive(Serialize)]
struct TemplateContext {
    sql_to_run : String, 
    sql_result: Vec<Vec<String>>,
}

fn _context_builder(sql_result: Vec<Vec<String>>, sql_to_run: String) -> TemplateContext {
    TemplateContext {
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
                let mut result_row :Vec<String> = vec![];

                for (i, col) in row.columns().into_iter().enumerate() {
                    let res :String = match col.type_().name() {
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
                    };
                    result_row.push(res);
                }
                result.push(result_row);
            }
        },
        Err(error) => {
            result.push(vec![error.to_string()])
        }
    }
    result
}

#[post("/", data = "<sink>")]
fn post_db(conn: db::DbConn, sink: Result<Form<FormInput>, Option<String>>) -> Template {
    match sink {
        Ok(form) => {
            let sql_command = &form.get().sql_to_run;
            let result = _run_sql(&conn, sql_command.as_ref());
            Template::render("q1", &_context_builder(result, sql_command.to_string()))
        },
        Err(Some(f)) => {
            let sql_command = "";
            let result = vec![vec![f.to_string()]];
            Template::render("q1", &_context_builder(result, sql_command.to_string()))
        },
        Err(None) => {
            Template::render("q1", &_context_builder(vec![], format!("total Error ")))
        }
    }
}


#[get("/")]
fn get_db(conn: db::DbConn) ->  Template {
    Template::render("q1", &_context_builder(vec![], "select * from cats ".to_string()))
}

fn rocket() -> rocket::Rocket {
        rocket::ignite()
            .manage(db::init_pool())
            .mount("/", routes![post_db, get_db ])
            .attach(Template::fairing())
}

fn main() {
        rocket().launch();
}


