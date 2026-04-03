use rand::RngExt;

use std::fs;
use std::path::{Path, PathBuf};
// use std::thread::sleep;
use std::time::{Duration, SystemTime};

// TODO:
// Collect monkey pics
// Pick random time
// Wait until time elapsed (countdown, solves "accidentally planned when PC off")
// Store time left in file
// On time:
//  Open window - black screen
//   Floating window (sway)
//    Pick random screen place
//  Fade in monkey pic
//  Drumroll

const MONKER_DIR: &str = "./monkers";

// NOTE: When should I use &Path vs PathBuf?
fn pick_monker() -> PathBuf {
    let files: Vec<PathBuf> = match fs::read_dir(Path::new(MONKER_DIR)) {
        Ok(rd) => rd
            .filter_map(|res| res.ok())
            .map(|entry| entry.path())
            .collect(),
        Err(e) => panic!("Error when reading monkey dir: {}", e),
    };

    let index = rand::rng().random_range(0..files.len());

    files[index].clone()
}

fn present() {
    let monker_pic = pick_monker();

    // Make window
}

fn update_time(sec_left: &mut u32, wait_s: u64) {
    *sec_left -= wait_s as u32;

    // Save time to file
    match fs::write("time_left.txt", sec_left.to_string()) {
        Ok(_) => {}
        Err(e) => panic!("Error saving timing file: {error}", error = e),
    };
}

fn pick_time(start: bool) -> u32 {
    // picking secs + countdown instead of random future time prevents planned time when PC off.
    // Now on PC reboot, it can just continue the countdown.
    if start {
        return rand::rng().random_range(8 * 3600..16 * 3600);
    }

    // NOTE: How can I avoid such nesting? I miss guard clauses.
    match fs::read_to_string("time_left.txt") {
        Ok(val) => {
            match val.parse::<u32>() {
                Ok(num) => return num,
                Err(_) => panic!("Timing file value not a number: {}", val),
            };
        }
        Err(e) => panic!("Error reading timing file: {}", e),
    }
}

fn main() {
    let mut sec_left: u32 = pick_time(true);

    let wait_s = 1;

    print!("{}", pick_monker().display());

    // loop {
    //     sleep(Duration::new(wait_s, 0));
    //     update_time(&mut sec_left, wait_s);
    //
    //     if sec_left == 0 {
    //         present();
    //         sec_left = pick_time(false)
    //     }
    // }
}
