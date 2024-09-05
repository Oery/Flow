use log::{error, info};
use windows::{
    core::HSTRING,
    Security::Credentials::{PasswordCredential, PasswordVault},
};

pub fn store_token(resource: &str, token: &str) -> windows::core::Result<()> {
    let resource = resource.to_string() + "OAuthToken";

    let vault = PasswordVault::new()?;
    let resource_hstring = HSTRING::from(&resource);
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
                resource, error
            );
            let new_credential = PasswordCredential::CreatePasswordCredential(&resource_hstring, &flow_hstring, &token_hstring)?;
            vault.Add(&new_credential)?;
            info!("[VAULT | {}] Token stored successfully in vault", resource);
        }
    }

    Ok(())
}

pub fn get_token(resource: &str) -> windows::core::Result<String> {
    let resource = resource.to_string() + "OAuthToken";

    let vault = PasswordVault::new()?;
    let credential = vault.Retrieve(&HSTRING::from(resource), &HSTRING::from("Flow"))?;
    credential.RetrievePassword()?;
    Ok(credential.Password()?.to_string_lossy())
}

pub fn delete_token(resource: &str) -> windows::core::Result<()> {
    let resource = resource.to_string() + "OAuthToken";

    let vault = PasswordVault::new()?;
    let credential_result = vault.Retrieve(&HSTRING::from(resource), &HSTRING::from("Flow"));

    if let Ok(credential) = credential_result {
        vault.Remove(&credential)?;
    }

    Ok(())
}
