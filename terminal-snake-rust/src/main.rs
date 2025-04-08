use ruscii::app::{App, State};
use ruscii::terminal::{Color, Window};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};


fn main() {
    let mut fps_counter = FPSCounter::default();
    let mut key_pressed= Key::Unknown;
    let mut app = App::default();
    let win_size = app.window().size();
    let boarder_chars: RectCharset = RectCharset::simple_lines();

    let mut score: u32 = 11;


    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => key_pressed = key_event.pressed().unwrap_or(key_event.released().unwrap_or(Key::Unknown)),
            }
        }

        fps_counter.update();

        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(0, 1));
        pencil.draw_text(&format!("Key Pressed: {:?}", key_pressed), Vec2::xy(0, 2));
        pencil.draw_text(&format!("Score: {:0>4}", score), Vec2::xy(win_size.x - 11, 1));
        pencil.draw_text("Press q or esc to quit", Vec2::xy(win_size.x - 22, 2));
        pencil.draw_rect(
            &boarder_chars,
            Vec2::xy(0, 3),
            Vec2::xy(win_size.x, win_size.y - 3)
        );
        pencil.set_background(Color::White).draw_text("o", Vec2::xy(win_size.x/2, win_size.y/2));
    });
}