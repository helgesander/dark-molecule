pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Применяем миграции при запуске
    let mut conn = pool.get().expect("Failed to get connection from pool");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    pool
}
