use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use log::{info, warn};
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
    time::Instant,
};
use tauri::AppHandle;

use crate::{
    log_process::{process_line, update_packs, update_server},
    states::config::read_settings,
    tailer::{
        parser::{parse_packs, parse_server_address},
        utils::format_path,
    },
};

pub struct LogReader {
    path: String,
    file: Option<File>,
    pub last_position: u64,
    last_edit: Instant,
}

impl LogReader {
    pub fn new() -> Self {
        LogReader {
            path: "".to_string(),
            file: None,
            last_position: 0,
            last_edit: Instant::now(),
        }
    }

    pub fn load_file(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        if file_path.is_empty() {
            return Ok(());
        }

        let formatted_path = format_path(file_path);

        if self.path != formatted_path || self.file.is_none() {
            info!("[TAILER] File path changed, Reloading...");
            self.path = formatted_path.clone();
            self.file = Some(File::open(&formatted_path)?);
            self.last_position = self.file.as_ref().unwrap().metadata()?.len();
        }

        if self.file.is_none() {
            warn!("[TAILER] No file found");
            return Ok(());
        }

        Ok(())
    }

    pub async fn read_init(&mut self, app_handle: &AppHandle, file_path: &str) -> Result<(), Box<dyn Error>> {
        self.load_file(file_path)?;

        let file = self.file.as_ref().unwrap();
        let reader = BufReader::new(DecodeReaderBytesBuilder::new().encoding(Some(WINDOWS_1252)).build(file));

        let settings = read_settings(app_handle).await;

        let mut packs_vec: Vec<String> = Vec::new();
        let mut server_address = String::new();

        for result in reader.lines() {
            let line = result?;

            if line.contains(" [Client thread/INFO]: [OptiFine] Resource packs: ")
                || line.contains(" [Render thread/INFO]: Reloading ResourceManager: ")
            {
                packs_vec = parse_packs(&line);
            }

            if line.contains(" [Client thread/INFO]: Connecting to ")
                || line.contains(" [Render thread/INFO]: Connecting to ")
                || line.contains(" [Client thread/INFO]: Worker done, connecting to ")
            {
                if let Some(address) = parse_server_address(&line) {
                    server_address = address.to_string();
                }
            }
        }

        if !packs_vec.is_empty() {
            info!("[PARSER] Updating Packs");
            update_packs(app_handle, packs_vec, &settings).await;
        }

        if !server_address.is_empty() {
            info!("[PARSER] Updating Server");
            update_server(app_handle, &server_address, &settings).await;
        }

        info!("[TAILER] Finished initialization");

        Ok(())
    }

    pub async fn read<'a, 'b>(&mut self, app_handle: AppHandle, file_path: &str) -> Result<(), Box<dyn Error>> {
        self.load_file(file_path)?;

        let file_ref = self.file.as_ref();

        if file_ref.is_none() {
            return Ok(());
        }

        let mut file = file_ref.unwrap();
        let current_length = file.metadata()?.len();

        if current_length < self.last_position {
            return Ok(());
        }

        file.seek(SeekFrom::Start(self.last_position))?;

        self.last_position = current_length;

        let reader = BufReader::new(DecodeReaderBytesBuilder::new().encoding(Some(WINDOWS_1252)).build(file));

        for result in reader.lines() {
            let line = result?;
            let _ = process_line(line, &app_handle).await;
            self.last_edit = Instant::now();
        }

        if self.last_edit.elapsed().as_secs() > 10 {
            self.load_file(file_path)?;
            self.last_edit = Instant::now();
        }

        Ok(())
    }
}
