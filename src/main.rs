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

#[macroquad::main("Tic-Tac_Toe")]
async fn main() {
    let mut state = GameState { 
        board: Board::default(), 
        is_x_turn: true,
        is_over: false,
    };

    

    loop {
        clear_background(BLACK);
        if !state.is_over {

            let index_selected = get_keyboard_input();
            
            if let Some(index) = index_selected {
                let value = if state.is_x_turn {Cell::X} else {Cell::O};
                state.board.set_cell(index, value);
                state.is_x_turn = !state.is_x_turn;
            }
    
            draw_board(screen_width() / 2.0, screen_height() / 2.0, &state.board);
            
            state.is_over = state.board.get_winner().is_some(); 
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



