pub mod error;

pub use error::*;

use engelsystem_rs_db::user;

#[tokio::main]
async fn main() -> Result<()> {
    let db = engelsystem_rs_db::connect_and_migrate().await?;
    user::add_guest("Meow", &db).await?;
    
    Ok(())
}
