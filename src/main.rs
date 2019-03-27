use cgmath::{
    BaseNum, Deg, Matrix4, Point2, Point3, Rad, SquareMatrix, Transform, Vector2, Vector3,
};
use std::time::{Duration, Instant};
use {
    sdl2, sdl2::event::Event, sdl2::keyboard::Keycode, sdl2::pixels::PixelFormatEnum,
    sdl2::render::WindowCanvas, sdl2::video::DisplayMode,
};

fn main() {
    let sdl_context = sdl2::init().expect("Failed to init SDL");
    let sdl_video = sdl_context.video().expect("Failed to init SDL video");
    let mut window = sdl_video
        .window("cgmath demo", 1024, 768)
        .position_centered()
        .build()
        .expect("Failed to create window");
    window
        .set_display_mode(DisplayMode::new(PixelFormatEnum::RGBA8888, 1024, 768, 60))
        .expect("Failed to set display mode");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create canvas");

    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");
    let start = Instant::now();
    loop {
        // clear to all black
        canvas.set_draw_color((0, 0, 0, 0));
        canvas.clear();

        // set color back to white
        canvas.set_draw_color((255, 255, 255, 255));

        // call our render function
        render(&mut canvas, start.elapsed().as_nanos() as f64 / 1.0e9);

        // show changes on the screen
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return,
                _ => {}
            }
        }
    }
}

fn draw_line<T: BaseNum>(canvas: &mut WindowCanvas, p1: Point3<T>, p2: Point3<T>) {
    let p1i = p1.cast::<i32>().unwrap();
    let p2i = p2.cast::<i32>().unwrap();
    canvas.draw_line((p1i.x, p1i.y), (p2i.x, p2i.y)).unwrap();
}

fn render(canvas: &mut WindowCanvas, time: f64) {
    let ul = Point3::new(100.0, 100.0, 0.0);
    let ur = Point3::new(1024.0 - 100.0, 100.0, 0.0);
    let ll = Point3::new(100.0, 768.0 - 100.0, 0.0);
    let lr = Point3::new(1024.0 - 100.0, 768.0 - 100.0, 0.0);

    let translate_to_center =
        Matrix4::from_translation(Vector3::new(-1024.0 / 2.0, -768.0 / 2.0, 0.0));
    let rotate = Matrix4::from_angle_y(Deg(time * 90.0));
    let translate_back = Matrix4::from_translation(Vector3::new(1024.0 / 2.0, 768.0 / 2.0, 0.0));

    let transform = translate_back * rotate * translate_to_center;
    let ul = transform.transform_point(ul);
    let ur = transform.transform_point(ur);
    let ll = transform.transform_point(ll);
    let lr = transform.transform_point(lr);

    draw_line(canvas, ul, ur);
    draw_line(canvas, ur, lr);
    draw_line(canvas, lr, ll);
    draw_line(canvas, ll, ul);
}
