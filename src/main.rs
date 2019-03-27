use cgmath::{Point2, Vector2};
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
    loop {
        // clear to all black
        canvas.set_draw_color((0, 0, 0, 0));
        canvas.clear();

        // set color back to white
        canvas.set_draw_color((255, 255, 255, 255));

        // call our render function
        render(&mut canvas);

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

fn draw_line(canvas: &mut WindowCanvas, p1: Point2<i32>, p2: Point2<i32>) {
    canvas.draw_line((p1.x, p1.y), (p2.x, p2.y)).unwrap();
}

fn render(canvas: &mut WindowCanvas) {
    let p0 = Point2::new(0, 0);
    let v1 = Vector2::new(400, 100);
    draw_line(canvas, p0, p0 + v1);
}
