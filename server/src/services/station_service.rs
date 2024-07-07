

extern crate sqlx;
use sqlx::{Error, PgPool};

use crate::models::station_model::Station;

pub async fn get_all_stations_in_db(pool: &PgPool) -> Result<Vec<Station>, Error> {
    let rows = sqlx::query_as!(
        Station,
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
            images,
            fuels
        FROM stations
        "#
    ).fetch_all(pool)
    .await?;
    
    Ok(rows)
}

pub async fn create_station_in_db(pool: &PgPool, station: &Station) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO stations (
            id, name, description, location_addr, location_state, location_city, location_postal_code, location_neighborhood, location_lat, location_lng, images, fuels
        ) VALUES (
         $1, $2, $3, $4, $5, $6, $7, $8, $9::real, $10::real, $11, $12
        );
        "#,
        station.id,
        station.name,
        station.description,
        station.location_addr,
        station.location_state,
        station.location_city,
        station.location_postal_code,
        station.location_neighborhood,
        station.location_lat,
        station.location_lng,
        station.images,
        station.fuels,
    ).execute(pool)
    .await?;
    
    Ok(())
}

pub async fn delete_station_in_db_by_id(pool: &PgPool, station_id: &str) -> Result<(), Error> {
    sqlx::query!(
        r#"DELETE FROM stations WHERE id = $1"#,
        station_id
    ).execute(pool)
    .await?;
    Ok(())
}

pub async fn update_station_in_db(pool: &PgPool, station_id: &str, new_station: &Station) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE stations
        SET name = $1, description = $2, location_addr = $3, location_state = $4, location_city = $5, location_postal_code = $6, location_neighborhood = $7,
        location_lat = $8::real, location_lng = $9::real, images = $10, fuels = $11 
        WHERE id = $12
        "#,
        new_station.name,
        new_station.description,
        new_station.location_addr,
        new_station.location_state,
        new_station.location_city,
        new_station.location_postal_code,
        new_station.location_neighborhood,
        new_station.location_lat,
        new_station.location_lng,
        new_station.images,
        new_station.fuels,
        station_id
    ).execute(pool)
    .await?;
    Ok(())
}

pub async fn get_station_in_db_by_id(pool: &PgPool, station_id: &str) -> Result<Option<Station>, Error> {
    let station = sqlx::query_as!(
        Station,
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
            images,
            fuels
        FROM stations
        WHERE id = $1
        "#,
        station_id
    ).fetch_optional(pool)
    .await?;
    
    Ok(station)
}
