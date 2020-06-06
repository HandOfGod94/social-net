use std::env;
use diesel::r2d2::{ConnectionManager, Pool, CustomizeConnection, Error};
use diesel::{PgConnection, Connection};

#[derive(Debug)]
struct TestTransaction;

impl CustomizeConnection<PgConnection, Error> for TestTransaction {
    fn on_acquire(&self, conn: &mut PgConnection) -> Result<(), Error> {
        conn.begin_test_transaction().unwrap();
        Ok(())
    }
}

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder()
        .connection_customizer(Box::new(TestTransaction))
        .build(manager).expect("Postgres connection pool couldn't be created")
}
