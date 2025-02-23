mod api;

use std::path::PathBuf;
use api::{iron::Iron, todos::Todos};
use clap::{Parser, Subcommand};
use std::io::{self, Write};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New,
    Add { id: String, label: String },
    Update { id: String, label: String },
    Done { id: String },
    Delete { id: String },
    Join { ticket: String },
    Ticket,
    Print,
    Exit,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from("loc");
    let iroh = Iron::new(path).await?;
    let mut todos: Option<Todos> = None;

    loop {
        // Muestra el prompt y lee la entrada del usuario
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Divide la entrada en argumentos
        let args = std::iter::once("app") // Nombre ficticio de la aplicación
            .chain(input.trim().split_whitespace())
            .collect::<Vec<_>>();

        // Analiza los argumentos utilizando clap
        let cli = match Cli::try_parse_from(args) {
            Ok(cli) => cli,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        // Maneja los comandos
        match &cli.command {
            Some(Commands::New) => {
                todos = Some(
                    Todos::new(None, iroh.clone())
                        .await
                        .map_err(|e| e.to_string())?,
                );
            }
            Some(Commands::Add { id, label }) => {
                if let Some(ref mut t) = todos {
                    t.add(id.clone(), label.clone()).await?;
                } else {
                    println!("Por favor, crea una lista de tareas primero usando el comando 'new'.");
                }
            }
            Some(Commands::Update { id, label }) => {
                if let Some(ref mut t) = todos {
                    t.update(id.clone(), label.clone()).await?;
                } else {
                    println!("Por favor, crea una lista de tareas primero usando el comando 'new'.");
                }
            }
            Some(Commands::Done { id }) => {
                if let Some(ref mut t) = todos {
                    t.toggle_done(id.clone()).await?;
                } else {
                    println!("Por favor, crea una lista de tareas primero usando el comando 'new'.");
                }
            }
            Some(Commands::Delete { id }) => {
                if let Some(ref mut t) = todos {
                    t.delete(id.clone()).await?;
                } else {
                    println!("Por favor, crea una lista de tareas primero usando el comando 'new'.");
                }
            }
            Some(Commands::Join { ticket }) => {
                todos = Some(
                    Todos::new(Some(ticket.clone()), iroh.clone())
                        .await
                        .map_err(|e| e.to_string())?,
                );
            }
            Some(Commands::Ticket) => {
                if let Some(ref t) = todos {
                    println!("Tu ticket: {}", t.ticket());
                } else {
                    println!("Por favor, crea una lista de tareas primero usando el comando 'new'.");
                }
            }
            Some(Commands::Print) => {
                if let Some(ref t) = todos {
                    println!("Tus tareas: {:?}", t.get_todos().await);
                } else {
                    println!("Por favor, crea una lista de tareas primero usando el comando 'new'.");
                }
            }
            Some(Commands::Exit) => break,
            None => {
                println!("No se especificó ningún comando. Usa 'help' para ver la información de uso.");
            }
        }
    }

    Ok(())
}
