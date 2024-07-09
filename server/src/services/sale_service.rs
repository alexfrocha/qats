extern crate sqlx;

use sqlx::PgPool;
use sqlx::Error;

use crate::models::sale_model::Sale;

pub async fn get_all_sales_in_db(pool: &PgPool) -> Result<Vec<Sale>, Error> {
    let rows = sqlx::query_as!(
        Sale,
        r#"SELECT id, created_at, seller_id, buyer_id, status, info_currency, info_place, info_amount, info_price FROM sales"#
    ).fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_sale_in_db_by_id(pool: &PgPool, sale_id: &str) -> Result<Option<Sale>, Error> {
    let sale = sqlx::query_as!(
        Sale,
        r#"SELECT * FROM sales WHERE id = $1"#, sale_id
    ).fetch_optional(pool)
    .await?;

    Ok(sale)
}

pub async fn get_sale_in_db_by_seller_id(pool: &PgPool, seller_id: &str) -> Result<Option<Sale>, Error> {
    let sale = sqlx::query_as!(
        Sale,
        r#"SELECT * FROM sales WHERE seller_id = $1"#, seller_id
    ).fetch_optional(pool)
    .await?;

    Ok(sale)
}

pub async fn get_sale_in_db_by_buyer_id(pool: &PgPool, buyer_id: &str) -> Result<Option<Sale>, Error> {
    let sale = sqlx::query_as!(
        Sale,
        r#"SELECT * FROM sales WHERE buyer_id = $1"#, buyer_id
    ).fetch_optional(pool)
    .await?;

    Ok(sale)
}

pub async fn create_sale_in_db(pool: &PgPool, sale: &Sale) -> Result<(), Error> {
    sqlx::query!(r#"
        INSERT INTO sales (id, created_at, seller_id, buyer_id, status, info_currency, info_place, info_amount, info_price) VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9)
    "#, sale.id, sale.created_at, sale.seller_id, sale.buyer_id, sale.status, sale.info_currency, sale.info_place, sale.info_amount, sale.info_price).execute(pool).await?;
    Ok(())
}

pub async fn update_sale_in_db(pool: &PgPool, sale_id: &str, sale: &Sale) -> Result<(), Error> {
    sqlx::query!(
        r#" UPDATE sales SET created_at = $1, seller_id = $2, buyer_id = $3, status = $4, info_currency = $5, info_place = $6, info_amount = $7, info_price = $8 WHERE id = $9"#,
        sale.created_at, sale.seller_id, sale.buyer_id, sale.status, sale.info_currency, sale.info_place, sale.info_amount, sale.info_price, sale_id
    ).execute(pool).await?;
    Ok(())
}

pub async fn delete_sale_in_db_by_id(pool: &PgPool, sale_id: &str) -> Result<(), Error> {
    sqlx::query!(r#"DELETE FROM sales WHERE id = $1"#, sale_id).execute(pool).await?;
    Ok(())
}