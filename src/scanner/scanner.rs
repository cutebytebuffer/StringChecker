use std::fs;
use std::path::PathBuf;

use crate::parser::jar::Jar;
use crate::parser::process::Process;
use crate::scanner::signatures::CHEATS;
use crate::ui::App;

pub fn scan(app: &mut App) {
    let process = Process::new("javaw.exe".to_string());
    let mods: Vec<PathBuf> = get_mods(&process);

    app.log(format!("Found {} mods", mods.len()));

    for jar in mods {
        let jar_name = jar
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown.jar");

        app.log(format!("Scanning {jar_name}"));

        match Jar::new(jar.clone()) {
            Ok(current) => {
                for string in current.strings {
                    let normalized = normalize(&string);

                    for cheat in CHEATS {
                        if cheat
                            .strings
                            .iter()
                            .any(|signature| *signature == normalized)
                        {
                            app.log(format!(
                                "Possible {} found in {}: {}",
                                cheat.name,
                                jar_name,
                                string
                            ));
                        }
                    }
                }
            }

            Err(e) => {
                app.log(format!(
                    "Failed to read {}: {e}",
                    jar_name
                ));
            }
        }
    }

    app.log("Scan complete");
}
pub fn get_mods(process: &Process) -> Vec<PathBuf> {
    let Some(dir) = process.mod_dir() else {
        return Vec::new();
    };

    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };

    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("jar")))
        .collect()
}

pub fn normalize(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}
