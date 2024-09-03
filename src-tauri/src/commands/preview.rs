use tauri::AppHandle;

use super::format::format_command;

#[tauri::command]
pub async fn preview_command(cmd_name: &str, cmd_text: &str, app: AppHandle) -> Result<String, String> {
    let mut command = cmd_text.to_string();
    command = format_command(cmd_name, command, &app).await?;
    Ok(command)
}
