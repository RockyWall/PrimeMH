use serde_json::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::settings::Settings;

use super::cache;
use super::jsondata::SeedData;
use super::seeddata::SeedRequest;

pub fn get_seed_data(seed_request: SeedRequest) -> SeedData {
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
        Err(e) => {
            delete_cached_file(&cached_seed_data_file);
            panic!("{} {}", "Failed to generate map data!", e);
        }
    }
}

pub fn is_blacha_ok(settings: &Settings) -> Result<bool, String> {

    let seed_request = SeedRequest {
        map_seed: 123,
        difficulty: 2,
        d2lodpath: settings.general.d2lodpath.clone(),
        blacha_exe: settings.general.blacha_exe.clone(),
    };

    if !seed_request.d2lodpath.exists() {
        let msg = format!("Could not find d2lodpath {:?}, make sure you downloaded the d2lod zip as specified in #get-started.", &seed_request.d2lodpath);
        panic!("{}", msg);
    }
    if !seed_request.d2lodpath.join("d2data.mpq").exists() {
        let msg = format!("Could not find d2data.mpq in d2lodpath {:?}, make sure you downloaded the d2lod zip as specified in #get-started.", &seed_request.d2lodpath);
        panic!("{}", msg);
    }
    if !seed_request.blacha_exe.exists() {
        let msg = format!("Could not find d2-mapgen.exe {:?}, check your paths, check settings.toml, follow the instructions.", &seed_request.blacha_exe);
        panic!("{}", msg);
    }
    
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
        Ok(_) => Ok(true),
        Err(_) => {
            log::error!(
                "Error with generating map data from D2LoD\nCheck your D2LoD settings\nBlacha tool returned:\n{}",
                seed_data_str
            );
            panic!("Error generating map data, check you have Visual C++ installed. Restart your PC.\nMake sure you aren't running from your desktop or dropbox etc");
        }
    }
}

fn delete_cached_file(cached_seed_data_file: &PathBuf) {
    fs::remove_file(cached_seed_data_file).unwrap();
}

fn generate_data(seed_request: SeedRequest) -> String {
    let d2log_absolute_path = seed_request.d2lodpath.canonicalize().expect("Failed to get absolute path for d2lodpath");
    // generate data
    let output = Command::new(seed_request.blacha_exe)
        .arg(d2log_absolute_path)
        .arg("--seed")
        .arg(seed_request.map_seed.to_string())
        .arg("--difficulty")
        .arg(seed_request.difficulty.to_string())
        // .arg("--map")
        // .arg("1")
        .output()
        .unwrap();

    // parse stdout and clean it up
    let start_of_seed_data =
        format!("{{\"seed\":{},\"difficulty\":{},\"levels\":[", seed_request.map_seed, seed_request.difficulty);
    let mut seed_data = String::from(&start_of_seed_data);
    let stdout = String::from_utf8(output.stdout).unwrap();
    for line in stdout.lines() {
        if line.starts_with("{\"type\":\"map\"") {
            seed_data.push_str(line);
            seed_data.push(',');
        }
    }
    seed_data.pop();
    seed_data.push_str("]}");

    // save to file
    let cached_seed_data_file = cache::cached_file_name(&seed_request.map_seed, &seed_request.difficulty);
    fs::write(cached_seed_data_file, &seed_data).expect("Unable to write map data file");
    seed_data
}
