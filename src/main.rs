//#![windows_subsystem = "windows"]
mod model;
mod system;
use i_slint_backend_winit::WinitWindowAccessor;
use system::{system_window, run_main};
slint::include_modules!();

#[tokio::main]
async fn main() {

    let _ = slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new()));

    let main = Main::new().expect("Form initialization failed!");

    let main_weak = main.as_weak();
    main_weak
        .unwrap()
        .window()
        .with_winit_window(|winit_window: &winit::window::Window| {
            winit_window.set_decorations(false);
            winit_window.set_transparent(true);
        });
    system_window(main_weak);

    
    run_main(main);
    //main.run().expect("fail to start!");
}
