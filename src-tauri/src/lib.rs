#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use tauri_plugin_opener::open_url;
use std::process::Command;
use std::sync::{Arc, Mutex};
use sysinfo::{CpuExt, System, SystemExt};
use tauri::AppHandle;


#[derive(Clone, Serialize)]
struct SystemStats {
    cpu: f32,
    ram: f32,
    gpu: f32,
    net_down: f64,
    net_up: f64,
}

struct SystemState {
    sys: System,
}

#[tauri::command]
fn get_system_stats(
    state: tauri::State<'_, Arc<Mutex<SystemState>>>,
) -> Result<SystemStats, String> {
    let mut state_guard = state
        .lock()
        .map_err(|_| "Failed to lock sysinfo state mutex".to_string())?;

    // Refresh only what is required to conserve CPU cycles
    state_guard.sys.refresh_cpu();
    state_guard.sys.refresh_memory();

    // Calculate global CPU usage
    let cpu_pct = state_guard.sys.global_cpu_info().cpu_usage();

    // Calculate RAM usage percentage
    let total_mem = state_guard.sys.total_memory() as f32;
    if total_mem == 0.0 {
        return Err("Total memory is reported as 0".into());
    }
    let used_mem = state_guard.sys.used_memory() as f32;
    let ram_pct = (used_mem / total_mem) * 100.0;

    Ok(SystemStats {
        cpu: cpu_pct,
        ram: ram_pct,
        gpu: 0.0, // Platform specific GPU metrics can be hooked here
        net_down: 0.0,
        net_up: 0.0,
    })
}

#[tauri::command]
fn execute_action(
    _app_handle: AppHandle,
    action_type: String,
    payload: String,
) -> Result<(), String> {
    println!(
        "Tauri executing action: {} with payload: {}",
        action_type, payload
    );

    match action_type.as_str() {
        "launch" => {
            if payload.starts_with("http://") || payload.starts_with("https://") {
                // Open standard browser link (Tauri v2, opener plugin)
                open_url(payload, None::<&str>)
                    .map_err(|e| format!("Failed to open URL: {}", e))?;
            } else {
                // Spawn local executable shell command in a separate background thread
                std::thread::spawn(move || {
                    #[cfg(target_os = "windows")]
                    let _ = Command::new("cmd").args(["/C", &payload]).spawn();

                    #[cfg(not(target_os = "windows"))]
                    let _ = Command::new("sh").args(["-c", &payload]).spawn();
                });
            }
        }
        "hotkey" => {
            // Hotkey execution on Linux via xdotool, xkb-cli or standard native macros
            let hotkey_payload = payload.clone();
            std::thread::spawn(move || {
                #[cfg(target_os = "windows")]
                {
                    // Basic fallback powershell script for keyboard events on windows
                    let ps_script = format!(
                        "$wshell = New-Object -ComObject wscript.shell; $wshell.SendKeys('{}')",
                        hotkey_payload
                    );
                    let _ = Command::new("powershell")
                        .args(["-Command", &ps_script])
                        .spawn();
                }

                #[cfg(target_os = "linux")]
                {
                    // Map common hotkeys to xdotool keysyms
                    let mapped_key = match hotkey_payload.as_str() {
                        "ctrl+shift+m" => "ctrl+shift+m",
                        "super+shift+s" => "super+shift+s",
                        "ctrl+shift+r" => "ctrl+shift+r",
                        "ctrl+shift+l" => "ctrl+shift+l",
                        _ => &hotkey_payload,
                    };
                    let _ = Command::new("xdotool").args(["key", mapped_key]).spawn();
                }
            });
        }
        _ => return Err(format!("Unsupported action type: {}", action_type)),
    }

    Ok(())
}

#[tauri::command]
fn adjust_system_setting(setting_type: String, direction: String) -> Result<(), String> {
    println!(
        "Adjusting setting: {} direction: {}",
        setting_type, direction
    );

    std::thread::spawn(move || {
        #[cfg(target_os = "linux")]
        {
            match setting_type.as_str() {
                "volume" => {
                    if direction == "up" {
                        let _ = Command::new("pactl")
                            .args(["set-sink-volume", "@DEFAULT_SINK@", "+5%"])
                            .status();
                    } else if direction == "down" {
                        let _ = Command::new("pactl")
                            .args(["set-sink-volume", "@DEFAULT_SINK@", "-5%"])
                            .status();
                    } else if direction == "mute" {
                        let _ = Command::new("pactl")
                            .args(["set-sink-mute", "@DEFAULT_SINK@", "toggle"])
                            .status();
                    }
                }
                "brightness" => {
                    if direction == "up" {
                        let _ = Command::new("brightnessctl").args(["set", "+10%"]).status();
                        let _ = Command::new("xbacklight").args(["-inc", "10"]).status();
                    } else if direction == "down" {
                        let _ = Command::new("brightnessctl").args(["set", "10%-"]).status();
                        let _ = Command::new("xbacklight").args(["-dec", "10"]).status();
                    }
                }
                _ => {}
            }
        }

        #[cfg(target_os = "windows")]
        {
            // Windows volume adjustment via powershell helper
            if setting_type == "volume" {
                if direction == "up" {
                    let _ = Command::new("powershell")
                        .args([
                            "-Command",
                            "(New-Object -ComObject Wscript.Shell).SendKeys([char]175)",
                        ])
                        .status();
                } else if direction == "down" {
                    let _ = Command::new("powershell")
                        .args([
                            "-Command",
                            "(New-Object -ComObject Wscript.Shell).SendKeys([char]174)",
                        ])
                        .status();
                } else if direction == "mute" {
                    let _ = Command::new("powershell")
                        .args([
                            "-Command",
                            "(New-Object -ComObject Wscript.Shell).SendKeys([char]173)",
                        ])
                        .status();
                }
            }
        }
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let sys_state = Arc::new(Mutex::new(SystemState {
        sys: System::new_all(),
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .manage(sys_state)
        .invoke_handler(tauri::generate_handler![
            get_system_stats,
            execute_action,
            adjust_system_setting
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
