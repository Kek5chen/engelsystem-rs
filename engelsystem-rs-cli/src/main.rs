use std::env;

use clap::Parser;
use cli::EngelCli;
use engelsystem_rs_db::{
    DatabaseConnection, UserView, connect,
    role::RoleType,
    user::{add_guest, get_all_user_views, get_role_by_username, set_role_by_username},
};
use log::{info, warn};
use rand::{Rng as _, distr::Alphanumeric};
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Constraint,
    style::{Style, Stylize},
    widgets::{Block, Row, Table, TableState},
};

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
                UsersCmd::List => list_users_tui(&db).await,
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

        email = format!("dummy-{email}@engelsystem.rs");
        add_guest(&username, &email, &password, db).await.unwrap();
    }
}

async fn list_users_tui(db: &DatabaseConnection) {
    let users = get_all_user_views(db).await.unwrap();
    UserList::new(&users).run();
}

struct UserList {
    state: TableState,
    items: Vec<[String; 5]>,
}

impl UserList {
    pub fn new(users: &[UserView]) -> Self {
        let user_data = users
            .iter()
            .map(|u| {
                [
                    u.id.to_string(),
                    u.created_at.to_string(),
                    u.username.to_string(),
                    u.email.to_string(),
                    u.role.to_string(),
                ]
            })
            .collect();

        UserList {
            state: TableState::default(),
            items: user_data,
        }
    }

    pub fn run(mut self) {
        let mut terminal = ratatui::init();
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();

            if let Event::Key(key) = event::read().unwrap() {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break,

                    KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                    KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                    _ => {}
                }
            }
        }
        ratatui::restore();
    }

    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let users: Vec<Row> = self
            .items
            .iter()
            .map(|r| r.iter().map(|s| s.as_str()).collect())
            .collect();

        let table = Table::new(
            users,
            [
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .header(
            Row::new(vec!["Id", "Created at", "Username", "Email", "Role"])
                .style(Style::new().light_blue()),
        )
        .block(Block::new().title("User List"))
        .highlight_symbol(">>")
        .row_highlight_style(Style::new().reversed());

        frame.render_stateful_widget(table, frame.area(), &mut self.state);
    }
}
