use ::r2d2::PooledConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PooledPgConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub fn create_pool(
    user: &str,
    password: &str,
    host: &str,
    port: u16,
    name: &str,
    pool: u8,
) -> PgPool {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, name
    );
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(pool.into()) // Convert pool size to u32
        .build(manager)
        .expect("Failed to create connection pool")
}
