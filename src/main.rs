extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;

use std::ffi::CStr;

fn main() {
    
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem.window("Game", 900, 700).opengl().resizable().build().unwrap();

    let _gl_attr = video_subsystem.gl_attr();
    _gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    _gl_attr.set_context_version(4, 5);

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        let version = CStr::from_ptr( gl::GetString(gl::VERSION) as *const i8 ).to_string_lossy();
        let renderer = CStr::from_ptr( gl::GetString(gl::RENDERER) as *const i8 ).to_string_lossy();
        println!("Program start!\n GPU: {} \n GL: {} ", renderer , version);
    
        gl::Enable(gl::DEPTH_TEST); 
        gl::Viewport(0, 0, 900, 700); 
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }
    
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    println!("break loop");
                    break 'main
                },
                _ => {},
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        window.gl_swap_window();
    }

}
