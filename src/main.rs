use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use freedesktop_entry_parser::parse_entry;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    autostart_paths: Vec<String>,
    blocked_by_name: Vec<String>,
    blocked_by_path: Vec<String>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self { autostart_paths: vec!["$HOME/.config/autostart/".to_string()], blocked_by_name: Vec::new() , blocked_by_path: Vec::new() }
    }
}

fn main() -> Result<(), confy::ConfyError>{
    let cfg: Config = confy::load("autostart-cleaner", "config")?;
    for autostart_path in cfg.autostart_paths {
        let dir_path = Path::new(&autostart_path);
        if dir_path.is_dir(){
            let paths: Vec<PathBuf>  = fs::read_dir(dir_path).unwrap().map(|entry|{
                entry.unwrap().path()
            }).collect();
            for path in paths {
                let mut to_delete = false;
                let entry = parse_entry(&path).unwrap();
                if entry.section("Desktop Entry").has_attr("Name"){
                    let name = entry.section("Desktop Entry").attr("Name").unwrap();
                    if cfg.blocked_by_name.contains(&name.to_string()){
                        to_delete = true;
                    }
                }
                if entry.section("Desktop Entry").has_attr("Exec"){
                    let exec = entry.section("Desktop Entry").attr("Exec").unwrap();
                    if cfg.blocked_by_path.contains(&exec.to_string()){
                        to_delete = true;
                    }
                }
                if to_delete{
                    fs::remove_file(Path::new(&path)).unwrap();
                }
            }
        }
    }

    Ok(())
}
