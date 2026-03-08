use tauri_plugin_cli::CliExt;

fn main() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_cli::init());

    #[cfg(desktop)]
    {
        builder = builder.setup(|app| {
            // Get CLI matches
            let matches = app.cli().matches().unwrap();
            
            println!("CLI Plugin Integration Test");
            println!("===========================");
            println!("Arguments received: {:?}", matches.args);
            
            if let Some(subcommand) = matches.subcommand {
                println!("Subcommand: {}", subcommand.name);
                println!("Subcommand args: {:?}", subcommand.matches.args);
                
                if subcommand.name == "test-command" {
                    println!("\n✅ SUCCESS: CLI plugin is working!");
                    println!("Test command executed successfully.");
                    std::process::exit(0);
                }
            }
            
            if matches.args.get("test").is_some() {
                println!("\n✅ SUCCESS: CLI plugin is working!");
                println!("Test flag detected.");
                std::process::exit(0);
            }
            
            println!("\n✅ CLI plugin is properly initialized!");
            println!("Use --help to see available commands.");
            
            Ok(())
        });
    }

    builder
        .invoke_handler(tauri::generate_handler![
            test_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn test_command() -> String {
    "CLI plugin works!".to_string()
}
