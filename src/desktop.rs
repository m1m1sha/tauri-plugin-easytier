use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Easytier<R>> {
    Ok(Easytier(app.clone()))
}

/// Access to the easytier APIs.
pub struct Easytier<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Easytier<R> {}
