use futures::executor::block_on;
use std::error::Error;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties,
};

fn get_current_session(
    session_manager: &GlobalSystemMediaTransportControlsSessionManager,
) -> Result<GlobalSystemMediaTransportControlsSession, Box<dyn Error>> {
    let session = session_manager.GetCurrentSession()?;
    Ok(session)
}

fn get_session_info(
    session: &GlobalSystemMediaTransportControlsSession,
) -> Result<GlobalSystemMediaTransportControlsSessionMediaProperties, Box<dyn Error>> {
    let result = session.TryGetMediaPropertiesAsync()?;
    let session_info = block_on(result)?;
    Ok(session_info)
}

#[tauri::command]
pub fn get_current_song() -> String {
    let async_operation = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap();
    let session_manager: GlobalSystemMediaTransportControlsSessionManager =
        block_on(async_operation).unwrap();

    let current_session = get_current_session(&session_manager).unwrap();
    let session_info = get_session_info(&current_session).unwrap();

    let title = session_info.Title().unwrap();
    title.to_string().into()
}
