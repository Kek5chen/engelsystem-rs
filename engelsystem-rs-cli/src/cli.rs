use clap::{Args, Parser, Subcommand};
use engelsystem_rs_db::role::RoleType;

#[derive(Debug, Parser)]
#[command(name = "engelcli")]
#[command(about = "Cli for administratively managing the engelsystem and its data")]
pub enum EngelCli {
    #[command(subcommand)]
    Users(UsersCmd),

    #[command(subcommand)]
    Debug(DebugCmd)
}

#[derive(Debug, Subcommand)]
pub enum UsersCmd {
    List,

    Role(RoleCmd)
}

#[derive(Debug, Args)]
pub struct RoleCmd {
    pub user: String,
    
    #[clap(subcommand)]
    pub action: Option<RoleAction>,
}

#[derive(Debug, Subcommand)]
pub enum RoleAction {
    Set {
        role: RoleType,
    }
}


#[derive(Debug, Subcommand)]
pub enum DebugCmd {
    CreateDummyUsers {
        amount: u32,
    }
}
