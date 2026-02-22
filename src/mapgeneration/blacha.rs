use serde_json::Error;
use std::os::windows::process::CommandExt;
use std::time::Instant;
use std::{env, fs};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::LOCALISATION;
use crate::settings::Settings;

use super::cache;
use super::jsondata::SeedData;
use super::seeddata::SeedRequest;

pub fn get_seed_data(seed_request: SeedRequest) -> SeedData {
    let localisation = LOCALISATION.lock().unwrap();
    let cached_seed_data_file = cache::cached_file_name(&seed_request.map_seed, &seed_request.difficulty);
    let seed_data_str: String = if Path::new(&cached_seed_data_file).exists() {
        log::info!("Reading cached map data from file {}", &cached_seed_data_file.to_str().unwrap());
        cache::read_cached_file(&cached_seed_data_file)
    } else {
        log::info!(
            "Generating fresh data for seed {} and difficulty {}",
            seed_request.map_seed,
            seed_request.difficulty
        );
        generate_data(seed_request)
    };
    let json: Result<SeedData, Error> = serde_json::from_str(&seed_data_str);
    match json {
        Ok(json) => json,
        Err(_e) => {
            delete_cached_file(&cached_seed_data_file);
            log::debug!("Couldn't get seed data");
            log::error!("{}", localisation.get_primemh("error7"));
            panic!("Failed to parse SeedData from JSON");
        }
    }
}

pub fn is_blacha_ok(settings: &Settings) -> Result<bool, String> {
    let localisation = LOCALISATION.lock().unwrap();
    let seed_request = SeedRequest {
        map_seed: 123,
        difficulty: 2,
        d2lodpath: settings.general.d2lodpath.clone(),
        blacha_exe: settings.general.blacha_exe.clone(),
    };

    let d2lodpath = get_path_as_str(&settings.general.d2lodpath);
    
    if !settings.general.d2lodpath.exists() {
        let msg = format!("{}\n{}", localisation.get_primemh("error8"), d2lodpath);
        log::error!("{}", msg);
    }
    if !settings.general.d2lodpath.join("d2data.mpq").exists() {
        let msg = format!("{}\n{}", localisation.get_primemh("error8"), d2lodpath);
        log::error!("{}", msg);
    }

    let blacha_exe = get_path_as_str(&settings.general.blacha_exe);
    
    if !settings.general.blacha_exe.exists() {
        let msg = format!("{}\n{}", localisation.get_primemh("error10"), blacha_exe);
        log::error!("{}", msg);
    }
    let d2lodpath = seed_request.d2lodpath.clone().canonicalize().expect("Failed to get absolute path for d2lodpath");
    
    log::info!(
        "Generating fresh data for seed {} and difficulty {} d2lod: {} blacha: {}",
        seed_request.map_seed,
        seed_request.difficulty,
        seed_request.d2lodpath.to_string_lossy(),
        seed_request.blacha_exe.to_string_lossy()
    );
    let seed_data_str = generate_data(seed_request);

    let json: Result<SeedData, Error> = serde_json::from_str(&seed_data_str);
    match json {
        Ok(_) => {
            log::info!("Blacha test generation successful!");
            Ok(true)
        },
        Err(e) => {
            let d2log_absolute_path = d2lodpath.to_str().unwrap();
            log::debug!("Path used: {}", d2log_absolute_path);
            let forbidden_folders: Vec<&str> = vec!["Desktop", "Dropbox", "Google Drive", "OneDrive"];
            
            for folder in forbidden_folders {
                if d2log_absolute_path.contains(folder) {
                    log::error!("{}\n{}", localisation.get_primemh("error1"), folder);
                }
            }
            log::error!("{}", localisation.get_primemh("error2"));
            Err(e.to_string())
        }
    }
}

fn get_path_as_str(path: &PathBuf) -> String {
    let new_path = if path.is_absolute() {
        PathBuf::from(path)
    } else {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        current_dir.join(path)
    };
    new_path.to_string_lossy().replace("/","\\").replace("\\\\","\\")
}

fn delete_cached_file(cached_seed_data_file: &PathBuf) {
    fs::remove_file(cached_seed_data_file).unwrap();
}

fn is_running_in_wine() -> bool {
    env::var("WINEDEBUG").is_ok() || env::var("WINEPREFIX").is_ok()
}

fn generate_data(seed_request: SeedRequest) -> String {
    let d2lod_absolute_path = seed_request.d2lodpath.canonicalize().expect("Failed to get absolute path for d2lodpath");
    // generate data
    let start = Instant::now();

    let output = if is_running_in_wine() {
        log::info!("Running in wine d2lod_absolute_path: {:?} blacha {:?}", d2lod_absolute_path.display(), seed_request.blacha_exe);
    
        Command::new("wine")
            .arg(seed_request.blacha_exe)
            .arg(d2lod_absolute_path)
            .arg("--seed")
            .arg(seed_request.map_seed.to_string())
            .arg("--difficulty")
            .arg(seed_request.difficulty.to_string())
            // .arg("--map")
            // .arg("1")
            .env("WINEPREFIX", "/app/wine_d2")
            .env("WINEDEBUG", "-all,fixme-all")
            .env("WINEARCH", "win32")
            .output()
            .unwrap()
    } else {
        Command::new(seed_request.blacha_exe)
            .creation_flags(0x08000000)
            .arg("/C")
            .arg(d2lod_absolute_path)
            .arg("--seed")
            .arg(seed_request.map_seed.to_string())
            .arg("--difficulty")
            .arg(seed_request.difficulty.to_string())
            // .arg("--map")
            // .arg("1")
            .output()
            .unwrap()
    };

    log::info!("Map data generation took {:.3} seconds", (start.elapsed().as_millis() as f64 / 1000.0));

    // parse stdout and clean it up
    let start_of_seed_data =
        format!("{{\"seed\":{},\"difficulty\":{},\"levels\":[", seed_request.map_seed, seed_request.difficulty);
    let mut seed_data = String::from(&start_of_seed_data);
    let stdout = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(e) => {
            log::error!("Failed to parse stdout for map generation: {}", e);
            String::new()
        },
    };
    let mut found = false;
    for line in stdout.lines() {
        if line.starts_with("{\"type\":\"map\"") {
            seed_data.push_str(line);
            seed_data.push(',');
            found = true;
        }
    }
    seed_data.pop();
    seed_data.push_str("]}");

    // save to file
    if seed_request.map_seed == 123 {
        if !found {
            log::debug!("{}", stdout);
        }
        return seed_data
    }
    let cached_seed_data_file = cache::cached_file_name(&seed_request.map_seed, &seed_request.difficulty);
    fs::write(cached_seed_data_file, &seed_data).expect("Unable to write map data file");
    seed_data
}



#[cfg(test)]
mod tests {
    use std::fs::{self, OpenOptions};
    use std::io::Write;
    use crate::logger::configure_logging;
    use crate::{mapgeneration::{blacha, jsondata::SeedData, seeddata::SeedRequest}};

    
    #[test]
    fn generate_test_seeds() {
        // configure_logging();

        let logfile = std::path::Path::new("./test_generate_seeds.log");
        let settings = crate::settings::Settings::default();
        for i in 1..=u32::MAX {
            let seed_request = SeedRequest {
                map_seed: i,
                difficulty: 0,
                d2lodpath: settings.general.d2lodpath.clone(),
                blacha_exe: settings.general.blacha_exe.clone(),
            };
            let seed_data_json: SeedData = blacha::get_seed_data(seed_request);
            assert_eq!(seed_data_json.seed, i);

            let rogue = seed_data_json.levels.get(0).expect("No level data found");
            let cold = seed_data_json.levels.get(1).expect("No level data found");
            let loggedline = format!("{:?},{:?},{:?},{:?},{:?}", i, rogue.offset.x, rogue.offset.y, cold.offset.x, cold.offset.y);
            // log::info!("{}", loggedline);
            {
                let mut file = OpenOptions::new().create(true).append(true).open(logfile).unwrap();
                writeln!(file, "{}", loggedline).unwrap();
            }
            
        }
        
    }
}