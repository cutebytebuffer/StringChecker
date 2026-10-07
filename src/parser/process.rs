use std::path::PathBuf;
use sysinfo::Pid;
use sysinfo::System;

pub struct Process {
    pub pid: u32,
    pub name: String,
    pub args: Vec<String>
}

impl Process {
    pub fn new(name: String) -> Self {
        let mut system = System::new_all();
        let mut pid = 0;
        let mut args: Vec<String> = Vec::new();

        system.refresh_all();

        for process in system.processes().values() {
            if process.name().to_str() == Some(name.as_str()) {
                pid = process.pid().as_u32();

                for arg in process.cmd() {
                    args.push(arg.to_string_lossy().into_owned());
                }

                break;
            }
        }

        Self {pid, name, args }
    }

    pub fn game_dir(&self) -> Option<PathBuf> {
        self.args
            .windows(2)
            .find(|args| args[0] == "--gameDir")
            .map(|args| PathBuf::from(&args[1]))
    }

    pub fn mod_dir(&self) -> Option<PathBuf> {
        self.game_dir().map(|dir| dir.join("mods"))
    }
}