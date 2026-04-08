use rand::RngExt;

use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use winit::event_loop::EventLoop;

use serde::{Deserialize, Serialize};

mod window;

#[derive(Serialize, Deserialize)]
struct State {
    time_left_s: u32,
    monkers_last_picked: Vec<PathBuf>,
}

impl State {
    fn save(&self) {
        fs::write(
            "./data/state.json",
            serde_json::to_string(self).expect("Failed to serialize state to JSON"),
        )
        .expect("Failed to write state.json - check disk space and permissions")
    }

    fn new() -> Self {
        State {
            time_left_s: pick_time(),
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

fn pick_monker(state: &mut State) -> PathBuf {
    let files: Vec<PathBuf> = fs::read_dir(Path::new(MONKER_DIR))
        .expect("Failed to open monkers directory - does ./data/monkers exist?")
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .collect();

    loop {
        let index = rand::rng().random_range(0..files.len());

        let picked = files[index].clone();

        if !state.monkers_last_picked.contains(&picked) {
            state.push_monkey(picked.clone());

            return picked;
        }
    }
}

fn pick_time() -> u32 {
    // Picking secs + countdown instead of random future time prevents planned time when PC off.
    // Now on PC reboot, it can just continue the countdown.

    rand::rng().random_range(8 * 3600..16 * 3600)
}

fn main() {
    let mut state = State::load_or_default();

    let event_loop = EventLoop::with_user_event()
        .build()
        .expect("Failed to create winit event loop");
    let proxy = event_loop.create_proxy();

    let wait_interval_s = 5;

    // Countdown thread - manages state and timing
    thread::spawn(move || {
        let mut save_loop_tracker = 0;
        loop {
            thread::sleep(Duration::from_secs(wait_interval_s));

            state.time_left_s -= wait_interval_s as u32;

            // Save every min.
            save_loop_tracker += 1;
            if save_loop_tracker == 12 {
                state.save();
                save_loop_tracker = 0;
            }

            if state.time_left_s == 0 {
                let monker_path = pick_monker(&mut state);
                proxy
                    .send_event(monker_path)
                    .expect("UI thread died - can't send monker event");

                state.time_left_s = pick_time();
                state.save();
            }
        }
    });

    let mut app = window::App::new();
    event_loop.run_app(&mut app).expect("Event loop crashed");
}
