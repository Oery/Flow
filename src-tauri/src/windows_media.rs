use std::error::Error;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession as ControlsSession, GlobalSystemMediaTransportControlsSessionManager as SessionManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties as MediaProperties,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus as PlaybackStatus,
};

fn get_current_session(session_manager: &SessionManager) -> Result<ControlsSession, Box<dyn Error>> {
    let session = session_manager.GetCurrentSession()?;
    Ok(session)
}

async fn get_session_info(session: &ControlsSession) -> Result<MediaProperties, Box<dyn Error>> {
    let session_info = session.TryGetMediaPropertiesAsync()?.get()?;
    Ok(session_info)
}

pub struct MusicInfo {
    pub title: String,
    pub artist: String,
}

pub async fn get_current_song() -> Result<MusicInfo, Box<dyn Error>> {
    let session_manager = SessionManager::RequestAsync()?.get()?;

    let current_session = get_current_session(&session_manager)?;
    let session_info = get_session_info(&current_session).await?;

    Ok(MusicInfo {
        title: session_info.Title()?.to_string(),
        artist: session_info.Artist()?.to_string(),
    })
}

pub async fn is_media_paused() -> Result<bool, Box<dyn Error>> {
    let session_manager = SessionManager::RequestAsync()?.get()?;
    let current_session = get_current_session(&session_manager)?;
    let playback_status = current_session.GetPlaybackInfo()?.PlaybackStatus()?;

    Ok(playback_status == PlaybackStatus::Paused)
}
