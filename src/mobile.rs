use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "org.easytier.plugin";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_easytier);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Easytier<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "EasytierPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_easytier)?;
    Ok(Easytier(handle))
}

/// Access to the easytier APIs.
pub struct Easytier<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Easytier<R> {
    #[cfg(target_os = "android")]
    pub fn fd(&self, payload: FdRequest) -> crate::Result<FdResponse> {
        self.0.run_mobile_plugin("fd", payload).map_err(Into::into)
    }
}
