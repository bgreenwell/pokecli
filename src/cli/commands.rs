use crate::{
    api::PokeApiClient,
    cli::{Commands, OutputFormat},
    output::{Formatter, JsonFormatter, TableFormatter, YamlFormatter},
    Result,
};

pub async fn handle_command(
    command: Commands,
    output_format: OutputFormat,
    _no_cache: bool,
    verbose: bool,
) -> Result<()> {
    let client = PokeApiClient::new()?;
    
    let formatter: Box<dyn Formatter> = match output_format {
        OutputFormat::Table => Box::new(TableFormatter::new()),
        OutputFormat::Json => Box::new(JsonFormatter::new()),
        OutputFormat::Yaml => Box::new(YamlFormatter::new()),
    };

    match command {
        Commands::Pokemon { name_or_id, detailed } => {
            handle_pokemon_command(&client, &*formatter, &name_or_id, detailed, verbose).await
        }
        Commands::Move { name_or_id } => {
            handle_move_command(&client, &*formatter, &name_or_id, verbose).await
        }
        Commands::Item { name_or_id } => {
            handle_item_command(&client, &*formatter, &name_or_id, verbose).await
        }
        Commands::ClearCache => {
            handle_clear_cache_command(verbose).await
        }
    }
}

async fn handle_pokemon_command(
    client: &PokeApiClient,
    formatter: &dyn Formatter,
    name_or_id: &str,
    _detailed: bool,
    verbose: bool,
) -> Result<()> {
    if verbose {
        eprintln!("Fetching Pokemon data for: {name_or_id}");
    }
    
    let pokemon = client.get_pokemon(name_or_id).await?;
    let output = formatter.format_pokemon(&pokemon)?;
    println!("{output}");
    Ok(())
}

async fn handle_move_command(
    _client: &PokeApiClient,
    _formatter: &dyn Formatter,
    _name_or_id: &str,
    _verbose: bool,
) -> Result<()> {
    println!("Move command not yet implemented");
    Ok(())
}

async fn handle_item_command(
    _client: &PokeApiClient,
    _formatter: &dyn Formatter,
    _name_or_id: &str,
    _verbose: bool,
) -> Result<()> {
    println!("Item command not yet implemented");
    Ok(())
}

async fn handle_clear_cache_command(_verbose: bool) -> Result<()> {
    println!("Clear cache command not yet implemented");
    Ok(())
}