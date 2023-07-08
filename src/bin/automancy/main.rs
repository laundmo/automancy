use env_logger::Env;
use expect_dialog::ExpectDialog;
use futures::executor::block_on;
use tokio::runtime::Runtime;
use winit::event_loop::EventLoop;
use winit::window::{Fullscreen, Icon, WindowBuilder};

use automancy::camera::Camera;
use automancy::gpu::Gpu;
use automancy::input::KeyActions;
use automancy::renderer::Renderer;
use automancy_defs::gui::{init_gui, set_font};
use automancy_defs::{log, window};

use crate::event::{on_event, EventLoopStorage};
use crate::setup::GameSetup;

pub static LOGO: &[u8] = include_bytes!("assets/logo.png");

mod event;
mod gui;
mod setup;

/// Gets the game icon.
fn get_icon() -> Icon {
    let image = image::load_from_memory(LOGO).unwrap().to_rgba8();
    let width = image.width();
    let height = image.height();

    Icon::from_rgba(image.into_flat_samples().samples, width, height).unwrap() // unwrap ok
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let runtime = Runtime::new().unwrap();

    // --- window ---
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("automancy")
        .with_window_icon(Some(get_icon()))
        .build(&event_loop)
        .expect_dialog("Failed to open window!");

    let camera = Camera::new(window::window_size_double(&window));

    // --- setup ---
    let (mut setup, vertices, indices) = runtime
        .block_on(GameSetup::setup(camera))
        .expect_dialog("Critical failure in game setup!");

    // --- render ---
    log::info!("setting up rendering...");
    let gpu = block_on(Gpu::new(
        window,
        &setup.resource_man,
        vertices,
        indices,
        setup.options.graphics.fps_limit == 0.0,
    ));
    log::info!("render setup.");

    // --- gui ---
    log::info!("setting up gui...");
    let mut gui = init_gui(
        egui_wgpu::Renderer::new(&gpu.device, gpu.config.format, None, 1),
        &gpu.window,
    );
    set_font(setup.options.gui.font.clone(), &mut gui);
    log::info!("gui set up.");

    let mut renderer = Renderer::new(setup.resource_man.clone(), gpu);

    let mut storage = EventLoopStorage::default();

    event_loop.run(move |event, _, control_flow| {
        let _ = on_event(
            &runtime,
            &mut setup,
            &mut storage,
            &mut renderer,
            &mut gui,
            event,
            control_flow,
        );

        renderer
            .gpu
            .set_vsync(setup.options.graphics.fps_limit == 0.0);
        setup.options.graphics.fullscreen = setup.input_handler.key_active(&KeyActions::Fullscreen);
        if setup.options.graphics.fullscreen {
            renderer
                .gpu
                .window
                .set_fullscreen(Some(Fullscreen::Borderless(None)));
        } else {
            renderer.gpu.window.set_fullscreen(None);
        }
    });
}
