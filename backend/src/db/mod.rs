pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::utils::config::CONFIG;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(CONFIG.database.url.clone());
    let pool = Pool::builder()
        .max_size(CONFIG.database.max_connections)
        .build(manager)
        .expect("Failed to create pool.");

    // Применяем миграции при запуске
    let mut conn = pool.get().expect("Failed to get connection from pool");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    pool
}
