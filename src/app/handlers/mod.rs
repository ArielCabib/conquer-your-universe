mod canvas;
mod menu;
mod modal;
mod storage;

pub use canvas::{click_handler, context_menu_handler};
pub use menu::build_house_menu_handler;
pub use modal::{modal_close_handler, modal_open_handler, restart_game_handler};
pub use storage::{file_change_handler, open_file_dialog_handler, save_game_handler};
