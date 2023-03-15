use actix_web::{error, web};
use crate::db::Pool;

pub async fn get_by_id_if_exists(pool: &Pool, poke_id: String) -> Result<Vec<String>, actix_web::Error> {
    let pool = pool.clone();

    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || {
        let mut statement = conn.prepare(
            &("SELECT name FROM names WHERE poke_id = ?"),
        )?;

        statement
            .query_map([poke_id], |row| {
                Ok(row.get(0)?)
            }).and_then(Iterator::collect)
    })
        .await?
        .map_err(error::ErrorInternalServerError)
}

pub async fn add_cache_entry(pool: &Pool, name: String, poke_id: String) -> Result<(), actix_web::Error> {
    let pool = pool.clone();

    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || {
        conn.execute(
            "INSERT INTO names (name, poke_id) VALUES ($1, $2)", [&name, &poke_id],
        )
    }).await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(())
}