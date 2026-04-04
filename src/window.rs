use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::wayland::WindowAttributesExtWayland;
use winit::window::{Window, WindowAttributes, WindowId};

use image::io::Reader as ImageReader;
use image::{ImageResult, image_dimensions};

use std::path::{Path, PathBuf};

struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,

    width: u32,
    height: u32,

    img_path: PathBuf,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Leak the Window so it has 'static lifetime
        let window: &'static Window = Box::leak(Box::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Chimp")
                        .with_name("chimp", "chimp"),
                )
                .unwrap(),
        ));

        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, window);
        let pixels = Pixels::new(self.width, self.height, surface).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                // Render here
                println!("render");
                let frame = self.pixels.as_mut().unwrap().frame_mut();

                let img = ImageReader::open(self.img_path.as_os_str())
                    .unwrap()
                    .decode()
                    .unwrap();

                let rgba = img.to_rgba8();
                let raw_pixels: &[u8] = rgba.as_raw();

                frame.copy_from_slice(raw_pixels);
                self.pixels.as_mut().unwrap().render().unwrap();
            }
            _ => {}
        }
    }
}

pub fn present(img_filepath: PathBuf) {
    let (width, height) = match image_dimensions(img_filepath.as_os_str()) {
        ImageResult::Ok((w, h)) => (w, h),
        ImageResult::Err(e) => panic!("Error getting file dimensions: {}", e),
    };

    let event_loop = EventLoop::new().unwrap();
    let mut app = App {
        window: None,
        pixels: None,

        width,
        height,

        img_path: img_filepath,
    };

    println!("Starting app");
    event_loop.run_app(&mut app).unwrap();
    println!("Closed app");
}
