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
