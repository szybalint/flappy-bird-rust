use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

struct Bird {
    xpos: i32,
    ypos: i32,
    speed: i32,
    accel: i32,
    jumpspeed: i32,
}

impl Bird {
    fn jump(&mut self) {
        self.speed = self.jumpspeed;
    }

    fn update(&mut self) {
        self.speed += self.accel;
        self.ypos += self.speed;
    }
}

fn main() {
    let mut engine = console_engine::ConsoleEngine::init_fill(60);
    loop {
        engine.wait_frame();
        engine.clear_screen();

        for y in 0..engine.get_height() {
            for x in 0..engine.get_width() {
                engine.set_pxl(
                    x as i32,
                    y as i32,
                    pixel::pxl_fbg(' ', Color::Green, Color::Green),
                );
            }
        }

        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        engine.draw();
    }
}
