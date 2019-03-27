use cgmath::{
    BaseNum, Deg, Matrix4, Point2, Point3, Rad, SquareMatrix, Transform, Vector2, Vector3,
};
use itertools::Itertools;
use std::path::Path;
use std::time::{Duration, Instant};
use tobj;
use tobj::{Mesh, Model};
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

    let (models, _) = tobj::load_obj(&Path::new("car.obj")).expect("Failed to load model file");

    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");
    let start = Instant::now();
    loop {
        // clear to all black
        canvas.set_draw_color((0, 0, 0, 0));
        canvas.clear();

        // set color back to white
        canvas.set_draw_color((255, 255, 255, 255));

        // call our render function
        render(
            &mut canvas,
            &models,
            start.elapsed().as_nanos() as f32 / 1.0e9,
        );

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

trait PositionsAsPoint3 {
    fn get_position(&self, index: u32) -> Option<Point3<f32>>;
}

impl PositionsAsPoint3 for Mesh {
    fn get_position(&self, index: u32) -> Option<Point3<f32>> {
        match (
            self.positions.get(index as usize * 3),
            self.positions.get(index as usize * 3 + 1),
            self.positions.get(index as usize * 3 + 2),
        ) {
            (Some(&x), Some(&y), Some(&z)) => Some(Point3::new(x, y, z)),
            _ => None,
        }
    }
}

fn draw_line<T: BaseNum>(canvas: &mut WindowCanvas, p1: Point3<T>, p2: Point3<T>) {
    let p1i = p1.cast::<i32>().unwrap();
    let p2i = p2.cast::<i32>().unwrap();
    canvas.draw_line((p1i.x, p1i.y), (p2i.x, p2i.y)).unwrap();
}

fn draw_models(canvas: &mut WindowCanvas, transform: Matrix4<f32>, models: &Vec<Model>) {
    for model in models.iter() {
        for (i0, i1, i2) in model.mesh.indices.iter().tuples() {
            let p0 = transform.transform_point(model.mesh.get_position(*i0).unwrap());
            let p1 = transform.transform_point(model.mesh.get_position(*i1).unwrap());
            let p2 = transform.transform_point(model.mesh.get_position(*i2).unwrap());
            draw_line(canvas, p0, p1);
            draw_line(canvas, p1, p2);
            draw_line(canvas, p2, p0);
        }
    }
}

fn render(canvas: &mut WindowCanvas, models: &Vec<Model>, time: f32) {
    // let rotate_y = Matrix4::from_angle_y(Deg(time * 90.0));
    // let rotate_x = Matrix4::from_angle_x(Deg(time * 45.0));
    let scale = Matrix4::from_scale(100.0);
    let rotate_z = Matrix4::from_angle_z(Deg(180.0));
    let rotate_y = Matrix4::from_angle_y(Deg(90.0 * time));
    let translate_back = Matrix4::from_translation(Vector3::new(1024.0 / 2.0, 768.0 / 2.0, 0.0));
    let transform = translate_back * rotate_y * rotate_z * scale;
    draw_models(canvas, transform, models);
}
