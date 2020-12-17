extern crate rand;

use ggez::*;
use ggez::conf::*;

use nalgebra as na;

type Point2 = na::Point2<f32>;

enum Shape {
    Circle(mint::Point2<f32>, f32)
}

struct State {
    dt: std::time::Duration,
    meshes: Vec<Shape>,
    pieces: [[i32; 8] ; 8],
    colors: Vec<ggez::graphics::Color>,
    is_player1_turn: bool
}

const BOARD_X_POS: f32 = 150.0;
const BOARD_Y_POS: f32 = 50.0;
const BOARD_WIDTH: f32 = 500.0;
const BOARD_COL_WIDTH: f32 = BOARD_WIDTH / 8.0;

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
            if x >= BOARD_X_POS && x <= BOARD_X_POS + BOARD_WIDTH && y >= BOARD_Y_POS && y <= BOARD_Y_POS + BOARD_WIDTH{
                println!("IN BOARD!");
                let mut y_pos = BOARD_Y_POS;
                let mut x_pos = BOARD_X_POS;
                //let mut piece = 0;

                for col in 0..8 {
                    for row in 0..8 {
                        if x >= x_pos && x <= x_pos + BOARD_COL_WIDTH && y >= y_pos && y <= y_pos + BOARD_COL_WIDTH {
                            let result = is_move_legal(self.pieces, self.is_player1_turn, row, col);
                            if result {
                                println!("Move LEGAL");
                                place_piece(self, row, col);
                            } else {
                                println!("Move ILLEGAL");
                            }
                        }
                        //piece += 1;
                        x_pos += BOARD_COL_WIDTH;
                    }
                    x_pos = BOARD_X_POS;
                    y_pos += BOARD_COL_WIDTH;
                }
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        {
            // draw board
            let board = graphics::Rect::new(BOARD_X_POS, BOARD_Y_POS, BOARD_WIDTH, BOARD_WIDTH);
            let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), board, BOARD_COLOR)?;
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

            for x in 1..8 { // Horizontal lines
                let mut points = Vec::new();
                points.push(mint::Point2{
                    x: BOARD_X_POS,
                    y: BOARD_Y_POS + BOARD_COL_WIDTH * x as f32,
                });
                points.push(mint::Point2{
                    x: BOARD_X_POS + BOARD_COL_WIDTH * x as f32 + BOARD_WIDTH,
                    y: BOARD_Y_POS + BOARD_COL_WIDTH * x as f32,
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

            let to_draw = convert_board_to_shapes(self.pieces);

            for (shape, color) in to_draw {
                let mesh = match shape {
                    Shape::Circle(origin, radius) => {
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

            // not sure why, but this is crashing..
            // graphics::draw(ctx, &fps_display, (fps_point, 0.0, graphics::WHITE))?;
        }

        graphics::present(ctx)?;
        timer::yield_now();

        Ok(())
    }
}

fn main() {

    let state = &mut State { 
        dt: std::time::Duration::new(0,0),
        meshes: Vec::new(),
        colors: Vec::new(),
        pieces: [[0;8];8],
        is_player1_turn: true
    };

    // Initialize the board
    place_piece(state, 4, 3);
    place_piece(state, 3, 3);
    place_piece(state, 3, 4);
    place_piece(state, 4, 4);

    let ws = WindowSetup {
        title: "Othello".to_owned(),
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

    // Currently there is a bug in winit that causes this to fail.
    // Can either attempt to run off of devel branch of ggez or downgrade rust to 1.47
    event::run(ctx, event_loop, state).unwrap();
}

fn debug_print_board(board: [[i32;8];8]) {
    for x in board.iter() {
        for y in x {
            print!("{} ", y);
        }
        println!();
    }

    println!();
}

// Adds piece to pieces array 
fn place_piece(state: &mut State, row: usize, col: usize){
    if state.is_player1_turn {
        state.pieces[col][row] = 1;
        state.colors.push(graphics::WHITE);
    } else {
        state.pieces[col][row] = 2;
        state.colors.push(graphics::BLACK);
    }

    state.is_player1_turn = !state.is_player1_turn;

    debug_print_board(state.pieces);
}

fn convert_board_to_shapes(board: [[i32;8];8]) -> Vec<(Shape, ggez::graphics::Color)>{
    let mut to_return = Vec::new();
    let mut color = ggez::graphics::WHITE;

    for col in 0..8 {
        for row in 0..8 {
            if board[col][row] != 0 { // If there is a piece
                if board[col][row] == 1 {
                    color = ggez::graphics::WHITE;
                } else {
                    color = ggez::graphics::BLACK;
                }

                to_return.push((Shape::Circle(
                    mint::Point2{
                        x: BOARD_X_POS + BOARD_COL_WIDTH * row as f32 + BOARD_COL_WIDTH / 2.0,
                        y: BOARD_Y_POS + BOARD_COL_WIDTH * col as f32 + BOARD_COL_WIDTH / 2.0
                    },
                    25.0), color));
            }
        }
    }

    return to_return;
}

fn is_move_legal(board: [[i32;8];8], is_player1_turn: bool, row: usize, col: usize) -> bool {
    let piece_to_place = match is_player1_turn {
        true => 1,
        false => 2
    };

    let mut i = 1;

    if board[col][row] == 0 {
        return check_direction(board, row as i32, col as i32, piece_to_place, 1, 0) 
               || check_direction(board, row as i32, col as i32, piece_to_place, -1, 0)
               || check_direction(board, row as i32, col as i32, piece_to_place, 0, 1)
               || check_direction(board, row as i32, col as i32, piece_to_place, 0, -1);
    }

    return false;
}

fn check_direction(board: [[i32;8];8], row_to_place: i32, col_to_place: i32, piece_to_place: i32, col_direction: i32, row_direction: i32) -> bool {

    let col_step = col_direction;
    let row_step = row_direction;

    let mut found = false;

    let mut col_index = col_step.clone();
    let mut row_index = row_step.clone();

    // Do some initial checks that look at the pieces that are placed at the end of the board
    if col_to_place == 0 && col_step < 0 && row_step == 0 {  // while checking straight north, if we are at 0 we are the northmost spot checking north should pass
        return true;
    } else if col_to_place == 7 && col_step > 0 && row_step == 0 {
        return true;
    }

    if row_to_place == 0 && row_step < 0 {
        return true;
    } else if row_to_place == 7 && row_step > 0 {
        return true;
    }

    // If none of the edge cases are true, search through the pieces to determine legality
    while !found { 
        if col_to_place + col_index == 0 {
            found = true;
        }
        else if col_to_place as i32 + col_index >= 0
            && board[(col_to_place + col_index) as usize][(row_to_place + row_index) as usize] != piece_to_place
            && board[(col_to_place + col_index) as usize][(row_to_place + row_index) as usize] != 0 {
                col_index += col_step;
                row_index += row_step;
        } else {
            found = true;
        }
    }

    // the second condition below to double check that this isn't just the price right next to the piece to be placed. 
    if board[(col_to_place + col_index) as usize][(row_to_place + row_index) as usize] == piece_to_place && !(col_index == col_step && row_index == row_step) {
        return true;
    } else {
        return false
    }
}
