use crate::handler::user;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "user-cli")]
#[command(about = "User management CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Create a new user")]
    Create {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        department: Option<String>,
    },
    #[command(about = "Fetch user by ID")]
    Fetch {
        #[arg(short, long)]
        id: i64,
    },
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name, department } => {
            let repo = super::repository::UserAggregateRepository::new(super::repository::POOL.clone());
            let command = user::commands::CreateUser {
                user_name: name,
                department_name: department,
            };
            let user_id = user::create_user(repo, command).await?;
            println!("Created user with ID: {}", user_id);
        }
        Commands::Fetch { id } => {
            let repo = super::repository::UserAggregateRepository::new(super::repository::POOL.clone());
            let query = user::queries::FetchUserByIdQuery { user_id: id };
            let users = user::fetch_user_by_id(repo, query).await?;

            if users.is_empty() {
                println!("User not found");
            } else {
                for user in users {
                    println!("{}", serde_json::to_string_pretty(&user)?);
                }
            }
        }
    }

    Ok(())
}