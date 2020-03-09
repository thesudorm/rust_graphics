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
    shapes: Vec<Shape>,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const TARGET_FPS: u32 = 60;

        while timer::check_update_time(ctx, TARGET_FPS){
            self.dt = timer::delta(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Drawing shapes from state
        {
            for shape in &self.shapes {
                let mesh = match shape {
                    &Shape::Rectangle(rect) => {
                        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?
                    }
                    &Shape::Circle(origin, radius) => {
                        graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), origin, radius, 0.1, graphics::WHITE)?
                    }
                };

                graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
            }

        }

        // Shapes
        {
            // My first shape
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                mint::Point2{x: 200.0, y: 300.0},
                100.0,
                0.1,
                graphics::WHITE
            )?;
            
            let rectangle = graphics::Mesh::new_rectangle(
                ctx, 
                graphics::DrawMode::fill(),
                graphics::Rect::new(500.0, 250.0, 200.0, 200.0),
                graphics::WHITE,
            )?;

            graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
            graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;

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
    let mut shapes = Vec::new();
    shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(
        10.0,
        10.0,
        50.0,
        100.0,
    )));

    shapes.push(Shape::Circle(
        mint::Point2{x: 400.0, y: 40.0},
        30.0
    ));

    let state = &mut State { 
        dt: std::time::Duration::new(0,0),
        shapes: shapes,
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
