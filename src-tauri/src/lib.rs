use tauri_plugin_shell::ShellExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn v2ray(app_handle: tauri::AppHandle, arg: &str) -> Result<String, String> {
    let output = app_handle.shell()
        .command("v2ray")
        .args([arg])
        .output()
        .await
        .unwrap();

    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        Err(format!("Exit with code: {}", output.status.code().unwrap()))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![v2ray])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
