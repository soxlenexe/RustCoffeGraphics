use coffee::graphics::{Color, Frame, Mesh,Rectangle, Shape};

/*modulos propios*/
#[path = "entity.rs"]
mod entity;
use entity::Entity;

pub struct Block{
    pub width:f32,
    pub height:f32,
    pub x:f32,
    pub y:f32,
    // pub middle:Vec<f32>,
    pub solid: bool,
    pub tag: String

}

impl Block{
    pub fn new(xx:f32,yy:f32)->Block{
        Block{
            width:100.0,
            height:100.0,
            x:xx,
            y:yy,
            solid:true,
            tag:String::new(),
        }
    }
}




pub struct Level{
    pub tile_map:Vec<Block>,
    // cam_x:f32,
    // cam_y:f32,
    // canvas:Vec,
    pub player:Entity,
    pub controls: [bool;4],

}

impl Level{
    pub fn new()->Level{
        Level{
            tile_map: Vec::new(),
            player: Entity::new(),
            controls: [false,false,false,false],
        }
    }

    pub fn add_from_text(&mut self,text:&String){
        let mut sub_x: f32 = 0.0;
        let mut sub_y: f32 = 0.0;
        for c in text.chars(){
            if c == '='{
                let mut temp_block = Block::new(sub_x ,sub_y );
                temp_block.tag = String::from("pared");
                let mut temp_bloc = Block::new(sub_x ,sub_y );
                temp_bloc.tag = String::from("piso");
                temp_bloc.height = 25.0;

                self.tile_map.push(temp_block);
                self.tile_map.push(temp_bloc);
                sub_x += 100.0;
            }
            if c == '+'{
                let mut temp_block = Block::new(sub_x ,sub_y );
                temp_block.tag = String::from("pared");
                self.tile_map.push(temp_block);
                sub_x += 100.0;
            }
            if c == '#'{
                let mut temp_block = Block::new(sub_x ,sub_y );
                temp_block.tag = String::from("piso");
                self.tile_map.push(temp_block);
                sub_x += 100.0;
            }
            if c == ' '{
                
                let mut temp_block = Block::new(sub_x ,sub_y );
                temp_block.solid = false;
                self.tile_map.push(temp_block);
                sub_x += 100.0;
            }
            if c == '\n'{
                sub_x = 0.0;
                sub_y += 100.0;
            }
            if c == '0'{
                self.player.position.x = sub_x;
                self.player.position.y = sub_y;
            }
            
        }
    }

    pub fn controls(&mut self, jump:bool,left:bool,right:bool){

        if self.player.piso == true && jump == true{
            self.player.velocity.y = self.player.salto;
            self.player.piso = false;
        }
        if left == true{
            self.player.velocity.x =  - self.player.aceleration_x;
            
        }

        if right == true{
            self.player.velocity.x =  self.player.aceleration_x;
            
        }

    }

    pub fn update(&mut self){
        self.update_player_phisics();
    }

    fn update_player_phisics(&mut self){

        self.player.acc.y = self.player.gravity.y;
        self.player.acc.x = 0.0;

        self.player.acc.x += self.player.velocity.x * self.player.friction;
        self.player.velocity.x += self.player.acc.x;
        self.player.velocity.y += self.player.acc.y;
        
        
        self.player.position.y += self.player.velocity.y + 0.5 * self.player.acc.y;
        self.player.position.x += self.player.velocity.x + 0.5 * self.player.acc.x;

        self.do_collisions();
    }

    fn do_collisions(&mut self){
        let player_top = self.player.position.y;
        let player_bottom = self.player.position.y + self.player.height;
        let player_left = self.player.position.x;
        let player_right = self.player.position.x + self.player.width;
        let player_center_x = player_right / 2.0;
        let player_center_y = player_bottom / 2.0;

        let tiles:_ = &self.tile_map;

        for tile in tiles{

            if  self.player.position.x < tile.x + tile.width &&
                self.player.position.x + self.player.width > tile.x &&
                self.player.position.y < tile.y + tile.height &&
                self.player.height + self.player.position.y > tile.y &&
                tile.solid == true && tile.tag == "pared"
            {
                 
                    if self.player.position.x + self.player.width <= tile.x + tile.width/2.0 {
                        self.player.position.x = tile.x - self.player.width;
                        self.player.velocity.x = 0.0;
                    }
                    if self.player.position.x  > tile.x + tile.width/2.0{
                        self.player.position.x = tile.x + tile.width;
                        self.player.velocity.x = 0.0;
                    }


             }







            // if player_center_y >= tile.y && player_center_y <= tile.y + tile.height && 
            //     player_center_x <= tile.x && player_right >=  tile.x && tile.solid == true
            // {
            //     self.player.position.x = tile.x - self.player.width;
            // }

            // if player_center_y >= tile.y && player_center_y <= tile.y + tile.height && 
            //     player_center_x >= tile.x && player_left <=  tile.x +tile.width && tile.solid == true
            // {
            //     self.player.position.x = tile.x +tile.width;

            // }


            if player_right > tile.x && player_left < tile.x + tile.width && tile.solid == true && tile.tag == "piso"{
            
                if player_bottom >= tile.y && player_bottom <= tile.y + tile.height/2.0{
                    self.player.velocity.y = 0.0;
                    self.player.position.y = tile.y - self.player.height;
                    self.player.piso = true;
                }

                // if player_top >= tile.y + tile.width a&&  bottom <= r_top  {
                //     if player_right <= (tile.x + (tile.width /2.0)){
                //         self.player.position.x = tile.x;
                //     }
                // }
            }
        }
    }

}