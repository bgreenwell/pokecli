use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub height: u32,
    pub weight: u32,
    pub base_experience: Option<u32>,
    pub types: Vec<PokemonType>,
    pub abilities: Vec<PokemonAbility>,
    pub stats: Vec<PokemonStat>,
    pub sprites: PokemonSprites,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonType {
    pub slot: u8,
    #[serde(rename = "type")]
    pub type_info: NamedApiResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonAbility {
    pub is_hidden: bool,
    pub slot: u8,
    pub ability: NamedApiResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonStat {
    pub base_stat: u32,
    pub effort: u32,
    pub stat: NamedApiResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonSprites {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedApiResource {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    pub id: u32,
    pub name: String,
    pub accuracy: Option<u32>,
    pub power: Option<u32>,
    pub pp: Option<u32>,
    #[serde(rename = "type")]
    pub move_type: NamedApiResource,
    pub damage_class: NamedApiResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub cost: u32,
    pub category: NamedApiResource,
}