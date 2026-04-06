use image::flat::Error;
use rand::RngExt;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json;

mod window;

#[derive(Serialize, Deserialize)]
struct State {
    secs_left: u32,
    monkers_last_picked: Vec<PathBuf>,
}

impl State {
    fn save(&self) {
        fs::write("./data/state.json", serde_json::to_string(self).unwrap()).unwrap()
    }

    fn new() -> Self {
        State {
            secs_left: pick_time(),
            monkers_last_picked: Vec::new(),
        }
    }

    // That sounds mean.
    fn push_monkey(&mut self, filepath: PathBuf) {
        self.monkers_last_picked.insert(0, filepath);
        self.monkers_last_picked.truncate(3);
        self.save()
    }

    fn load() -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string("./data/state.json")?;

        Ok(serde_json::from_str(&contents)?)
    }

    fn load_or_default() -> Self {
        Self::load().unwrap_or_else(|_| Self::new())
    }
}

const MONKER_DIR: &str = "./data/monkers";

// NOTE: When should I use &Path vs PathBuf?
fn pick_monker(state: &mut State) -> PathBuf {
    let files: Vec<PathBuf> = match fs::read_dir(Path::new(MONKER_DIR)) {
        Ok(rd) => rd
            .filter_map(|res| res.ok())
            .map(|entry| entry.path())
            .collect(),
        Err(e) => panic!("Error when reading monkey dir: {}", e),
    };

    loop {
        let index = rand::rng().random_range(0..files.len());

        let picked = files[index].clone();

        if !state.monkers_last_picked.contains(&picked) {
            state.push_monkey(picked.clone());

            println!("New: {}", picked.display());
            return picked;
        }
        println!("Recent: {}", picked.display())
    }
}

fn pick_time() -> u32 {
    // picking secs + countdown instead of random future time prevents planned time when PC off.
    // Now on PC reboot, it can just continue the countdown.

    // return rand::rng().random_range(8 * 3600..16 * 3600);
    return 30;
}

fn main() {
    let mut state = State::load_or_default();

    let wait_interval_s = 1;

    // window::present(pick_monker());
    // exit(0);

    loop {
        if state.secs_left == 0 {
            window::present(pick_monker(&mut state));
            println!("Present: {}", pick_monker(&mut state).display());

            state.secs_left = pick_time();
            state.save();
        }

        state.secs_left -= wait_interval_s as u32;
        state.save();

        sleep(Duration::new(wait_interval_s, 0));
    }
}
