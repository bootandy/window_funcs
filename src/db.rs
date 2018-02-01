
extern crate rocket;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde_json;

/*#[cfg(test)]
mod tests;
*/

use std::ops::Deref;

use rocket::request::{self, FromRequest, Request, State};
use rocket::outcome::Outcome::*;

use rocket::http::{Status};
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::{PooledConnection};


type Pool = r2d2::Pool<PostgresConnectionManager>;

pub fn init_pool() -> Pool {
    let manager = PostgresConnectionManager::new(
        "postgres://rusty@localhost", TlsMode::None
    ).unwrap();
    r2d2::Pool::new(manager).unwrap()
}
pub struct DbConn(pub PooledConnection<PostgresConnectionManager>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Success(DbConn(conn)),
            Err(_) => Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &Connection. So we dont have to do conn.0 
impl Deref for DbConn {
    type Target = PooledConnection<PostgresConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

