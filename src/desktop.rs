use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<IosNoBounce<R>> {
    Ok(IosNoBounce(app.clone()))
}

/// Access to the ios-no-bounce APIs.
pub struct IosNoBounce<R: Runtime>(AppHandle<R>);

impl<R: Runtime> IosNoBounce<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }
}
