pub struct Vector{
    pub x:f32,
    pub y:f32,
}
impl Vector{
    pub fn new(xx:f32,yy:f32)->Vector{
        Vector{
            x:xx,
            y:yy
        }
    }
}


pub struct Entity{
    pub aceleration_x: f32,
    pub friction: f32,
    pub position: Vector,
    pub velocity: Vector,
    pub  acc: Vector,
    pub gravity: Vector,
    pub width: f32,
    pub height:f32,
    pub piso: bool,
    pub salto: f32,
    // pub moviendo:bool,
}

impl Entity{

    pub fn new() -> Entity{
        Entity{
            aceleration_x: 9.8,
            friction: -0.120,
            position: Vector::new(0.0,0.0),
            velocity: Vector::new(0.0,0.0),
            acc: Vector::new(0.0,0.0),
            gravity: Vector::new(0.0,1.9),   
            width: 25.0,
            height: 25.0,
            piso: false,
            salto: -30.0,
        }
    }


}