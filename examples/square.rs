extern crate tetrahedrane;

use tetrahedrane::vid::*;
use tetrahedrane::start;
use tetrahedrane::shaders;
use tetrahedrane::texture;

fn main() {
    let mut window = start::Window::new(640, 480, "Hello World!", 1 as usize);

    let mut shaders: Vec<Shader> = Vec::new();

    let triangle = Triangle::new(DepthPoint::new(0.0, -0.5, 3.0),  
                                 DepthPoint::new(0.5, 0.5, 3.0), 
                                 DepthPoint::new(-0.5, 0.5, 3.0), 
                                 0.0, 0.0, 0.0,
                                 Color::new(200, 200, 200));

    let mut triangle_group = TriangleGroup::square(-0.5, -0.5, 3.0, 1.0, 1.0);

    //let mut triangle_group = TriangleGroup::new(vec![triangle]);

    let texture = texture::UVTexture::path(&"bmp/crate.bmp");
    shaders.push(shaders::filled_texture(1, texture, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0));

    let texture = texture::UVTexture::path(&"bmp/crate.bmp");
    shaders.push(shaders::filled_texture(2, texture, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0));
    //shaders.push(shaders::filled_triangle_color(1));
    //shaders.push(shaders::wireframe(1));

    //triangle_group.triangles[0].shader_ids[0] = 1;

    let mut frames = 0;

    let mut total_duration = std::time::Duration::new(0, 0);

    loop {
        let start_time = std::time::Instant::now();

        window.window.set(Color::new(20, 40, 60).orb_color());
        window.window.set(Color::new(20, 40, 60).orb_color());

        for triangle in &mut triangle_group.triangles {
            triangle.coord_rotate_x_y(0.5, 0.5, 0.01);
            triangle.coord_rotate_x_z(0.5, 0.0, 0.02);
            triangle.coord_rotate_y_z(0.5, 0.0, 0.03);
        }

        window.render_addon_shader(triangle_group.triangles[0], &shaders, [1, 0, 0, 0, 0, 0, 0, 0]); 
        window.render_addon_shader(triangle_group.triangles[1], &shaders, [2, 0, 0, 0, 0, 0, 0, 0]);

        window.window.sync();

        let end_time = std::time::Instant::now();

        let duration = end_time.duration_since(start_time);

        if total_duration.as_secs() < 1 {
            frames += 1;
            total_duration += duration;
        } else {
            println!("{:?}", frames);
            frames = 0;
            total_duration = std::time::Duration::new(0, 0);
        }
        //std::thread::sleep(std::time::Duration::from_millis(33));
    }
}