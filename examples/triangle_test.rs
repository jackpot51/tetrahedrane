extern crate tetrahedrane;

use tetrahedrane::vid::*;
use tetrahedrane::start;

fn main() {

    let mut window = start::Window::new(640, 480, "Hello World!", 1 as usize);

    window.window.set(Color::new(20, 40, 60).orb_color());

    let triangle = Triangle::new(DepthPoint::new(0.0, -0.5, 3.0),  
                                 DepthPoint::new(0.5, 0.5, 3.0), 
                                 DepthPoint::new(-0.5, 0.5, 3.0), 
                                 0.0, 0.0, 0.0,
                                 Color::new(200, 200, 200));

    'game_loop: loop {
        window.window.set(Color::new(20, 40, 60).orb_color());

        window.render_queue.push(triangle);
        window.render();

        window.window.sync();

        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}