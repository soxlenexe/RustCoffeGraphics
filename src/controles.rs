use coffee::input::{self, keyboard, Input};

pub struct Control {
    pub up:bool,
    pub down:bool,
    pub left:bool,
    pub right:bool
}

impl Input for Control {
    fn new() -> Control {
        Control {
            up:false,
            down:false,
            left:false,
            right:false
        }
    }

    fn update(&mut self, event: input::Event) {
        

        match event {
            input::Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::TextEntered {character:_}=> {},
                keyboard::Event::Input { key_code, state } => match state {
                    input::ButtonState::Pressed => {
                        if key_code == input::keyboard::KeyCode::A{
                            self.left = true;
                            
                        }
                        if key_code == input::keyboard::KeyCode::D{
                            self.right = true;
                        }
                        if key_code == input::keyboard::KeyCode::W{
                            self.up = true;
                        }
                        if key_code == input::keyboard::KeyCode::S{
                            self.down = true;
                        }
                    }
                    input::ButtonState::Released => {
                        if key_code == input::keyboard::KeyCode::A{
                            self.left = false;
                        }
                        if key_code == input::keyboard::KeyCode::D{
                            self.right = false;
                        }
                        if key_code == input::keyboard::KeyCode::W{
                            self.up = false;
                        }
                        if key_code == input::keyboard::KeyCode::S{
                            self.down = false;
                        }
                    }
                },
            },
            _ => {}
        }
    }

    fn clear(&mut self){

    }


}


