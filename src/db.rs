use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

use lazy_static::lazy_static;

use crate::error_handler::CustomError;

//use diesel_migrations::{EmbeddedMigrations, MigrationHarness};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

// pub fn init() {
//     lazy_static::initialize(&POOL);
//     let mut conn = connection().expect("Failed to get db connection");
//     conn.run_pending_migrations(MIGRATIONS).unwrap();
// }

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {e}")))
}
