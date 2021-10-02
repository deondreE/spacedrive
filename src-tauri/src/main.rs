#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod app;
mod commands;
mod crypto;
mod db;
mod filesystem;
mod util;
use crate::app::menu;
use env_logger;
use futures::executor::block_on;
use log;

fn main() {
  // env_logger::builder()
  //   .filter_level(log::LevelFilter::Debug)
  //   .is_test(true)
  //   .init();

  // create primary data base if not exists
  block_on(db::connection::create_primary_db()).unwrap();
  // init filesystem and create library if missing
  block_on(filesystem::init::init_library()).unwrap();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      commands::read_file_command,
      // commands::generate_buffer_checksum
    ])
    .menu(menu::get_menu())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
