extern crate sdl2;

use sdl2::surface::Surface;
use std::path::Path;

fn main() {
    
    let sdl2context = sdl2::init().unwrap(); // Initialise the SDL2 context instance
    let video_subsystem = sdl2context.video().unwrap(); // Initialise video subsystem instance
    let mut timer = sdl2context.timer().unwrap(); // Initialise the timer instance  

    // Create a new OpenGL window
    let window = match video_subsystem
        .window("Hello World!", 620, 387)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(build_window) => build_window,
        Err(err) => panic!("failed to create window: {}", err),
    };


    // Get the window surface, and prepare to handle events
    let event_pump = sdl2context.event_pump().unwrap();
    let mut window_surface = window.surface(&event_pump).unwrap();

    // Update the window surface to display it
    window_surface.update_window().unwrap();

    for _ in 0..20 {
        
        // Wait 100ms
        timer.delay(100);
    }
}
