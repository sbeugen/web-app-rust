use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::db::Pool;
use crate::repository::pokemon_repository;

#[derive(Serialize, Deserialize, Debug)]
struct Pokemon {
    name: String,
    id: u32,
}

#[get("/pokemon/{poke_id}")]
pub async fn get_pokemon_by_id(db: web::Data<Pool>, poke_id: web::Path<String>) -> impl Responder {
    let result = pokemon_repository::get_by_id_if_exists(&db, poke_id.to_string()).await;
    let result = result.ok().flatten();

    match result {
        Some(name) => HttpResponse::Ok().body(name),
        None => {
            let pokemon_response = fetch_pokemon_for_id(poke_id.as_str()).await;

            match pokemon_response {
                Ok(pokemon) => {
                    let _store_result = pokemon_repository::add_cache_entry(
                        &db,
                        pokemon.name.clone(),
                        pokemon.id.to_string(),
                    )
                    .await;
                    HttpResponse::Ok().body(pokemon.name)
                }
                Err(_) => HttpResponse::InternalServerError().body("Error"),
            }
        }
    }
}

async fn fetch_pokemon_for_id(poke_id: &str) -> Result<Pokemon, Box<dyn Error>> {
    let pokemon_response = reqwest::get("https://pokeapi.co/api/v2/pokemon/".to_string() + poke_id)
        .await?
        .json::<Pokemon>()
        .await?;

    Ok(pokemon_response)
}
