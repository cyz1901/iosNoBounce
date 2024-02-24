use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

use std::{collections::HashMap, sync::Mutex};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::IosNoBounce;
#[cfg(mobile)]
use mobile::IosNoBounce;

#[derive(Default)]
struct MyState(Mutex<HashMap<String, String>>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the ios-no-bounce APIs.
pub trait IosNoBounceExt<R: Runtime> {
  fn ios_no_bounce(&self) -> &IosNoBounce<R>;
}

impl<R: Runtime, T: Manager<R>> crate::IosNoBounceExt<R> for T {
  fn ios_no_bounce(&self) -> &IosNoBounce<R> {
    self.state::<IosNoBounce<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("ios-no-bounce")
    .invoke_handler(tauri::generate_handler![commands::execute])
    .setup(|app, api| {
      #[cfg(mobile)]
      let ios_no_bounce = mobile::init(app, api)?;
      #[cfg(desktop)]
      let ios_no_bounce = desktop::init(app, api)?;
      app.manage(ios_no_bounce);

      // manage state so it is accessible by the commands
      app.manage(MyState::default());
      Ok(())
    })
    .build()
}
