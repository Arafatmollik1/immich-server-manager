use std::fs;
use std::process::Command;
use tauri::Manager;
use tauri::menu::{MenuBuilder, MenuItemBuilder}; 
use tauri::tray::TrayIconBuilder; 

#[tauri::command]
fn check_docker() -> Result<String, String> {
    let output = Command::new("docker")
        .arg("info")
        .output()
        .map_err(|e| format!("Failed to execute docker: {}", e))?;

    if output.status.success() {
        Ok("OK".to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
        if stderr.contains("permission denied") {
            Err("PERMISSION_DENIED: Please run 'sudo usermod -aG docker $USER' in your terminal, then log out and back in.".to_string())
        } else {
            Err("DAEMON_OFFLINE: Docker is installed, but the engine is not running. Please start the Docker daemon.".to_string())
        }
    }
}

#[tauri::command]
async fn setup_immich_folder() -> Result<String, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let immich_dir = home_dir.join(".immich-server");
    let postgres_dir = immich_dir.join("postgres");

    fs::create_dir_all(&postgres_dir).map_err(|e| format!("Failed to create directories: {}", e))?;

    let compose_url = "https://github.com/immich-app/immich/releases/latest/download/docker-compose.yml";
    let env_url = "https://github.com/immich-app/immich/releases/latest/download/example.env";

    let compose_text = reqwest::get(compose_url).await.map_err(|e| format!("Failed to download compose file: {}", e))?
        .text().await.map_err(|e| e.to_string())?;
    
    let env_text = reqwest::get(env_url).await.map_err(|e| format!("Failed to download env file: {}", e))?
        .text().await.map_err(|e| e.to_string())?;

    fs::write(immich_dir.join("docker-compose.yml"), compose_text).map_err(|e| e.to_string())?;
    fs::write(immich_dir.join(".env"), env_text).map_err(|e| e.to_string())?;

    Ok(format!("Server files successfully installed in ~/.immich-server"))
}

#[tauri::command]
fn update_env_file(upload_path: String) -> Result<String, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let immich_dir = home_dir.join(".immich-server");
    let env_path = immich_dir.join(".env");
    let db_path = immich_dir.join("postgres");

    let env_content = fs::read_to_string(&env_path)
        .map_err(|e| format!("Failed to read .env file. Did you download the server files first?: {}", e))?;

    let mut new_env_content = String::new();
    for line in env_content.lines() {
        if line.starts_with("UPLOAD_LOCATION=") {
            new_env_content.push_str(&format!("UPLOAD_LOCATION={}\n", upload_path));
        } else if line.starts_with("DB_DATA_LOCATION=") {
            new_env_content.push_str(&format!("DB_DATA_LOCATION={}\n", db_path.display()));
        } else {
            new_env_content.push_str(line);
            new_env_content.push('\n');
        }
    }

    fs::write(&env_path, new_env_content)
        .map_err(|e| format!("Failed to save .env file: {}", e))?;

    Ok("Configuration updated! Your server is ready to launch.".to_string())
}

#[tauri::command]
fn start_server() -> Result<String, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let immich_dir = home_dir.join(".immich-server");

    let output = Command::new("docker")
        .current_dir(&immich_dir)
        .args(["compose", "up", "-d"])
        .output()
        .map_err(|e| format!("Failed to execute docker compose: {}", e))?;

    if output.status.success() {
        Ok("Server is LIVE! 🟢".to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to start server: {}", stderr))
    }
}

#[tauri::command]
fn stop_server() -> Result<String, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let immich_dir = home_dir.join(".immich-server");

    let output = Command::new("docker")
        .current_dir(&immich_dir)
        .args(["compose", "down"])
        .output()
        .map_err(|e| format!("Failed to execute docker compose: {}", e))?;

    if output.status.success() {
        Ok("Server is DEAD 🔴. Safe to close or unplug.".to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to stop server: {}", stderr))
    }
}

#[tauri::command]
fn check_server_status() -> Result<bool, String> {
    let output = Command::new("docker")
        .args(["ps", "--filter", "name=immich", "--format", "{{.Names}}"])
        .output()
        .map_err(|e| format!("Failed to check docker status: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    if stdout.trim().is_empty() {
        Ok(false) 
    } else {
        Ok(true)  
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        
        .setup(|app| {
            let show_btn = MenuItemBuilder::with_id("show", "Open Dashboard").build(app)?;
            let quit_btn = MenuItemBuilder::with_id("quit", "Quit Server Manager").build(app)?;
            let menu = MenuBuilder::new(app).items(&[&show_btn, &quit_btn]).build()?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        std::process::exit(0); 
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();    
                window.hide().unwrap(); 
            }
            _ => {}
        })

        .invoke_handler(tauri::generate_handler![
            check_docker, 
            setup_immich_folder, 
            update_env_file, 
            start_server, 
            stop_server,
            check_server_status 
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}