use std::env;

use clap::Parser;
use cli::EngelCli;
use engelsystem_rs_db::{
    connect,
    role::RoleType,
    user::{add_guest, get_role_by_username, set_role_by_username},
    DatabaseConnection,
};
use log::{info, warn};
use rand::{distr::Alphanumeric, Rng as _};

mod cli;

#[tokio::main]
async fn main() {
    _ = dotenvy::dotenv();
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let cmd = EngelCli::parse();

    let url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        warn!("No DATABASE_URL set. Using an sqlite file in the current directory.");
        "sqlite://meow.sqlite?mode=rwc".to_string()
    });
    let db = connect(&url).await.unwrap();

    match cmd {
        EngelCli::Users(users_cmd) => {
            use cli::UsersCmd;
            match users_cmd {
                UsersCmd::List => todo!(),
                UsersCmd::Role(role_cmd) => {
                    use cli::RoleAction;

                    match role_cmd.action {
                        None => get_role(&role_cmd.user, &db).await,
                        Some(RoleAction::Set { role }) => set_role(&role_cmd.user, role, &db).await,
                    }
                }
            }
        }
        EngelCli::Debug(debug_cmd) => {
            use cli::DebugCmd;

            match debug_cmd {
                DebugCmd::CreateDummyUsers { amount } => create_dummy_users(amount, &db).await,
            }
        }
    }
}

async fn set_role(username: &str, role: RoleType, db: &DatabaseConnection) {
    set_role_by_username(username, role, db).await.unwrap();
    info!("Role of User {username:?} has been changed to {role:?}");
}

async fn get_role(username: &str, db: &DatabaseConnection) {
    let role = get_role_by_username(username, db).await.unwrap();
    info!("User {username:?} has role {role:?}");
}

async fn create_dummy_users(amount: u32, db: &DatabaseConnection) {
    info!("Creating {amount} random users..");

    for _ in 0..amount {
        let mut email: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        let username: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        let password: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();

        email.push_str("@engelsystem.rs");
        add_guest(&username, &email, &password, db).await.unwrap();
    }
}
