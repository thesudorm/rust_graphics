extern crate rand;

use rand::{thread_rng, Rng};
use ggez::*;
use ggez::conf::*;

use nalgebra as na;

type Point2 = na::Point2<f32>;

enum Shape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}

struct State {
    dt: std::time::Duration,
    start: std::time::Instant,
    shapes: Vec<Shape>,
    colors: Vec<graphics::Color>
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const TARGET_FPS: u32 = 30;

        while timer::check_update_time(ctx, TARGET_FPS){
            self.dt = timer::delta(ctx);
        }

        if self.start.elapsed().as_secs() >= 3 {
            self.start = std::time::Instant::now();
            self.shapes = Vec::new();

            self.shapes = generate_random_shapes();
            self.colors = generate_random_colors();
        }

        Ok(())
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, mb: ggez::event::MouseButton, x: f32, y: f32){
        if mb == ggez::event::MouseButton::Left{
            self.shapes = generate_random_shapes();
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Drawing shapes from state
        {
            for (shape, color) in self.shapes.iter().zip(self.colors.iter()) {
                let mesh = match shape {
                    &Shape::Rectangle(rect) => {
                        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color.clone())?
                    }
                    &Shape::Circle(origin, radius) => {
                        graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), origin, radius, 0.1, color.clone())?
                    }
                };

                graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
            }

        }

        // GUI
        {
            let fps_point = Point2::new(10.0, 10.0);
            let fps_display = graphics::Text::new(timer::fps(ctx).to_string());

            graphics::draw(ctx, &fps_display, (fps_point, 0.0, graphics::WHITE))?;
        }

        graphics::present(ctx)?;
        timer::yield_now();

        Ok(())
    }
}

fn main() {

    let state = &mut State { 
        dt: std::time::Duration::new(0,0),
        start: std::time::Instant::now(),
        shapes: generate_random_shapes(),
        colors: generate_random_colors()
    };

    let ws = WindowSetup {
        title: "Demo".to_owned(),
        vsync: true,
        samples: NumSamples::Zero,
        icon: "".to_owned(),
        srgb: true
    };

    let c = conf::Conf {
        window_mode: WindowMode::default(),
        window_setup: ws,
        backend: Backend::default(),
        modules: ModuleConf::default()
    };

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "Joseph")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}

fn generate_random_shapes() -> Vec<Shape> {
    let mut shapes = Vec::new();

    for _ in 0..8 {
        if thread_rng().gen_range(0,2) % 2 == 0 {
            shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(
                thread_rng().gen_range(0.0, 800.0),
                thread_rng().gen_range(0.0, 600.0),
                thread_rng().gen_range(0.0, 800.0),
                thread_rng().gen_range(0.0, 600.0),
            )));
        } else {
            shapes.push(Shape::Circle(
                mint::Point2{
                    x: thread_rng().gen_range(0.0, 800.0),
                    y: thread_rng().gen_range(0.0, 800.0)
                },
                thread_rng().gen_range(0.0, 300.0)
            ));
        }
    }

    return shapes;
}

fn make_circle(_x: f32, _y:f32) -> Shape {
    return Shape::Circle(
        mint::Point2{
            x: _x,
            y: _y
        },
        50.0);
}

fn generate_random_colors() -> Vec<graphics::Color> {

    let mut colors = Vec::new();

    for _ in 0..8 {
        colors.push(graphics::Color::from_rgb(
            thread_rng().gen_range(0,255),
            thread_rng().gen_range(0,255),
            thread_rng().gen_range(0,255)));
    }

    return colors;
}