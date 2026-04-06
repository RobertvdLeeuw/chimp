use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::wayland::WindowAttributesExtWayland;
use winit::window::{Window, WindowAttributes, WindowId};

use image::imageops::Lanczos3;
use image::{DynamicImage, GenericImageView, ImageReader, ImageResult};

use swayipc::Connection;

use std::path::{Path, PathBuf};

struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,

    width: u32,
    height: u32,

    img_pixels: Vec<u8>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Leak the Window so it has 'static lifetime
        let window: &'static Window = Box::leak(Box::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Chimp")
                        .with_inner_size(LogicalSize::new(self.width, self.height))
                        .with_name("chimp", "chimp"),
                )
                .unwrap(),
        ));

        // let size = window.inner_size();
        let surface = SurfaceTexture::new(self.width, self.height, window);
        let pixels = Pixels::new(self.width, self.height, surface).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let frame = self.pixels.as_mut().unwrap().frame_mut();

                frame.copy_from_slice(&self.img_pixels);
                self.pixels.as_mut().unwrap().render().unwrap();
            }
            _ => {}
        }
    }
}

const DISPLAY_RATIO: f32 = 0.7;
fn calc_display_size() -> (u32, u32) {
    let mut conn = Connection::new().expect("Failed to get sway connection.");
    let monitors = conn.get_outputs().unwrap();

    let (w, h) = monitors
        .iter()
        .filter(|mon| mon.focused)
        .map(|mon| mon.current_mode.unwrap())
        .map(|mode| (mode.width as f32, mode.height as f32))
        .collect::<Vec<(f32, f32)>>()[0];

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

pub fn present(img_filepath: PathBuf) {
    let (pixels, width, height) = get_normalized_image(img_filepath);

    let event_loop = EventLoop::new().unwrap();
    let mut app = App {
        window: None,
        pixels: None,

        width,
        height,

        img_pixels: pixels,
    };

    println!("Starting app");
    event_loop.run_app(&mut app).unwrap();
    println!("Closed app");
}
