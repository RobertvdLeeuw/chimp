use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::wayland::WindowAttributesExtWayland;
use winit::window::{Window, WindowAttributes, WindowId};

use std::path::{Path, PathBuf};

struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
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
        println!("1");

        let size = window.inner_size();
        println!("2");
        let surface = SurfaceTexture::new(size.width, size.height, window);
        println!("3");
        let pixels = Pixels::new(800, 600, surface).unwrap();
        println!("4");

        self.window = Some(window);
        self.pixels = Some(pixels);
        println!("5");
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                // Render here
                println!("render");
                let frame = self.pixels.as_mut().unwrap().frame_mut();
                frame.fill(0);
                self.pixels.as_mut().unwrap().render().unwrap();
            }
            _ => {}
        }
    }
}

pub fn present(img_filepath: PathBuf) {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App {
        window: None,
        pixels: None,
        // start_time: None,
    };

    println!("Starting app");
    event_loop.run_app(&mut app).unwrap();
    println!("Closed app");
}
