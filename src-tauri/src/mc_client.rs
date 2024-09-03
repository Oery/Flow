use windows::{
    Win32::Foundation::{BOOL, HWND, LPARAM},
    Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW, IsWindowVisible},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ClientName {
    LunarClient,
    BadlionClient,
    Minecraft,
    Forge,
    Feather,
    Final,
    NotFound,
}

#[derive(Debug, PartialEq)]
pub struct Client {
    pub name: ClientName,
    pub display_name: &'static str,
    pub path: &'static str,
    window_title: &'static str,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            name: ClientName::Minecraft,
            display_name: "Minecraft",
            path: "%APPDATA%\\.minecraft\\logs\\latest.log",
            window_title: "Minecraft",
        }
    }
}

pub const CLIENT_NOT_FOUND: &Client = &Client {
    name: ClientName::NotFound,
    display_name: "?",
    path: "",
    window_title: "?",
};

const CLIENTS: &[Client] = &[
    Client {
        name: ClientName::LunarClient,
        display_name: "Lunar Client",
        path: "%USERPROFILE%\\.lunarclient\\offline\\multiver\\logs\\latest.log",
        window_title: "Lunar Client 1.",
    },
    Client {
        name: ClientName::BadlionClient,
        display_name: "Badlion Client",
        path: "%APPDATA%\\.minecraft\\logs\\blclient\\minecraft\\latest.log",
        window_title: "Badlion Minecraft Client",
    },
    Client {
        name: ClientName::Minecraft,
        display_name: "Minecraft",
        path: "%APPDATA%\\.minecraft\\logs\\latest.log",
        window_title: "Minecraft 1.8.9",
    },
    Client {
        name: ClientName::Forge,
        display_name: "Forge",
        path: "%APPDATA%\\.minecraft\\logs\\latest.log",
        window_title: "Minecraft 1.8.9",
    },
    Client {
        name: ClientName::Feather,
        display_name: "Feather Client",
        path: "%APPDATA%\\.minecraft\\logs\\latest.log",
        window_title: "Feather Client 1.8.9",
    },
    Client {
        name: ClientName::Final,
        display_name: "Final Client",
        path: "%APPDATA%\\.oerymc\\logs\\latest.log",
        window_title: "Final Client",
    },
];

extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    unsafe {
        if IsWindowVisible(hwnd).as_bool() {
            let mut title = [0u16; 256]; // UTF-16 buffer
            let len = GetWindowTextW(hwnd, &mut title);
            if len > 0 {
                let title = String::from_utf16_lossy(&title[..len as usize]);
                let titles_vec = &mut *(lparam.0 as *mut Vec<String>);
                titles_vec.push(title);
            }
        }
    }
    BOOL::from(true) // Continue enumeration
}

pub fn get_current_client() -> Option<&'static Client> {
    let mut titles: Vec<String> = Vec::new();

    unsafe {
        let _ = EnumWindows(Some(enum_windows_proc), LPARAM(&mut titles as *mut _ as isize));
    }

    return CLIENTS
        .iter()
        .find(|client| titles.iter().any(|title| title.contains(client.window_title)));
}

#[tauri::command]
pub fn get_mc_client() -> Result<String, String> {
    match get_current_client() {
        Some(client) => Ok(client.display_name.to_string()),
        None => Ok("?".to_string()),
    }
}
