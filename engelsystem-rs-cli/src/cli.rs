use clap::{Args, Parser, Subcommand};
use engelsystem_rs_db::role::RoleType;

#[derive(Debug, Parser)]
#[command(name = "engelcli")]
#[command(about = "Interface for administratively managing the engelsystem and its data")]
pub enum EngelCli {
    #[command(subcommand)]
    Users(UsersCmd),

    #[command(subcommand)]
    Debug(DebugCmd)
}

#[derive(Debug, Subcommand)]
#[command(about = "User related management commands")]
pub enum UsersCmd {
    #[command(about = "List all users")]
    List,

    Role(RoleCmd)
}

#[derive(Debug, Args)]
#[command(about = "User role related management commands")]
pub struct RoleCmd {
    pub user: String,
    
    #[clap(subcommand)]
    pub action: Option<RoleAction>,
}

#[derive(Debug, Subcommand)]
pub enum RoleAction {
    #[command(about = "Set <USER>s role to <ROLE>")]
    Set {
        #[arg(help = "The new role of <USER>. This is dynamic, but default roles are 'Guest', 'User' and 'Admin'")]
        role: RoleType,
    }
}


#[derive(Debug, Subcommand)]
#[command(about = "Debugging related commands")]
pub enum DebugCmd {
    #[command(about = "Create dummy users in the database")]
    CreateDummyUsers {
        amount: u32,
    }
}
