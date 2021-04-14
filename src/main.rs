use coffee::graphics::{
    Color, Frame, Mesh,Rectangle, Shape, Window, WindowSettings
};
use coffee::load::Task;
use coffee::{Game, Timer};

/*Importamos los modulos propios */
mod controles;
use controles::Control;
// mod entity;
// use entity::Entity;
mod world;
use world::Level;



fn main() -> coffee::Result<()> {

    Example::run(WindowSettings {
        title: String::from("Juego"),
        size: (1080, 720),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

struct Example{
    pub x: f32,
    pub y: f32,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub nivel: Level,

    

}

impl Game for Example {
    
    type Input = Control;
    type LoadingScreen = ();
    const TICKS_PER_SECOND:u16=120 as u16;
    fn load(_window: &Window) -> Task<Example> {
        let mut niv = Level::new();
        niv.add_from_text(&String::from("#\n0\n\n\n   =\n   +\n#######"));
        Task::succeed(|| Example{
            up:false,
            down:false,
            left:false,
            right:false,
            x: 0.0,
            y: 0.0,
            nivel: niv,           
        })
    }
    
    fn interact(&mut self, input: &mut Control, _window: &mut Window) {
        self.down = input.down;
        self.up = input.up;
        self.right = input.right;
        self.left = input.left;

    }

    fn update(&mut self, _window: &Window){

        self.nivel.controls(self.up,self.left,self.right);
        
        // self.player.update();
        self.nivel.update();
    }

    fn draw(&mut self,frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::WHITE);
        let a:_ = &self.nivel.tile_map;
        for e in a{
            let mut mesh = Mesh::new();
            let mut color = Color::BLACK;
            if e.solid == false{
                color = Color::new(1.0,1.0,1.0,0.0);
            }
            mesh.fill(
                Shape::Rectangle(Rectangle{
                    x: e.x,
                    y: e.y,
                    width: e.width,
                    height: e.height
                }),
                color,
            );
            mesh.draw(&mut frame.as_target());
        }

        let mut mesh = Mesh::new();
        mesh.fill(
            Shape::Rectangle(Rectangle{
                x: self.nivel.player.position.x,
                y: self.nivel.player.position.y,
                width: self.nivel.player.width,
                height: self.nivel.player.height
            }),
            Color::RED,
        );
        mesh.draw(&mut frame.as_target());
    }
}

