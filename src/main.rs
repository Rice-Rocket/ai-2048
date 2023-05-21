use macroquad::prelude::*;
use std::cmp::min;


#[derive(PartialEq)]
enum GameState {
    AILoseMenu,
    AIPlaying,
    Playing,
    LoseMenu
}





#[derive(Clone)]
#[derive(Copy)]
struct Tile {
    row: i8,
    col: i8,
    val: i8,
    color: Color,
}


impl Tile {
    fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            val: 0,
            color: Color::new(0.8, 0.75, 0.7, 1.0)
        }
    }
    fn set_pos(&mut self, row: i8, col: i8) {
        self.row = row;
        self.col = col;
    }
    fn update_color(&mut self) {
        self.color = match self.val {
            0 => Color::new(0.8, 0.75, 0.7, 1.0),
            1 => Color::new(0.93, 0.89, 0.86, 1.0),
            2 => Color::new(0.94, 0.89, 0.79, 1.0),
            3 => Color::new(0.95, 0.69, 0.47, 1.0),
            4 => Color::new(0.93, 0.55, 0.33, 1.0),
            5 => Color::new(0.98, 0.48, 0.36, 1.0),
            6 => Color::new(0.92, 0.35, 0.22, 1.0),
            7 => Color::new(0.93, 0.81, 0.45, 1.0),
            8 => Color::new(0.95, 0.82, 0.28, 1.0),
            9 => Color::new(0.93, 0.78, 0.31, 1.0),
            10 => Color::new(0.89, 0.73, 0.07, 1.0),
            11 => Color::new(0.93, 0.77, 0.01, 1.0),
            12 => Color::new(0.38, 0.85, 0.57, 1.0),
            _ => Color::new(0.8, 0.75, 0.7, 1.0)
        };
    }
    fn set_value(&mut self, value: i8) {
        self.val = value;
        self.update_color();
    }
    fn draw(&mut self, font: Font) {
        let size = min(screen_width() as i128, screen_height() as i128) as f32;
        let posx = self.col as f32 * (size / 4f32) + 10f32;
        let posy = self.row as f32 * (size / 4f32) + 10f32;
        let tilesize = size / 4f32 - 20f32;
        draw_rectangle(posx, posy, tilesize, tilesize, self.color);
        let color = match self.val {
            0..=2 => Color::new(0.46, 0.43, 0.40, 1.0),
            _ => Color::new(0.97, 0.96, 0.94, 1.0)
        };

        if self.val > 0 {
            let text = format!("{}", 2i32.pow(self.val as u32));
            let dims = measure_text(&text, Some(font), 50u16, 1.0);
            draw_text_ex(
                &text,
                posx + (tilesize / 2f32 - dims.width / 2f32),
                posy + (tilesize / 2f32 + dims.height / 2f32),
                TextParams{font, font_size: 50u16, color: color, ..Default::default()}
            );
        };
    }
    fn reset(&mut self) {
        self.val = 0;
        self.update_color();
    }
}



fn spawn_tile(tiles: &mut [[Tile; 4]; 4]) {
    let mut rand_row: usize = rand::gen_range(0, 4);
    let mut rand_col: usize = rand::gen_range(0, 4);
    
    if rand_row > 3 {
        rand_row = 3;
    }
    if rand_col > 3 {
        rand_col = 3;
    }

    if tiles[rand_row][rand_col].val != 0 {
        return spawn_tile(tiles);
    }

    let roll_4 = rand::gen_range(0, 10);
    if roll_4 == 0 {
        tiles[rand_row][rand_col].set_value(2i8);
    }
    else {
        tiles[rand_row][rand_col].set_value(1i8);
    }
}


fn compress(tiles: &mut [[Tile; 4]; 4], move_log: &mut Vec<(i8, i8, i8)>) {
    let (board_rows, board_cols): (usize, usize) = (tiles.len() as usize, tiles[0].len() as usize);
    for i in 0..board_rows {
        for j in 0..board_cols {
            let mut col = usize::clone(&j);
            let mut success = false;
            while (tiles[i][col].val != 0) && (col != 0) && (tiles[i][col - 1].val == 0) {
                tiles[i][col - 1].val = tiles[i][col].val;
                tiles[i][col].val = 0;
                success = true;
                col -= 1;
            };
            match success {
                true => {
                    move_log.push((i as i8, col as i8, 0));
                },
                false => {}
            };
        };
    };
}



fn merge(tiles: &mut [[Tile; 4]; 4], move_log: &mut Vec<(i8, i8, i8)>) {
    let (board_rows, board_cols): (usize, usize) = (tiles.len() as usize, tiles[0].len() as usize);
    for i in 0..board_rows {
        for j in 0..board_cols {
            if (tiles[i][j].val != 0) && (j != 0) && (tiles[i][j - 1].val != 0) && (tiles[i][j].val == tiles[i][j - 1].val) {
                tiles[i][j - 1].val = tiles[i][j].val + 1;
                tiles[i][j].val = 0;
                move_log.push((i as i8, j as i8 - 1, 1));
            };
        };
    };
}



fn flip_x(tiles: &mut [[Tile; 4]; 4], move_log: &mut Vec<(i8, i8, i8)>) {
    let board_rows: usize = tiles.len() as usize;
    for i in 0..board_rows {
        (tiles[i][0], tiles[i][3]) = (tiles[i][3], tiles[i][0]);
        (tiles[i][1], tiles[i][2]) = (tiles[i][2], tiles[i][1]);
    };

    for m in move_log.iter_mut() {
        let new_pos = match m.1 {
            0 => 3,
            1 => 2,
            2 => 1,
            3 => 0,
            _ => 0
        };
        m.1 = new_pos;
    };
}



fn transpose(tiles: &mut [[Tile; 4]; 4], move_log: &mut Vec<(i8, i8, i8)>) {
    (tiles[0][1], tiles[1][0]) = (tiles[1][0], tiles[0][1]);
    (tiles[0][2], tiles[2][0]) = (tiles[2][0], tiles[0][2]);
    (tiles[0][3], tiles[3][0]) = (tiles[3][0], tiles[0][3]);
    (tiles[1][2], tiles[2][1]) = (tiles[2][1], tiles[1][2]);
    (tiles[1][3], tiles[3][1]) = (tiles[3][1], tiles[1][3]);
    (tiles[2][3], tiles[3][2]) = (tiles[3][2], tiles[2][3]);

    for m in move_log.iter_mut() {
        (m.0, m.1) = (m.1, m.0);
    }
}




fn execute_move(tiles: &mut [[Tile; 4]; 4], direction: u8) -> (i32, bool) {
    let mut score = 0i32;
    let mut move_log: Vec<(i8, i8, i8)> = Vec::new(); // stores positions that were moved to

    if direction == 0 { // left
        compress(tiles, &mut move_log);
        merge(tiles, &mut move_log);
        compress(tiles, &mut move_log)
    }
    else if direction == 1 { // right
        flip_x(tiles, &mut move_log);
        compress(tiles, &mut move_log);
        merge(tiles, &mut move_log);
        compress(tiles, &mut move_log);
        flip_x(tiles, &mut move_log)
    }
    else if direction == 2 { // up
        transpose(tiles, &mut move_log);
        compress(tiles, &mut move_log);
        merge(tiles, &mut move_log);
        compress(tiles, &mut move_log);
        transpose(tiles, &mut move_log)
    }
    else { // down
        transpose(tiles, &mut move_log);
        flip_x(tiles, &mut move_log);
        compress(tiles, &mut move_log);
        merge(tiles, &mut move_log);
        compress(tiles, &mut move_log);
        flip_x(tiles, &mut move_log);
        transpose(tiles, &mut move_log)
    }

    if move_log.len() > 0 {
        for m in move_log.iter() {
            if m.2 == 1 {
                score += 2i32.pow(tiles[m.0 as usize][m.1 as usize].val as u32);
            };
        };
    };
    
    for row in tiles.iter_mut() {
        for tile in row.iter_mut() {
            tile.update_color();
        }
    }
    return (score, move_log.len() > 0);
}


fn is_terminal(tiles: &mut [[Tile; 4]; 4]) -> bool {
    let mut temp_board = tiles.clone();
    for m in 0..4 {
        let (_, was_move) = execute_move(&mut temp_board, m);
        if was_move {
            return false;
        }
    };
    return true;
}



fn clear_board(tiles: &mut [[Tile; 4]; 4]) {
    for row in tiles.iter_mut() {
        for tile in row.iter_mut() {
            tile.reset();
        }
    }
}





fn mcts(tiles: &mut [[Tile; 4]; 4], search_length: i32, searches_per_move: i32) -> u8 {
    let first_moves = [0, 1, 2, 3];
    let mut scores = [0, 0, 0, 0];

    for first_idx in 0..4 {
        let first_move = first_moves[first_idx];
        let mut first_state = tiles.clone();
        let (first_score, was_moved) = execute_move(&mut first_state, first_move);

        if was_moved {
            spawn_tile(&mut first_state);
            scores[first_idx] += first_score;
        }
        else {
            scores[first_idx] = -1;
            continue;
        }

        for _ in 0..searches_per_move {
            let mut move_number = 1;
            let mut search_board = first_state.clone();
            let mut is_valid = true;

            while (is_valid) && (move_number < search_length) {
                let (score, was_move) = execute_move(&mut search_board, rand::gen_range(0, 4));

                if was_move {
                    spawn_tile(&mut search_board);
                    scores[first_idx] += score;
                    move_number += 1;
                }
                else {
                    is_valid = false;
                }
            }
        }
    }

    let mut best_score = -1;
    let mut best_idx: usize = 0;
    for (i, s) in scores.iter().enumerate() {
        if s > &best_score {
            best_score = *s;
            best_idx = i;
        }
    }
    return first_moves[best_idx];
}








#[macroquad::main("2048")]
async fn main() {

    let font = load_ttf_font("res/Monaco.ttf").await.unwrap();
    let mut total_score = 0i32;
    let mut game_state = GameState::AIPlaying;
    
    let mut tiles = [[Tile::new(); 4]; 4];
    for (i, row) in tiles.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            col.set_pos(i as i8, j as i8);
        };
    };
    spawn_tile(&mut tiles);
    spawn_tile(&mut tiles);
    
    let ai_move_delay = 0.0; // seconds
    let search_length: i32 = 3200;
    let searches_per_move: i32 = 4800;
    let mut last_move_time = 0f64;
    // let mut ai_queued_move: u8 = mcts(&mut tiles, search_length, searches_per_move);
    loop {
        clear_background(WHITE);

        match game_state {
            GameState::AIPlaying => {
                if ai_move_delay > get_time() - last_move_time {
                    continue;
                }
                
                let queued_move = mcts(&mut tiles, search_length, searches_per_move);
                let (score_gained, was_moved) = execute_move(&mut tiles, queued_move);
                last_move_time = get_time();

                if was_moved {
                    total_score += score_gained;
                    spawn_tile(&mut tiles);
                }
                
                for row in tiles.iter_mut() {
                    for col in row.iter_mut() {
                        col.draw(font);
                    };
                };
                if is_terminal(&mut tiles) {
                    game_state = GameState::AILoseMenu;
                };
            },
            GameState::Playing => {
                let mut was_moved = false;
                let mut score_gained = 0i32;
                if is_key_pressed(KeyCode::Left) {
                    (score_gained, was_moved) = execute_move(&mut tiles, 0);
                };
                if is_key_pressed(KeyCode::Right) {
                    (score_gained, was_moved) = execute_move(&mut tiles, 1);
                };
                if is_key_pressed(KeyCode::Up) {
                    (score_gained, was_moved) = execute_move(&mut tiles, 2);
                };
                if is_key_pressed(KeyCode::Down) {
                    (score_gained, was_moved) = execute_move(&mut tiles, 3);
                };
                if was_moved {
                    total_score += score_gained;
                    spawn_tile(&mut tiles);
                }
                
                for row in tiles.iter_mut() {
                    for col in row.iter_mut() {
                        col.draw(font);
                    };
                };
                if is_terminal(&mut tiles) {
                    game_state = GameState::LoseMenu;
                };
            },
            GameState::LoseMenu | GameState::AILoseMenu => {
                for row in tiles.iter_mut() {
                    for col in row.iter_mut() {
                        col.draw(font);
                    };
                };

                let text = format!("Game Over!");
                let dims = measure_text(&text, Some(font), 40u16, 1.0);
                draw_text_ex(
                    &text,
                    screen_width() / 2f32 - dims.width / 2f32,
                    screen_height() / 2f32 + dims.height / 2f32 - 50f32,
                    TextParams{font, font_size: 40u16, color: BLACK, ..Default::default()}
                );
                let text = format!("Score : {}", total_score);
                let dims = measure_text(&text, Some(font), 40u16, 1.0);
                draw_text_ex(
                    &text,
                    screen_width() / 2f32 - dims.width / 2f32,
                    screen_height() / 2f32 + dims.height / 2f32,
                    TextParams{font, font_size: 40u16, color: BLACK, ..Default::default()}
                );
                let text = format!("Press SPACE to restart");
                let dims = measure_text(&text, Some(font), 40u16, 1.0);
                draw_text_ex(
                    &text,
                    screen_width() / 2f32 - dims.width / 2f32,
                    screen_height() / 2f32 + dims.height / 2f32 + 50f32,
                    TextParams{font, font_size: 40u16, color: BLACK, ..Default::default()}
                );

                if is_key_pressed(KeyCode::Space) {
                    clear_board(&mut tiles);
                    spawn_tile(&mut tiles);
                    spawn_tile(&mut tiles);
                    total_score = 0i32;
                    if game_state == GameState::AILoseMenu {
                        game_state = GameState::AIPlaying;
                    }
                    else {
                        game_state = GameState::Playing;
                    }
                }
            }
        }
        next_frame().await
    };
}
