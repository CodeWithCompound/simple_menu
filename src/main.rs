use macroquad::prelude::*;
enum CurrentState {
    Game,
    MainMenu,
    Settings,
}

fn game_msg(text: &str, position: Vec2, font_size: f32) {
    draw_text(text, position.x, position.y, font_size as f32, WHITE);
}

fn screen_outline(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle_lines(x, y, w, h, 10.0, BLACK);
}
fn button(x: f32, y: f32, w: f32, h: f32, label: &str) -> bool {
    let (mx, my) = mouse_position();

    draw_rectangle(x, y, w, h, GRAY);
    draw_rectangle_lines(x, y, w, h, 4.0, BLACK);
    draw_text(label, x + 10.0, y + h * 0.6, 30.0, BLACK);

    //hear me out, this might not look complex to someone who knows how to code, but THIS, the 3 lines below, pure horror the moment i had to make that
    //eventually it clicked and i felt a little dumb because despite how it looks it's actually quite simple and makes a lot of sense
    let inside_x = mx >= x && mx <= x + w;
    let inside_y = my >= y && my <= y + h;
    let hovered = inside_x && inside_y;

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

#[macroquad::main("i click button, i happy")]
async fn main() {
    let mut state = CurrentState::MainMenu;
    loop {
        clear_background(PURPLE);

        let screen = vec2(screen_width(), screen_height());
        let btn_size = vec2(200.0, 60.0);
        let outline_size = vec2(screen_width(), screen_height());

        // center position: screen/2 - size/2
        // we make the buttons right side be in the center, to center the button itself we move it to the left by half it's lenght, same for height...
        let btn_pos = vec2(
            screen.x / 2.0 - btn_size.x / 2.0,
            screen.y / 2.0 - btn_size.y / 2.0,
        );
        let outline_pos = vec2(
            screen.x / 2.0 - outline_size.x / 2.0,
            screen.y / 2.0 - outline_size.y / 2.0,
        );
        screen_outline(outline_pos.x, outline_pos.y, screen.x, screen.y);

        match state {
            CurrentState::MainMenu => {
                let v_text = "v 0.0.2";
                let v_text_dims = measure_text(v_text, None, 20, 1.0);
                let game_text_pos = vec2(
                    10.0,
                    v_text_dims.height * 3.0,
                );

                 game_msg(v_text, game_text_pos, 40.0);
                
                if button(btn_pos.x, btn_pos.y, btn_size.x, btn_size.y, "START") {
                    println!("game started!");
                    state = CurrentState::Game;
                }
            }
            CurrentState::Game => {
                let game_text = "pretend heres a game okay?";
                let game_text_dims = measure_text(game_text, None, 40, 1.0);
                let game_text_pos = vec2(
                    screen_width() / 2.0 - game_text_dims.width / 2.0,
                    game_text_dims.height + 10.0,
                );
                game_msg(game_text, game_text_pos, 40.0);
                if button(btn_pos.x, btn_pos.y, btn_size.x, btn_size.y, "Settings") {
                    state = CurrentState::Settings;
                }
            }
            CurrentState::Settings => {
                let settings_text = "change your settings as you desire! (SETTINGS SOON!!)";
                let txt_dims = measure_text(settings_text, None, 30, 1.0);
                let settings_text_pos: Vec2 = vec2(
                    screen_width() / 2.0 - txt_dims.width / 2.0,
                    screen_height() - txt_dims.height / 2.0,
                );
                game_msg(settings_text, settings_text_pos, 30.0);
            } /*
              let txt_dims = measure_text(text, None, font_size, 1.0);
              let position_text = vec2(screen_width() / 2.0 - txt_dims.width / 2.0,
                screen_height() / 2.0 - txt_dims.height / 2.0);
                // this is for centering the text
              */
        }

        next_frame().await;
    }
}
