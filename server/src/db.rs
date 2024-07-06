use sqlx::pool::PoolOptions;
use sqlx::{ Error, PgPool };

pub async fn set_database() -> Result<PgPool, Error> {
    let pool = PoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/qats").await?;

    // let mut client = Client::connect("postgres://postgres:postgres@db:5432/qats", NoTls)?;
    // let (mut client, connection) = tokio_postgres::connect("", NoTls).await?;

    // client.batch_execute(
    "";
    // ).await?;

    sqlx
        ::query(
            r#"
                CREATE TABLE IF NOT EXISTS users (
                    id VARCHAR(36) NOT NULL,
                    name VARCHAR(255) NOT NULL,
                    email VARCHAR(255) NOT NULL,
                    password VARCHAR(255) NOT NULL,
                    date_of_birth VARCHAR(255) NOT NULL,
                    cpf VARCHAR(255) NOT NULL,
                    location_lat FLOAT NOT NULL,
                    location_lng FLOAT NOT NULL,
                    uniques_store VARCHAR(255),
                    uniques_station VARCHAR(255),
                    uniques_can_change BOOLEAN NOT NULL,
                    role VARCHAR(255) NOT NULL,
                    phone_number VARCHAR(255),
                    PRIMARY KEY (id)
                );
                "#
        )
        .execute(&pool).await?;

    sqlx
        ::query(
            r#"
                CREATE TABLE IF NOT EXISTS stores (
                    id VARCHAR(36) NOT NULL,
                    name VARCHAR(255) NOT NULL,
                    description TEXT,
                    location_addr VARCHAR(255) NOT NULL,
                    location_neighborhood VARCHAR(255) NOT NULL,
                    location_postal_code VARCHAR(255) NOT NULL,
                    location_city VARCHAR(255) NOT NULL,
                    location_state VARCHAR(255) NOT NULL,
                    location_lat FLOAT NOT NULL,
                    location_lng FLOAT NOT NULL,
                    PRIMARY KEY (id)
                );
                "#
        )
        .execute(&pool).await?;

    sqlx
        ::query(
            r#"
                CREATE TABLE IF NOT EXISTS stations (
                    id VARCHAR(36) NOT NULL,
                    name VARCHAR(255) NOT NULL,
                    description TEXT,
                    location_addr VARCHAR(255) NOT NULL,
                    location_neighborhood VARCHAR(255) NOT NULL,
                    location_postal_code VARCHAR(255) NOT NULL,
                    location_city VARCHAR(255) NOT NULL,
                    location_state VARCHAR(255) NOT NULL,
                    location_lat FLOAT NOT NULL,
                    location_lng FLOAT NOT NULL,
                    PRIMARY KEY (id)
                );
                "#
        )
        .execute(&pool).await?;

    sqlx
        ::query(
            r#"
                CREATE TABLE IF NOT EXISTS sales (
                    id VARCHAR(36) NOT NULL,
                    created_at TIMESTAMP NOT NULL,
                    seller_id VARCHAR(36),
                    buyer_id VARCHAR(36) NOT NULL,
                    status VARCHAR(255) NOT NULL,
                    info_currency VARCHAR(255) NOT NULL,
                    info_place VARCHAR(255) NOT NULL,
                    info_amount VARCHAR(255),
                    info_price VARCHAR(255),
                    PRIMARY KEY (id)
                );
                "#
        )
        .execute(&pool).await?;

    Ok(pool)
}

pub async fn pool() -> PgPool {
    let pool = PoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/qats").await
        .unwrap();

    pool
}
