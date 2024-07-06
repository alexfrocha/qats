extern crate sqlx;

use sqlx::Error;

use sqlx::PgPool;

use crate::models::user_model::User;


pub async fn get_all_users_in_db(pool: &PgPool) -> Result<Vec<User>, Error> {
    let rows = sqlx::query_as!(
        User,
        r#"
            SELECT 
                id, 
                name, 
                email, 
                password, 
                date_of_birth, 
                cpf, 
                location_lat::float4 as "location_lat!", 
                location_lng::float4 as "location_lng!", 
                uniques_store, 
                uniques_station, 
                uniques_can_change, 
                role, 
                phone_number
            FROM users
        "#
    )
    .fetch_all(pool)
    .await?;


    Ok(rows)
}



pub async fn get_user_in_db_by_id(pool: &PgPool, user_id: String) -> Result<Option<User>, Error> {
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT 
                id, 
                name, 
                email, 
                password, 
                date_of_birth, 
                cpf, 
                location_lat::float4 as "location_lat!", 
                location_lng::float4 as "location_lng!", 
                uniques_store, 
                uniques_station, 
                uniques_can_change, 
                role, 
                phone_number
            FROM users
            WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_in_db_by_email(pool: &PgPool, email: String) -> Result<Option<User>, Error> {
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT 
                id, 
                name, 
                email, 
                password, 
                date_of_birth, 
                cpf, 
                location_lat::float4 as "location_lat!", 
                location_lng::float4 as "location_lng!", 
                uniques_store, 
                uniques_station, 
                uniques_can_change, 
                role, 
                phone_number
            FROM users
            WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn update_user_in_db(pool: &PgPool, user_id: &str, new_data: &User) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET name = $1, email = $2, password = $3, date_of_birth = $4,
            cpf = $5, location_lat = $6::real, location_lng = $7::real,
            uniques_store = $8, uniques_station = $9, uniques_can_change = $10,
            role = $11, phone_number = $12
        WHERE id = $13
        "#,
        new_data.name,
        new_data.email,
        new_data.password,
        new_data.date_of_birth,
        new_data.cpf,
        new_data.location_lat,
        new_data.location_lng,
        new_data.uniques_store,
        new_data.uniques_station,
        new_data.uniques_can_change,
        new_data.role,
        new_data.phone_number,
        user_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_user_in_db_by_id(pool: &PgPool, user_id: &str) -> Result<(), Error> {
    sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        user_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_user_in_db_by_cpf(pool: &PgPool, cpf: String) -> Result<Option<User>, Error> {
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT 
                id, 
                name, 
                email, 
                password, 
                date_of_birth, 
                cpf, 
                location_lat::float4 as "location_lat!", 
                location_lng::float4 as "location_lng!", 
                uniques_store, 
                uniques_station, 
                uniques_can_change, 
                role, 
                phone_number
            FROM users
            WHERE cpf = $1
        "#,
        cpf
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn create_user_in_db(pool: &PgPool, user: &User) -> Result<(), Error> {

    sqlx::query!(
        r#"
        INSERT INTO users (
            id, name, email, password, date_of_birth, cpf,
            location_lat, location_lng, uniques_store, uniques_station,
            uniques_can_change, role, phone_number
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7::real, $8::real, $9, $10, $11, $12, $13
        )
        "#,
        user.id,
        user.name,
        user.email,
        user.password,
        user.date_of_birth,
        user.cpf,
        user.location_lat,
        user.location_lng,
        user.uniques_store,
        user.uniques_station,
        user.uniques_can_change,
        user.role,
        user.phone_number
    )
    .execute(pool)
    .await?;

    Ok(())
}
