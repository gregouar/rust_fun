pub use game_app::GameApp;
pub use game_state_trait::{GameState, StateChangeAction};
pub use game_states_manager::GameStatesManager;

pub mod config;
mod game_app;
mod game_state_trait;
mod game_states_manager;
