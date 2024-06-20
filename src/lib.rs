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
use desktop::Easytier;
#[cfg(mobile)]
use mobile::Easytier;

#[derive(Default)]
struct MyState(Mutex<HashMap<String, String>>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the easytier APIs.
pub trait EasytierExt<R: Runtime> {
  fn easytier(&self) -> &Easytier<R>;
}

impl<R: Runtime, T: Manager<R>> crate::EasytierExt<R> for T {
  fn easytier(&self) -> &Easytier<R> {
    self.state::<Easytier<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("easytier")
    .invoke_handler(tauri::generate_handler![commands::execute])
    .setup(|app, api| {
      #[cfg(mobile)]
      let easytier = mobile::init(app, api)?;
      #[cfg(desktop)]
      let easytier = desktop::init(app, api)?;
      app.manage(easytier);

      // manage state so it is accessible by the commands
      app.manage(MyState::default());
      Ok(())
    })
    .build()
}
