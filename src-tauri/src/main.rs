#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serialport::SerialPortInfo;
use tauri::Manager;

// Import the serialport module
mod serialport;

fn main() {
    // Set up the Tauri application
    tauri::Builder::default()
    .setup(|app| {
        #[cfg(debug_assertions)]
        app.get_window("main").unwrap().open_devtools();
        Ok(())
      })
        // Define the command that can be called from the frontend
        .invoke_handler(tauri::generate_handler![get_serial_ports])
        // Run the Tauri application
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Tauri command to list available serial ports.
/// This function calls the `get_available_ports` function from the `serialport` module.
#[tauri::command]
fn get_serial_ports() -> Result<Vec<SerialPortInfo>, String> {
    
    // Call the function from the serialport module and map errors to strings
    let ports = serialport::get_available_ports().map_err(|e| e.to_string());

    // Unwrap the ports
    let ports_unwrapped = ports.unwrap();

    // Return the result
    Ok(ports_unwrapped)
}
