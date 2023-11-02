use crate::windows_media::get_current_song;
use crate::Settings;
use std::sync::{Arc, RwLock};
use std::time::Instant;

pub fn get_loop(settings: Arc<RwLock<Settings>>) {
    std::thread::spawn(move || loop {
        let start = Instant::now();
        let settings = {
            let locked_settings = settings.read().unwrap();
            locked_settings.clone()
        };
        // println!("About to lock settings in get_loop...");
        // println!("Released lock in get_loop...");
        if settings.music_enable {
            let title = get_current_song();
            println!("Currently listening to : {}", title);
            let duration = start.elapsed();
            println!("Time elapsed for this iteration is: {:?}", duration);
            println!("Scene Name : {}", settings.scenes_name);
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    });
}
