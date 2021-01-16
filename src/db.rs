extern crate r2d2;
extern crate r2d2_postgres;
extern crate rocket_include_tera;
extern crate serde_json;
extern crate rocket;

use rocket::request::{self, FromRequest, Request, State};
use rocket::outcome::Outcome::*;

use rocket::http::Status;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use r2d2::PooledConnection;

type Pool = r2d2::Pool<PostgresConnectionManager<NoTls>>;

pub fn init_pool() -> Pool {
    let config = "host=localhost user=rusty password=rusty".parse().unwrap();
    let manager = PostgresConnectionManager::new(config, NoTls);
    r2d2::Pool::new(manager).unwrap()
}
pub struct DbConn(pub PooledConnection<PostgresConnectionManager<NoTls>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Success(DbConn(conn)),
            Err(_) => Failure((Status::ServiceUnavailable, ())),
        }
    }
}
