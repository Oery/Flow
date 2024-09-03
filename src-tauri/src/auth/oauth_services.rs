#[derive(PartialEq)]
pub enum Service {
    Twitch,
    Nightbot,
    WizeBot,
}

impl Service {
    pub fn get_code_url(&self) -> Result<&'static str, String> {
        match self {
            Service::Twitch => Ok(SERVICES.twitch.code),
            Service::Nightbot => Ok(SERVICES.nightbot.code),
            _ => Err("Invalid service".to_string()),
        }
    }

    pub fn get_validate_url(&self) -> Result<&'static str, String> {
        match self {
            Service::Twitch => Ok(SERVICES.twitch.validate),
            Service::Nightbot => Ok(SERVICES.nightbot.validate),
            _ => Err("Invalid service".to_string()),
        }
    }

    pub fn get_refresh_url(&self) -> Result<&'static str, String> {
        match self {
            Service::Twitch => Ok(SERVICES.twitch.refresh),
            Service::Nightbot => Ok(SERVICES.nightbot.refresh),
            _ => Err("Invalid service".to_string()),
        }
    }

    pub fn get_header(&self, token: &str) -> Result<String, String> {
        match self {
            Service::Twitch => Ok("OAuth ".to_string() + token),
            Service::Nightbot => Ok("Bearer ".to_string() + token),
            _ => Err("Invalid service".to_string()),
        }
    }

    pub fn get_vault(&self) -> &'static str {
        match self {
            Service::Twitch => SERVICES.twitch.vault,
            Service::Nightbot => SERVICES.nightbot.vault,
            Service::WizeBot => SERVICES.wizebot.vault,
        }
    }
}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Service::Twitch => write!(f, "TWITCH"),
            Service::Nightbot => write!(f, "NIGHTBOT"),
            Service::WizeBot => write!(f, "WIZEBOT"),
        }
    }
}

pub struct ServiceURL {
    pub code: &'static str,
    pub refresh: &'static str,
    pub validate: &'static str,
    pub vault: &'static str,
}

pub struct Services {
    pub twitch: ServiceURL,
    pub nightbot: ServiceURL,
    pub wizebot: ServiceURL,
}

// PROD
pub const SERVICES: Services = Services {
    twitch: ServiceURL {
        code: "https://api.oery.dev/auth/twitch/code",
        refresh: "https://api.oery.dev/auth/twitch/refresh",
        validate: "https://id.twitch.tv/oauth2/validate",
        vault: "TwitchOAuthToken",
    },
    nightbot: ServiceURL {
        code: "https://api.oery.dev/auth/nightbot/code",
        refresh: "https://api.oery.dev/auth/nightbot/refresh",
        validate: "https://api.nightbot.tv/1/channel",
        vault: "NightbotOAuthToken",
    },
    wizebot: ServiceURL {
        code: "",
        refresh: "",
        validate: "",
        vault: "WizeBotOAuthToken",
    },
};

// DEV
// pub const SERVICES: Services = Services {
//     twitch: ServiceURL {
//         code: "http://localhost:8080/auth/twitch/code",
//         refresh: "http://localhost:8080/auth/twitch/refresh",
//         validate: "https://id.twitch.tv/oauth2/validate",
//         vault: "TwitchOAuthToken",
//     },
//     nightbot: ServiceURL {
//         code: "http://localhost:8080/auth/nightbot/code",
//         refresh: "http://localhost:8080/auth/nightbot/refresh",
//         validate: "https://api.nightbot.tv/1/channel",
//         vault: "NightbotOAuthToken",
//     },
//     wizebot: ServiceURL {
//         code: "",
//         refresh: "",
//         validate: "",
//         vault: "WizeBotOAuthToken",
//     },
// };
