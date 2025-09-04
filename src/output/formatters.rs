use crate::{api::models::Pokemon, Result};
use colored::*;
use serde_json;
use serde_yaml;
use tabled::{settings::Style, Table, Tabled};

pub trait Formatter {
    fn format_pokemon(&self, pokemon: &Pokemon) -> Result<String>;
    fn format_error(&self, error: &str) -> String;
}

pub struct TableFormatter {
    colored: bool,
}

impl Default for TableFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl TableFormatter {
    pub fn new() -> Self {
        Self { colored: true }
    }
    
    pub fn with_colors(mut self, colored: bool) -> Self {
        self.colored = colored;
        self
    }
}

impl Formatter for TableFormatter {
    fn format_pokemon(&self, pokemon: &Pokemon) -> Result<String> {
        let mut output = String::new();
        
        // Header section
        let name = if self.colored {
            pokemon.name.to_uppercase().bright_cyan().to_string()
        } else {
            pokemon.name.to_uppercase()
        };
        
        let id = if self.colored {
            format!("#{}", pokemon.id).yellow().to_string()
        } else {
            format!("#{}", pokemon.id)
        };
        
        let types: Vec<String> = pokemon.types.iter()
            .map(|t| {
                if self.colored {
                    t.type_info.name.to_uppercase().green().to_string()
                } else {
                    t.type_info.name.to_uppercase()
                }
            })
            .collect();
        
        output.push_str(&format!("{} {} - {}\n\n", name, id, types.join("/")));
        
        // Basic info table
        let basic_info = vec![
            BasicInfoRow { attribute: "Height".to_string(), value: format!("{} decimeters", pokemon.height) },
            BasicInfoRow { attribute: "Weight".to_string(), value: format!("{} hectograms", pokemon.weight) },
            BasicInfoRow { attribute: "Base Experience".to_string(), value: pokemon.base_experience.map_or("Unknown".to_string(), |exp| exp.to_string()) },
        ];
        
        let mut basic_table = Table::new(basic_info);
        basic_table.with(Style::rounded());
        output.push_str(&format!("{basic_table}\n\n"));
        
        // Stats table
        let stats_data: Vec<StatRow> = pokemon.stats.iter()
            .map(|s| StatRow {
                stat: s.stat.name.clone(),
                base_value: s.base_stat,
                effort: s.effort,
            })
            .collect();
        
        let mut stats_table = Table::new(stats_data);
        stats_table.with(Style::rounded());
        output.push_str(&format!("Stats:\n{stats_table}\n\n"));
        
        // Abilities table
        let abilities_data: Vec<AbilityRow> = pokemon.abilities.iter()
            .map(|a| AbilityRow {
                ability: a.ability.name.clone(),
                hidden: if a.is_hidden { "Yes" } else { "No" }.to_string(),
                slot: a.slot,
            })
            .collect();
        
        let mut abilities_table = Table::new(abilities_data);
        abilities_table.with(Style::rounded());
        output.push_str(&format!("Abilities:\n{abilities_table}"));
        
        Ok(output)
    }
    
    fn format_error(&self, error: &str) -> String {
        if self.colored {
            format!("{}: {}", "Error".red().bold(), error)
        } else {
            format!("Error: {error}")
        }
    }
}

#[derive(Tabled)]
struct BasicInfoRow {
    attribute: String,
    value: String,
}

#[derive(Tabled)]
struct StatRow {
    stat: String,
    base_value: u32,
    effort: u32,
}

#[derive(Tabled)]
struct AbilityRow {
    ability: String,
    hidden: String,
    slot: u8,
}

pub struct JsonFormatter;

impl Default for JsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Formatter for JsonFormatter {
    fn format_pokemon(&self, pokemon: &Pokemon) -> Result<String> {
        Ok(serde_json::to_string_pretty(pokemon)?)
    }
    
    fn format_error(&self, error: &str) -> String {
        format!(r#"{{"error": "{error}"}}"#)
    }
}

pub struct YamlFormatter;

impl Default for YamlFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl YamlFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Formatter for YamlFormatter {
    fn format_pokemon(&self, pokemon: &Pokemon) -> Result<String> {
        let json_value: serde_json::Value = serde_json::to_value(pokemon)?;
        serde_yaml::to_string(&json_value).map_err(|e| {
            crate::PokeCliError::Cache(format!("YAML serialization error: {e}"))
        })
    }
    
    fn format_error(&self, error: &str) -> String {
        format!("error: {error}")
    }
}