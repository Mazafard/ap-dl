use std::path::Path;
use std::process::Command;

pub fn replace_and_relaunch(new_binary: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = std::fs::metadata(new_binary) {
            let mut perms = metadata.permissions();
            perms.set_mode(0o755);
            let _ = std::fs::set_permissions(new_binary, perms);
        }
    }

    self_replace::self_replace(new_binary).map_err(|e| format!("Self replace error: {}", e))?;

    let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
    
    // Spawn updated process detached
    Command::new(&current_exe)
        .spawn()
        .map_err(|e| format!("Failed to relaunch new binary: {}", e))?;

    // Exit old process cleanly
    std::process::exit(0);
}
