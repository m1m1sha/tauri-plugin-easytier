use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use easytier;
pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Easytier;
#[cfg(mobile)]
use mobile::Easytier;

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
        .invoke_handler(tauri::generate_handler![
            commands::parse_network_config,
            commands::stop_network_instance,
            commands::collect_network_infos,
            commands::start_network_instance,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let easytier = mobile::init(app, api)?;

            #[cfg(desktop)]
            let easytier = desktop::init(app, api)?;
            app.manage(easytier);

            Ok(())
        })
        .build()
}
