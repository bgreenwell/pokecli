use crate::{
    api::{endpoints::Endpoints, models::*},
    cache::{Cache, MemoryCache},
    utils::validate_pokemon_input,
    PokeCliError, Result,
};
use reqwest::Client;
use std::time::Duration;

pub struct PokeApiClient {
    client: Client,
    cache: Option<MemoryCache>,
}

impl PokeApiClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("pokecli/0.1.0")
            .build()?;
            
        Ok(Self {
            client,
            cache: None,
        })
    }
    
    pub fn with_cache(mut self, cache: MemoryCache) -> Self {
        self.cache = Some(cache);
        self
    }
    
    pub async fn get_pokemon(&self, name_or_id: &str) -> Result<Pokemon> {
        let input = validate_pokemon_input(name_or_id)?;
        let url = Endpoints::pokemon(&input);
        
        // Try cache first if available
        if let Some(cache) = &self.cache {
            let cache_key = format!("pokemon:{input}");
            if let Ok(Some(cached)) = cache.get::<Pokemon>(&cache_key) {
                return Ok(cached);
            }
        }
        
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let pokemon: Pokemon = response.json().await?;
            
            // Cache the result if cache is available
            if let Some(cache) = &self.cache {
                let cache_key = format!("pokemon:{input}");
                let _ = cache.set(&cache_key, &pokemon, Duration::from_secs(3600));
            }
            
            Ok(pokemon)
        } else if response.status() == 404 {
            Err(PokeCliError::PokemonNotFound {
                name: name_or_id.to_string(),
            })
        } else {
            Err(PokeCliError::Http(response.error_for_status().unwrap_err()))
        }
    }
    
    pub async fn get_move(&self, name_or_id: &str) -> Result<Move> {
        let url = Endpoints::move_(name_or_id);
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(PokeCliError::Http(response.error_for_status().unwrap_err()))
        }
    }
    
    pub async fn get_item(&self, name_or_id: &str) -> Result<Item> {
        let url = Endpoints::item(name_or_id);
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(PokeCliError::Http(response.error_for_status().unwrap_err()))
        }
    }
}