use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: String },
    List,
}

#[derive(Serialize, Deserialize)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let filename = "deck.json";
    let mut deck: Deck = fs::read_to_string(filename)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(Deck { cards: vec![] });

    match &cli.command {
        Commands::Add { name } => {
            deck.cards.push(name.clone());
            let json = serde_json::to_string_pretty(&deck).unwrap();
            fs::write(filename, json).unwrap();
            println!("Carta añadida: {}", name);
        }
        Commands::List => {
            println!("Cartas en el mazo:");
            for card in &deck.cards {
                println!("- {}", card);
            }
        }
    }
}