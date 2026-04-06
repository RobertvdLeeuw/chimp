use pixels::{Pixels, SurfaceTexture};
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::platform::wayland::WindowAttributesExtWayland;
use winit::window::{Window, WindowAttributes, WindowId};

use image::imageops::Lanczos3;
use image::{DynamicImage, GenericImageView, ImageReader};

use swayipc::{Connection, Output};

use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;

pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,

    _sink: Option<MixerDeviceSink>,
    _player: Option<Player>,
    start_time: Option<Instant>,

    width: u32,
    height: u32,
    img_pixels: Vec<u8>,
}

impl App {
    pub fn new() -> Self {
        App {
            window: None,
            pixels: None,
            _sink: None,
            _player: None,
            start_time: None,
            width: 0,
            height: 0,
            img_pixels: Vec::new(),
        }
    }
}

impl ApplicationHandler<PathBuf> for App {
    // PathBuf is user event type
    fn user_event(&mut self, event_loop: &ActiveEventLoop, monker_path: PathBuf) {
        // Create window for new monker display
        let (pixels, width, height) = get_normalized_image(monker_path.clone());

        let mut sink = DeviceSinkBuilder::open_default_sink().unwrap();
        sink.log_on_drop(false);
        let player = rodio::play(
            sink.mixer(),
            BufReader::new(File::open("data/Drumroll.mp3").unwrap()),
        )
        .unwrap();

        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Chimp")
                        .with_inner_size(LogicalSize::new(width, height))
                        .with_name("chimp", "chimp"),
                )
                .unwrap(),
        );

        let surface = SurfaceTexture::new(width, height, window.clone());
        let px = Pixels::new(width, height, surface).unwrap();

        self.window = Some(window);
        self.pixels = Some(px);
        self._sink = Some(sink);
        self._player = Some(player);
        self.start_time = Some(Instant::now());
        self.width = width;
        self.height = height;
        self.img_pixels = pixels;

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                // User manually closed - clean up this window
                self.pixels = None;
                self.window = None;
                self._sink = None;
                self._player = None;
                // Don't exit event loop - just wait for next monker
            }
            WindowEvent::RedrawRequested => {
                if self.pixels.is_none() {
                    return;
                }

                let frame = self.pixels.as_mut().unwrap().frame_mut();

                if self.start_time.unwrap().elapsed() < Duration::from_millis(4_390) {
                    frame.fill(0);
                } else {
                    frame.copy_from_slice(&self.img_pixels);
                }

                self.pixels.as_mut().unwrap().render().unwrap();
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }

    fn resumed(&mut self, _: &ActiveEventLoop) {}
}

const DISPLAY_RATIO: f32 = 0.7;
fn calc_display_size() -> (u32, u32) {
    let mut conn = Connection::new().expect("Failed to get sway connection.");
    let monitors = conn.get_outputs().unwrap();

    let cur_mon = monitors
        .iter()
        .filter(|mon| mon.focused)
        .collect::<Vec<&Output>>()[0];

    let mon_angle: &'static str = cur_mon.transform.clone().unwrap().leak();

    let mode = cur_mon.current_mode.unwrap();

    let (w, h) = match mon_angle {
        "90" | "270" | "flipped-90" | "flipped-270" => (mode.height as f32, mode.width as f32),
        "normal" | "180" | "flipped-180" => (mode.width as f32, mode.height as f32),
        _ => panic!("Unsupported monitor rotation: {}", mon_angle),
    };

    ((w * DISPLAY_RATIO) as u32, (h * DISPLAY_RATIO) as u32)
}

fn get_normalized_image(img_filepath: PathBuf) -> (Vec<u8>, u32, u32) {
    let img = ImageReader::open(img_filepath).unwrap().decode().unwrap();

    let (max_w, max_h) = calc_display_size();

    let resized = DynamicImage::resize(&img, max_w, max_h, Lanczos3);

    let img_buffer = resized.to_rgba8();

    let (width, height) = resized.dimensions();

    (img_buffer.as_raw().to_vec(), width, height)
}
