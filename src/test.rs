
use crate::{
    math::{vec2, Rect, Vec2},
    time::get_frame_time,
};

#[derive(Clone, Debug)]
pub struct Animation {
    pub name: String,
    pub row: u32,
    pub frames: u32,
    pub fps: u32,
}

pub struct AnimationFrame {
    pub source_rect: Rect,
    pub dest_size: Vec2,
}

#[derive(Clone)]
pub struct AnimatedSprite {
    tile_width: f32,
    tile_height: f32,
    animations: Vec<Animation>,

    current_animation: usize,
    time: f32,
    frame: u32,
    pub playing: bool,
}

impl AnimatedSprite {
    pub fn new(
        tile_width: u32,
        tile_height: u32,
        animations: &[Animation],
        playing: bool,
    ) -> AnimatedSprite {
        AnimatedSprite {
            tile_width: tile_width as f32,
            tile_height: tile_height as f32,
            animations: animations.to_vec(),
            current_animation: 0,
            time: 0.0,
            frame: 0,
            playing,
        }
    }

    pub fn set_animation(&mut self, animation: usize) {
        self.current_animation = animation;

        let animation = &self.animations[self.current_animation];
        self.frame %= animation.frames;
    }

    pub fn current_animation(&self) -> usize {
        self.current_animation
    }

    pub fn set_frame(&mut self, frame: u32) {
        self.frame = frame;
    }

    pub fn update(&mut self) {
        let animation = &self.animations[self.current_animation];

        if self.playing {
            self.time += get_frame_time();
            if self.time > 1. / animation.fps as f32 {
                self.frame += 1;
                self.time = 0.0;
            }
        }
        self.frame %= animation.frames;
    }

    pub fn frame(&self) -> AnimationFrame {
        let animation = &self.animations[self.current_animation];

        AnimationFrame {
            source_rect: Rect::new(
                self.tile_width * self.frame as f32,
                self.tile_height * animation.row as f32,
                self.tile_width,
                self.tile_height,
            ),
            dest_size: vec2(self.tile_width, self.tile_height),
        }
    }
}





for (i, point1) in WOOD_HUT_COORDINATES.iter().enumerate() {
    for (j, point2) in WOOD_HUT_COORDINATES.iter().enumerate() {
        if i != j && point1.distance(*point2) < INTERACTION_DISTANCE {
            draw_text("Cant place hut here", screen_width() / 2.0, screen_height() / 8.0, 30.0, ORANGE);
            can_draw = false;
        }
    }
}
}





let target= Vec2::new(x, y);
for (i, point1) in WOOD_HUT_COORDINATES.iter().enumerate() {
    for (j, point2) in WOOD_HUT_COORDINATES.iter().enumerate() {
        if i != j && point1.distance(*point2) < INTERACTION_DISTANCE {
            draw_text("Cant place hut here", screen_width() / 2.0, screen_height() / 8.0, 30.0, ORANGE);
            can_draw = false;
        }
    }
}
}








fn draw_hut(texture: Texture2D){
    let mut can_draw = true;
    if is_mouse_button_pressed(MouseButton::Left) {
        
        if unsafe {WOOD >= 30 && ROCKS >= 50}{          //need 30 wood and 50 stone
            let x = mouse_position().0;
            let y = mouse_position().1;
            let target= Vec2::new(x, y);
            unsafe{
                for point in WOOD_HUT_COORDINATES.iter() {
                    if point.distance(target) < INTERACTION_DISTANCE {
                        can_draw = true
                    }
                    else {
                        can_draw = false;
                    }
                }
            }
        }        
    }        
            unsafe {WOOD_HUT_COORDINATES.push(Vec2::new(x, y));
                WOOD -= 30;
                ROCKS -= 50;
            } 

    if unsafe {!WOOD_HUT_COORDINATES.is_empty()} {  //if u ever manage to draw impossible u won't be able to draw again
        
      unsafe{
        for position in &WOOD_HUT_COORDINATES{
            draw_texture(texture, position.x, position.y, WHITE);
            
     




















































n draw_hut(texture: Texture2D){ 
    let mut can_draw = true;
    if is_mouse_button_pressed(MouseButton::Left) {
        
        if unsafe {WOOD >= 30 && ROCKS >= 50}{          //need 30 wood and 50 stone
            let x = mouse_position().0;
            let y = mouse_position().1;
            let target= Vec2::new(x, y);
            unsafe{
                for point in WOOD_HUT_COORDINATES.iter() {
                    if point.distance(*target) < INTERACTION_DISTANCE {
                        can_draw = true
                    }
                    else {
                        can_draw = false;
                    }
                }
            }

            unsafe{ //shit time coplexity
            
            unsafe {WOOD_HUT_COORDINATES.push(Vec2::new(x, y));
                WOOD -= 30;
                ROCKS -= 50;
            } 
        }
        
    if unsafe {!WOOD_HUT_COORDINATES.is_empty()} {  //if u ever manage to draw impossible u won't be able to draw again
        
      unsafe{
        for position in &WOOD_HUT_COORDINATES{
            draw_texture(texture, position.x, position.y, WHITE);
            
        }
      }
    }
    
}