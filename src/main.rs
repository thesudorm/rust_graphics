extern crate rand;

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
    pieces: Vec<Shape>
}

const BOARD_X_POS: f32 = 150.0;
const BOARD_Y_POS: f32 = 50.0;
const BOARD_WIDTH: f32 = 500.0;

const BOARD_COLOR: ggez::graphics::Color = ggez::graphics::Color::new(0.05, 0.46, 0.14, 1.0);

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const TARGET_FPS: u32 = 30;

        while timer::check_update_time(ctx, TARGET_FPS){
            self.dt = timer::delta(ctx);
        }

        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, mb: ggez::event::MouseButton, x: f32, y: f32){
        if mb == ggez::event::MouseButton::Left{
            println!("{}, {}", x, y);
            self.pieces.push(make_circle(x, y));
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Drawing shapes from state
        {
            // draw board
            let board = graphics::Rect::new(BOARD_X_POS, BOARD_Y_POS, BOARD_WIDTH, BOARD_WIDTH);
            let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), board, BOARD_COLOR)?;
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

            for x in 1..8 { // Horizontal lines
                let mut points = Vec::new();
                points.push(mint::Point2{
                    x: BOARD_X_POS,
                    y: BOARD_Y_POS + (BOARD_WIDTH / 8.0) * x as f32,
                });
                points.push(mint::Point2{
                    x: BOARD_X_POS + (BOARD_WIDTH / 8.0) * x as f32 + BOARD_WIDTH,
                    y: BOARD_Y_POS + (BOARD_WIDTH / 8.0) * x as f32,
                });
                let line = graphics::Mesh::new_line(ctx, &points, 3.0, graphics::BLACK)?;
                graphics::draw(ctx, &line, graphics::DrawParam::default())?;
            }

            for x in 1..8 { // Vertical lines
                let mut points = Vec::new();
                points.push(mint::Point2{
                    x: BOARD_X_POS + (BOARD_WIDTH / 8.0) * x as f32,
                    y: BOARD_Y_POS
                });
                points.push(mint::Point2{
                    x: BOARD_X_POS + (BOARD_WIDTH / 8.0) * x as f32,
                    y: BOARD_WIDTH + BOARD_Y_POS
                });
                let line = graphics::Mesh::new_line(ctx, &points, 3.0, graphics::BLACK)?;
                graphics::draw(ctx, &line, graphics::DrawParam::default())?;
            }

            for piece in &self.pieces {
                let mesh = match piece {
                    &Shape::Rectangle(rect) => {
                        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?
                    }
                    &Shape::Circle(origin, radius) => {
                        graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), origin, radius, 0.1, graphics::BLACK)?
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
        pieces: Vec::new(),
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

fn make_circle(_x: f32, _y:f32) -> Shape {
    return Shape::Circle(
        mint::Point2{
            x: _x,
            y: _y
        },
        25.0);
}