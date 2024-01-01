// Copyright 2023 Felix Kahle. All rights reserved.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_parsing;
mod job_row;
mod parse_error;

use file_parsing::create_job_rows;
use job_row::{DispoMode, JobRow};
use parse_error::ParseFilesError;
use tauri::{Manager, Window};

/// Returns a list of all printers available on the system
/// as a vector of strings
///
/// # Returns
/// The list of printers as a vector of strings
#[tauri::command]
fn get_printer_names() -> Vec<String> {
    printers::get_printers().iter().map(|printer| printer.name.clone()).collect()
}

#[tauri::command]
fn parse_files(cl_view: String, shipper_site: String, mode: String) -> Result<Vec<JobRow>, ParseFilesError> {
    let mode: DispoMode = DispoMode::from_str(&mode)?;
    let rows = create_job_rows(&cl_view, &shipper_site, mode)?;
    Ok(rows)
}

/// Shows the splashscreen window
///
/// # Arguments
/// * `window` - The window manager
#[tauri::command]
async fn show_splashscreen(window: Window) {
    window
        .get_window("splashscreen")
        .expect("no window labeled 'splashscreen' found")
        .show()
        .unwrap();
}

/// Closes the splashscreen window and shows the main window
///
/// # Arguments
/// * `window` - The window manager
#[tauri::command]
async fn close_splashscreen(window: Window) {
    // Close splashscreen
    window
        .get_window("splashscreen")
        .expect("no window labeled 'splashscreen' found")
        .close()
        .unwrap();
    // Show main window
    window.get_window("main").expect("no window labeled 'main' found").show().unwrap();
}

// The main function that runs the application
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_printer_names,
            parse_files,
            show_splashscreen,
            close_splashscreen
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
