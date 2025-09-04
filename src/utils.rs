use crate::Result;

pub fn normalize_pokemon_name(name: &str) -> String {
    name.to_lowercase().trim().to_string()
}

pub fn is_numeric_id(input: &str) -> bool {
    input.chars().all(|c| c.is_ascii_digit())
}

pub fn validate_pokemon_input(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(crate::PokeCliError::InvalidInput {
            input: input.to_string(),
        });
    }
    Ok(normalize_pokemon_name(trimmed))
}