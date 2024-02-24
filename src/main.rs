use games::tic_tac_toe::GameState;
use macroquad::prelude::*;
use games::tic_tac_toe::board::{self, Board};
use games::tic_tac_toe::cell::{Cell, CellIndex};


const BOARD_SIZE: f32 = 100.0;
const CELL_SIZE: f32 = BOARD_SIZE / 1.5;
const HALF_CELL_SIZE: f32 = CELL_SIZE / 2.0;
const THREE_HALF_CELL_SIZE: f32 = HALF_CELL_SIZE * 3.0;
const THICKNESS: f32 = 3.5;
const RADIUS: f32 = CELL_SIZE * 0.5;
const NEGATIVE: f32 = -1.0;

#[macroquad::main("Tic-Tac_Toe")]
async fn main() {
    let mut state = GameState { 
        board: Board::default(), 
        is_x_turn: true,
        winner: None,
    };

    

    loop {
        clear_background(BLACK);
        if state.winner.is_none() {

            let index_selected = get_mouse_input();
            
            if let Some(index) = index_selected {
                let value = if state.is_x_turn {Cell::X} else {Cell::O};
                state.board.set_cell(index, value);
                state.is_x_turn = !state.is_x_turn;
            }
    
            draw_board(screen_width() / 2.0, screen_height() / 2.0, &state.board);
            
            state.winner = state.board.get_winner(); 
        } else {
            draw_game_over();
            if is_key_pressed(KeyCode::Space) {

            }
            if is_key_pressed(KeyCode::Escape) {
                break;
            } 
                   
        }
        

        next_frame().await
    }
    
}
fn calculate_cell_boundaries(board_center: (f32, f32)) -> [Rect; 9] {
    let top_left_position = ((board_center.0 - THREE_HALF_CELL_SIZE), (board_center.1 - HALF_CELL_SIZE));
    let left_position = ((board_center.0 - THREE_HALF_CELL_SIZE), (board_center.1 - HALF_CELL_SIZE));
    let top_right_position = ((board_center.0 - THREE_HALF_CELL_SIZE), (board_center.1 + HALF_CELL_SIZE));
    let top_position = ((board_center.0 - HALF_CELL_SIZE), (board_center.1 - THREE_HALF_CELL_SIZE));
    let center_position = ((board_center.0 - HALF_CELL_SIZE), (board_center.1 - HALF_CELL_SIZE));
    let bottom_position = ((board_center.0 - HALF_CELL_SIZE), (board_center.1 + HALF_CELL_SIZE));
    let bottom_left_position = ((board_center.0 + HALF_CELL_SIZE), (board_center.1 - THREE_HALF_CELL_SIZE));
    let right_position = ((board_center.0 + HALF_CELL_SIZE), (board_center.1 - HALF_CELL_SIZE));
    let bottom_right_position = ((board_center.0 + HALF_CELL_SIZE), (board_center.1 + HALF_CELL_SIZE));
    let boundaries = [
        Rect::new(top_left_position.0, top_left_position.1, CELL_SIZE, CELL_SIZE),
        Rect::new(left_position.0, left_position.1, CELL_SIZE, CELL_SIZE),
        Rect::new(bottom_left_position.0, bottom_left_position.1, CELL_SIZE, CELL_SIZE),
        Rect::new(top_position.0, top_position.1, CELL_SIZE, CELL_SIZE),
        Rect::new(center_position.0, center_position.1, CELL_SIZE, CELL_SIZE),
        Rect::new(bottom_position.0, bottom_position.1, CELL_SIZE, CELL_SIZE),
        Rect::new(top_right_position.0, top_right_position.1, CELL_SIZE, CELL_SIZE),
        Rect::new(right_position.0, right_position.1, CELL_SIZE, CELL_SIZE),
        Rect::new(bottom_right_position.0, bottom_right_position.1, CELL_SIZE, CELL_SIZE),
    ];
    for boundary in boundaries.iter() {
        draw_rectangle(boundary.x, boundary.y, boundary.w, boundary.h, RED)
    }
    return boundaries;
    
}

fn draw_board(x: f32, y: f32, board: &Board) {
    draw_grid(x, y);
    for (cell_index, cell) in board.iter_enumerated() {
        let (x, y) = calculate_cell_center(x, y, cell_index);
        match cell {
            Cell::Empty => (),
            Cell::O => draw_o(x, y),
            Cell::X => draw_x(x, y), 
        }
    }
}
fn calculate_cell_center(x: f32, y: f32, index: CellIndex) -> (f32, f32) {
    return match index {
        CellIndex::TOP_LEFT => (x - CELL_SIZE, y - CELL_SIZE),
        CellIndex::TOP_MIDDLE => (x - CELL_SIZE, y),
        CellIndex::TOP_RIGHT => (x - CELL_SIZE, y + CELL_SIZE),
        CellIndex::MIDDLE_LEFT => (x, y - CELL_SIZE),
        CellIndex::CENTER => (x, y),
        CellIndex::MIDDLE_RIGHT => (x, y + CELL_SIZE),
        CellIndex::BOTTOM_LEFT => (x + CELL_SIZE, y - CELL_SIZE),
        CellIndex::BOTTOM_MIDDLE => (x + CELL_SIZE, y),
        CellIndex::BOTTOM_RIGHT => (x + CELL_SIZE, y + CELL_SIZE),
        _ => panic!("This will never panic because of CellIndex will always be in bounds."),
    };
}
/// Draw grid centered at (`x`, `y`).
fn draw_grid(x: f32, y: f32) {
    let lines = [
        ((x - HALF_CELL_SIZE, y - THREE_HALF_CELL_SIZE),
         (x - HALF_CELL_SIZE, y + THREE_HALF_CELL_SIZE)),
        ((x + HALF_CELL_SIZE, y - THREE_HALF_CELL_SIZE),
         (x + HALF_CELL_SIZE, y + THREE_HALF_CELL_SIZE)),
        ((x + THREE_HALF_CELL_SIZE, y + HALF_CELL_SIZE),
         (x - THREE_HALF_CELL_SIZE, y + HALF_CELL_SIZE)),
        ((x + THREE_HALF_CELL_SIZE, y - HALF_CELL_SIZE),
         (x - THREE_HALF_CELL_SIZE, y - HALF_CELL_SIZE)),
    ];
    for ((x1, y1), (x2, y2)) in lines.into_iter() {
        draw_line(x1, y1, x2, y2, THICKNESS, WHITE);
    }
}
fn draw_o(x: f32, y: f32) {
    draw_circle(x, y, RADIUS, WHITE);
    draw_circle(x, y, RADIUS * 0.8, BLACK);
}
fn draw_x(x: f32, y: f32) {
    let lines = [
        ((x + HALF_CELL_SIZE, y + HALF_CELL_SIZE),
         (x - HALF_CELL_SIZE, y - HALF_CELL_SIZE)),
        ((x - HALF_CELL_SIZE, y + HALF_CELL_SIZE),
         (x + HALF_CELL_SIZE, y - HALF_CELL_SIZE)),
    ];
    for line in lines.into_iter() {
        draw_line(line.0.0, line.0.1, line.1.0, line.1.1, THICKNESS, WHITE);
    }
}
fn get_mouse_input() -> Option<CellIndex> {
    if is_mouse_button_pressed(MouseButton::Left) {
        for (index, cell_boundary) in calculate_cell_boundaries((screen_height() / 2.0, screen_height() / 2.0)).into_iter().enumerate() {
            if cell_boundary.contains(mouse_position().into()) {
                let cell_index = CellIndex::try_from(index).expect("Index will always be in bounds because calculate_cell_index always has an array of 9");
                println!("{:?}", cell_index);
                return Some(cell_index);
            }
        }
    }
    return None;
}
fn get_keyboard_input() -> Option<CellIndex> {
    return if is_key_pressed(KeyCode::Key1) {
        Some(CellIndex::TOP_LEFT)
    } else if is_key_pressed(KeyCode::Key2) {
        Some(CellIndex::TOP_MIDDLE)
    } else if is_key_pressed(KeyCode::Key3) {
        Some(CellIndex::TOP_RIGHT)
    } else if is_key_pressed(KeyCode::Key4) {
        Some(CellIndex::MIDDLE_LEFT)
    } else if is_key_pressed(KeyCode::Key5) {
        Some(CellIndex::CENTER)
    } else if is_key_pressed(KeyCode::Key6) {
        Some(CellIndex::MIDDLE_RIGHT)
    } else if is_key_pressed(KeyCode::Key7) {
        Some(CellIndex::BOTTOM_LEFT)
    } else if is_key_pressed(KeyCode::Key8) {
        Some(CellIndex::BOTTOM_MIDDLE)
    } else if is_key_pressed(KeyCode::Key9) {
        Some(CellIndex::BOTTOM_RIGHT)
    } else {
        None
    };
}
fn draw_game_over() {
    let text = "Press SPACE to PLAY AGAIN Press ESC to EXIT";
            let font_size = screen_width() * 0.05; 
            let text_width = measure_text(text, None, font_size as u16, 1.0).width;
            let text_position = ((screen_width() - text_width) / 2.0, screen_height() / 2.0);
            draw_text(text, text_position.0, text_position.1, font_size, WHITE);
}



