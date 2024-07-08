use sqlx::{Error, PgPool};

use crate::models::store_model::Store;

extern crate sqlx;

pub async fn get_all_stores_in_db(pool: &PgPool) -> Result<Vec<Store>, Error> {
    let rows = sqlx::query_as!(
        Store,
        r#"SELECT
            id, 
            name, 
            description, 
            location_addr, 
            location_state, 
            location_city, 
            location_postal_code, 
            location_neighborhood, 
            location_lat::float4 as "location_lat!",
            location_lng::float4 as "location_lng!", 
            images
        FROM stores"#
    ).fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn create_station_in_db(pool: &PgPool, store: &Store) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO stores (
            id, name, description, location_addr, location_state, location_city, location_postal_code, location_neighborhood, location_lat, location_lng, images
        ) VALUES (
         $1, $2, $3, $4, $5, $6, $7, $8, $9::real, $10::real, $11
        );
        "#,
        store.id,
        store.name,
        store.description,
        store.location_addr,
        store.location_state,
        store.location_city,
        store.location_postal_code,
        store.location_neighborhood,
        store.location_lat,
        store.location_lng,
        store.images,
    ).execute(pool)
    .await?;
    
    Ok(())
}

pub async fn delete_station_in_db_by_id(pool: &PgPool, store_id: &str) -> Result<(), Error> {
    sqlx::query!(
        r#"DELETE FROM stores WHERE id = $1"#,
        store_id
    ).execute(pool)
    .await?;
    Ok(())
}

pub async fn update_station_in_db(pool: &PgPool, station_id: &str, new_store: &Store) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE stores
        SET name = $1, description = $2, location_addr = $3, location_state = $4, location_city = $5, location_postal_code = $6, location_neighborhood = $7,
        location_lat = $8::real, location_lng = $9::real, images = $10
        WHERE id = $11
        "#,
        new_store.name,
        new_store.description,
        new_store.location_addr,
        new_store.location_state,
        new_store.location_city,
        new_store.location_postal_code,
        new_store.location_neighborhood,
        new_store.location_lat,
        new_store.location_lng,
        new_store.images,
        station_id
    ).execute(pool)
    .await?;
    Ok(())
}

pub async fn get_store_in_db_by_id(pool: &PgPool, store_id: &str) -> Result<Option<Store>, Error> {
    let station = sqlx::query_as!(
        Store,
        r#"
        SELECT 
            id, 
            name, 
            description, 
            location_addr, 
            location_state, 
            location_city, 
            location_postal_code, 
            location_neighborhood, 
            location_lat::float4 as "location_lat!",
            location_lng::float4 as "location_lng!", 
            images
        FROM stores
        WHERE id = $1
        "#,
        store_id
    ).fetch_optional(pool)
    .await?;
    
    Ok(station)
}
