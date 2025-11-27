use macroquad::prelude::*;
enum CurrentState {
    Game,
    MainMenu,
    Settings,
}
#[derive(Clone, Copy, Debug)]
enum BgColor {
    RED,
    GREEN,
    BLUE,
    PINK,
    PURPLE,
}
// okay the following 2? i couldn't explain, like i'm trying to be somewhat professional BUT WHO THOUGHT MAKING COLOR GO FROM A TO B COULD BE THIS HARD FFS
impl BgColor {
    fn next(self) -> BgColor {
        match self {
            BgColor::RED => BgColor::GREEN,
            BgColor::GREEN => BgColor::BLUE,
            BgColor::BLUE => BgColor::PINK,
            BgColor::PINK => BgColor::PURPLE,
            BgColor::PURPLE => BgColor::RED,
        }
    }
}
impl BgColor {
    fn to_color(self) -> Color {
        match self {
            BgColor::RED => RED,
            BgColor::GREEN => GREEN,
            BgColor::BLUE => BLUE,
            BgColor::PINK => PINK,
            BgColor::PURPLE => PURPLE,
        }
    }
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
fn dev_mode_display(dev_mode: bool, mouse: (f32, f32)) {
    // Pick the text based on dev_mode, and optionally print position
    let _dev_text = if dev_mode {
            draw_text(
        &format!("x: {} | y: {}", mouse.0, mouse.1),
        screen_width() - 150.0,
        40.0,
        20.0,
        BLACK,
    ); 
        
        "dev mode"
    } else {
        "no dev mode"
    };

}

#[macroquad::main("i click button, i happy")]
async fn main() {
    let version = "0.2.2";
    let mut dev_mode = false;
    let mut state = CurrentState::MainMenu;
    let mut current_color: BgColor = BgColor::PURPLE;
    loop {
        clear_background(current_color.to_color());

        let screen = vec2(screen_width(), screen_height());
        let btn_size = vec2(200.0, 60.0);
        let outline_size = vec2(screen_width(), screen_height());
        let mouse: (f32, f32) = mouse_position();
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
        dev_mode_display(dev_mode, mouse);
        match state {
            CurrentState::MainMenu => {
                let v_text = version;
                let v_text_dims = measure_text(v_text, None, 20, 1.0);
                let game_text_pos = vec2(10.0, v_text_dims.height * 3.0);

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
                    screen_height() / 2.0 - game_text_dims.height / 2.0,
                );
                game_msg(game_text, game_text_pos, 40.0);
                if button(10.0, 10.0, btn_size.x, btn_size.y, "Settings") {
                    state = CurrentState::Settings;
                }
            }
            CurrentState::Settings => {
                let settings_text = "change your settings as you desire! (SETTINGS SOON!!)";
                let settings_text_dims = measure_text(settings_text, None, 30, 1.0);
                let settings_text_pos: Vec2 = vec2(
                    screen_width() / 2.0 - settings_text_dims.width / 2.0,
                    screen_height() - settings_text_dims.height / 2.0,
                );
                game_msg(settings_text, settings_text_pos, 30.0);
                // for v 0.2.3 add dev mode

                let color_btn_size = vec2(200.0, 50.0);
                if button(
                    screen_width() / 2.0 - btn_size.x / 2.0,
                    125.0,
                    color_btn_size.x,
                    color_btn_size.y,
                     &format!("Dev: {}", dev_mode),
                ) {
                    dev_mode = !dev_mode;
                }
                if button(
                    screen_width() / 2.0 - btn_size.x / 2.0,
                    50.0,
                    color_btn_size.x,
                    color_btn_size.y,
                    "Choose color",
                ) {
                    current_color = current_color.next();
                }

                // go back to main menu
                if button(10.0, 10.0, btn_size.x, btn_size.y, "Back to game") {
                    state = CurrentState::Game;
                }
            }
        }
        next_frame().await;
    }
}
