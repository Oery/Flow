use log::{error, info};
use std::error::Error;

use windows::{
    core::HSTRING,
    Security::Credentials::{PasswordCredential, PasswordVault},
};

use super::oauth_services::Service;

pub fn store_token(service: &Service, token: &str) -> Result<(), Box<dyn Error>> {
    let resource = service.get_vault();

    let vault = PasswordVault::new()?;
    let resource_hstring = HSTRING::from(resource);
    let flow_hstring = HSTRING::from("Flow");
    let token_hstring = HSTRING::from(token);

    let credential_result = vault.Retrieve(&resource_hstring, &flow_hstring);
    match credential_result {
        Ok(credential) => {
            vault.Remove(&credential)?;
            let new_credential = PasswordCredential::CreatePasswordCredential(&resource_hstring, &flow_hstring, &token_hstring)?;
            vault.Add(&new_credential)?;
        }
        Err(error) => {
            error!(
                "[VAULT | {}] Token failed to save. Vault may miss the service, error: {}",
                service, error
            );
            let new_credential = PasswordCredential::CreatePasswordCredential(&resource_hstring, &flow_hstring, &token_hstring)?;
            vault.Add(&new_credential)?;
            info!("[VAULT | {}] Token stored successfully in vault", service);
        }
    }

    Ok(())
}

pub fn get_token(service: &Service) -> Result<String, Box<dyn Error>> {
    let resource = service.get_vault();

    let vault = PasswordVault::new()?;
    let credential = vault.Retrieve(&HSTRING::from(resource), &HSTRING::from("Flow"))?;
    credential.RetrievePassword()?;
    Ok(credential.Password()?.to_string_lossy())
}

pub fn delete_token(resource: &str) -> Result<(), Box<dyn Error>> {
    let vault = PasswordVault::new()?;
    let credential_result = vault.Retrieve(&HSTRING::from(resource), &HSTRING::from("Flow"));

    if let Ok(credential) = credential_result {
        vault.Remove(&credential)?;
    }

    Ok(())
}
