use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum MyError {
    #[display(fmt = "This combination of question type & number is not recognised")]
    BadPath,
    DBPoolError(PoolError),
}

// Actix Web uses `ResponseError` for conversion of errors to a response
impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::BadPath => HttpResponse::BadRequest().finish(),
            MyError::DBPoolError(err) => {
                println!("Failed to get connection from db");
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }
}
