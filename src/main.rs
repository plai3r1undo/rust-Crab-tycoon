extern crate rand;


use macroquad::prelude::*;
use macroquad::audio::*;
use std::thread;
use std::time::Duration;
use rand::Rng;




const PLAYER_SIZE: Vec2 = Vec2::from_array([120f32, 120f32]);
const PLAYER_SPEED: f32 = 250.;
const RUN_SPEED: f32 = 490.0;
static mut STAMINA: f32 = 300.0;  
const INTERACTION_DISTANCE: f32 = 220.0;
//TODO fix overflows with reseources



static mut WOOD: i32 = 0;
static mut ROCKS: i32 = 0;
static mut CHARCOAL: i32 = 0;

//Structures info
const N_TREES: u32 = 6;
const N_ROCKS: u32 = 8;
static mut WOOD_HUT_COORDINATES: Vec<Vec2> = Vec::new();
static mut CHAR_HUT_COORDINATES:Vec<Vec2> = Vec::new();
static mut STONE_MINE_COORDINATES:Vec<Vec2> = Vec::new();


//name
static FONT_SIZE: f32 = 30.0;

struct Rock {
    tetxure: Texture2D
}

struct Tree {
    texture: Texture2D,
}


struct Player {
    texture: Texture2D,
    rect: Rect,
}

struct InventoryBar {
    slot_width :f32,
    slot_height :f32,
    slot_padding :f32,
    num_slots: i32,
    inventory_x :f32,
    wood_texture : Texture2D,
    rock_texture: Texture2D,
    charcoal_texture: Texture2D,
}

impl InventoryBar {
    pub async fn new() -> Self {
        //change to something better in the future, same names may cause bugs ??
        let slot_width = 80.0;
        let slot_height = 80.0;
        let slot_padding = 5.0;
        let num_slots = 10;
        let inventory_width = num_slots as f32 * (slot_width + slot_padding);
        let inventory_x = (screen_width() - inventory_width) / 2.0;
        Self {
            slot_width: slot_width,
            slot_height:slot_height,
            slot_padding: slot_padding,
            num_slots: num_slots,
            inventory_x: inventory_x,
            wood_texture: load_texture("textures/wood.png").await.unwrap(),  //should handle error if texture doesn't load but idc
            rock_texture: load_texture("textures/rock-icon.png").await.unwrap(),
            charcoal_texture: load_texture("textures/charcoal.png").await.unwrap(),

        }
    }
    pub fn draw_inventory(&self) {
        let mut toomuch_ch = false;
        for i in 0..self.num_slots {
            let x = self.inventory_x + i as f32 * (self.slot_width + self.slot_padding);
            let y = 20.0;
            draw_rectangle(x, y, self.slot_width, self.slot_height, BLACK);
            if i == 0 && unsafe {WOOD} > 0 {
                draw_texture(self.wood_texture, x +5.0 , y + 5.0, WHITE);
                draw_text(&format!("{}", unsafe {WOOD}), x, 130.0, FONT_SIZE, BLACK);
                if unsafe { WOOD} < 0{
                    draw_text("Deficit", x, y, FONT_SIZE, RED);
                }
            }
            if i == 1 && unsafe { ROCKS } > 0 {
                draw_texture(self.rock_texture, x +5.0 , y + 5.0, WHITE);
                draw_text(&format!("{}", unsafe {ROCKS}), x, 130.0, FONT_SIZE, BLACK);
                if unsafe {ROCKS} < 0{
                    draw_text("Deficit", x, 130.0, FONT_SIZE, RED);
                }
            }
            if i == 2 && unsafe {CHARCOAL} > 0{
                draw_texture(self.charcoal_texture, x + 5.0, y + 5.0, WHITE);
                if unsafe {CHARCOAL < 10000} && !toomuch_ch{
                    draw_text(&format!("{}", unsafe {CHARCOAL}), x, 130.0, FONT_SIZE, BLACK);
                    toomuch_ch = false;
                }
                if unsafe {CHARCOAL} < 0{
                    draw_text("Deficit", x, 130.0, FONT_SIZE, RED);
                }
                else if unsafe{CHARCOAL} > 1000{
                    toomuch_ch = true;
                }
                
                
            }
            
        }
    }
    
}


impl Tree {
    pub async fn new() -> Self {
        Self {
            texture: match load_texture("textures/albero.png").await {
                Ok(texure) => texure,
                Err(error) => {eprintln!("Albero.png texture not found {}", error); Texture2D::empty()}
            }
        }
    }


    pub fn is_player_nearby(&self,tree_position:Vec2, player_pos: Vec2) -> bool {
        let disntace = (tree_position - player_pos).length();
        disntace <= INTERACTION_DISTANCE
    }
    pub fn interact(&self) {
        unsafe{ WOOD += 1; }
    }

    pub fn fill(&self) -> Vec<Vec2> {
        let mut tree_positions = Vec::new();
        for _ in 0..N_TREES {
            let mut rng0 = rand::thread_rng();
            let mut rng1 = rand::thread_rng();
            let x: u32 = rng0.gen_range(30u32..screen_width() as u32);  
            let y = rng1.gen_range(30..screen_height() as u32 - 30);
            println!("random coordinates x {}, y {}",x, y);
            tree_positions.push(Vec2::new(x as f32, y as f32))
        }
        return tree_positions;
    }
}   

impl Rock {
    pub async fn new() -> Self {
        Self {
            tetxure: match load_texture("textures/rock.png").await {
                Ok(texture) => texture,
                Err(error) => {
                    eprintln!("Failed to load rock.png texture, maybe missing textures folder ? {} ", error);
                    Texture2D::empty()
                }
            }
        }
    }
    pub fn fill(&self) -> Vec<Vec2> {
        let mut tree_positions = Vec::new();
        for _ in 0..N_ROCKS {
            let mut rng0 = rand::thread_rng();
            let mut rng1 = rand::thread_rng();
            let x: u32 = rng0.gen_range(30u32..screen_width() as u32);  
            let y = rng1.gen_range(30..screen_height() as u32 - 30);
            tree_positions.push(Vec2::new(x as f32, y as f32))
        }
        return tree_positions;
    }

    pub fn is_player_nearby(&self,rock_position:Vec2, player_pos: Vec2) -> bool {
        let disntace = (rock_position - player_pos).length();
        disntace <= INTERACTION_DISTANCE
    }
    pub fn interact(&self) {
        unsafe {
            ROCKS += 1;
        }
    }
}


impl Player {
    pub async fn new() -> Self {
        Self {
            texture: load_texture("textures/crab.png").await.unwrap(),
            rect: Rect::new(
                screen_width() / 2.,
                screen_height() / 2.,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn draw(&self) {
        draw_texture(self.texture, self.rect.x, self.rect.y, WHITE);
    }

    pub fn update_position(&mut self, delta_time: f32) {
        let x_move = match (is_key_down(KeyCode::A), is_key_down(KeyCode::D)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        let y_move = match (is_key_down(KeyCode::W), is_key_down(KeyCode::S)) {
            (true, false) => -1.,
            (false, true) => 1.,
            _ => 0.,
        };

        if is_key_down(KeyCode::LeftShift) && unsafe { STAMINA } >= 10.0 {  
            self.rect.y += y_move * delta_time * RUN_SPEED; 
            self.rect.x += x_move * delta_time * RUN_SPEED;
            unsafe { STAMINA -= 2.0; }
        }
        else {
            self.rect.y += y_move * delta_time * PLAYER_SPEED; 
            self.rect.x += x_move * delta_time * PLAYER_SPEED;
            if unsafe { STAMINA } < 200.0 && !is_key_down(KeyCode::LeftShift){
                unsafe {STAMINA += 1.0; }
            }
            
        }
        //check world boarder


        if self.rect.x < 0.0 {
            self.rect.x = 0.;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w
        }
        if self.rect.y < 0. {
            self.rect.y = 0.;
        }
        if self.rect.y > screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h;
        }

    }
    
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Crab Tycoon".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}


fn is_material_overflow() -> bool{
    let max = i32::MAX - 10;
    if unsafe{WOOD > max && ROCKS > max && CHARCOAL > max}{
        return true;
    }
    else {
        return false
    }
}


struct Huts{
    wood_hut_texture: Texture2D,
    charcoal_hut_texture: Texture2D,
    rock_mine_texture: Texture2D,
}


impl Huts{
    
    pub async fn new() -> Self{
        Self{
            wood_hut_texture: match load_texture("textures/wood_hut.png").await {
                Ok(texture) => texture,
                Err(error) => {
                    eprintln!("Failed to load wood_hut.png, maybe missing folder. \n {}", error);
                    Texture2D::empty()
                }
            },
            charcoal_hut_texture: match load_texture("textures/charcoal-hut.png").await {
                Ok(texture) => texture,
                Err(error) => {
                    eprintln!("Failed to load charcoal-hut.png, maybe missing textures folder ? {} ", error);
                    Texture2D::empty()
                }
            },
            rock_mine_texture: match load_texture("textures/rock-mine.png").await {
               Ok(texture) => texture,
                Err(error) => {
                    eprintln!("Failed to load rock-mine.png, maybe missing images folder ? \n {}", error);
                    Texture2D::empty()
                }
            }
        }
    }

    pub fn draw_wood_hut(&self, drawable: bool) {
        let mut can_draw = true;
        let x = mouse_position().0;
        let y = mouse_position().1;
        if drawable{//delete
            if unsafe { WOOD >= 30 && ROCKS >= 50} {
                let target = Vec2::new(x,y);
                unsafe{
                    for point in WOOD_HUT_COORDINATES.clone() {
                        if point.distance(target) > 300.0 {
                            can_draw = true;
                        }
                        else {
                            can_draw = false;
                        }
                    }
                    if can_draw{
                        WOOD_HUT_COORDINATES.push(Vec2::new(x, y));
                        WOOD -= 30;
                        ROCKS -= 30;
                    }
                }
            }
        }//delete
        unsafe{
            if !WOOD_HUT_COORDINATES.is_empty(){
                for position in &WOOD_HUT_COORDINATES{
                    draw_texture(self.wood_hut_texture, position.x, position.y, WHITE);
                }
            }
        }
    }
    pub fn draw_stone_mine(&self, drawable: bool){
        let mut can_draw = true;
        let x = mouse_position().0;
        let y = mouse_position().1;
        if drawable{
            if unsafe { WOOD >= 400 && ROCKS >= 150}{
                let target = Vec2::new(x, y);
                unsafe{
                    for point in STONE_MINE_COORDINATES.clone() {
                        if point.distance(target) > 300.0 {
                            can_draw = true;
                        }
                        else {
                            can_draw = false;
                        }
                    }
                    if can_draw {
                        STONE_MINE_COORDINATES.push(Vec2::new(x, y));
                        WOOD -= 400;
                        ROCKS -= 140;
                    }
                }
            }
        }
        unsafe{
            if !STONE_MINE_COORDINATES.is_empty() {
                for position in &STONE_MINE_COORDINATES{
                    draw_texture(self.rock_mine_texture, position.x, position.y, WHITE);
                }
            }
        }
    }


    pub fn draw_charcoal_hut(&self, drawable: bool) {
        let mut can_draw = true;
        let x = mouse_position().0;
        let y = mouse_position().1;
        if drawable{
            if unsafe{ WOOD >= 30 && ROCKS >= 50} {
                let target = Vec2::new(x, y);
                unsafe{
                    for point in CHAR_HUT_COORDINATES.clone() {
                        if point.distance(target) > 300.0 {
                            can_draw = true;
                        }
                        else {
                            can_draw = false;
                        }
                    }
                    if can_draw{
                        CHAR_HUT_COORDINATES.push(Vec2::new(x, y));
                        WOOD -= 20;
                        ROCKS -= 70;
                    }
                }
            }
        }
        unsafe{
            if !CHAR_HUT_COORDINATES.is_empty(){
                for position in &CHAR_HUT_COORDINATES{
                    draw_texture(self.charcoal_hut_texture, position.x, position.y, WHITE);
                }
            }
        }
    }
    
    pub fn produce_materials(&self){
        let n_wood_huts: i32;
        let n_charcoal_huts: i32;
        unsafe{
            let n:i32 = WOOD_HUT_COORDINATES.len() as i32; n_wood_huts = n;
            let n_char: i32 = CHAR_HUT_COORDINATES.len() as i32; n_charcoal_huts = n_char;             
            let n_rock_mine: i32 = STONE_MINE_COORDINATES.len() as i32;
            if (n_wood_huts / 2) as f32 >= n_charcoal_huts as f32 && !is_material_overflow() {
                WOOD -= 2 * n_charcoal_huts;
                WOOD += 1 * n_wood_huts;
                CHARCOAL += 1 * n_charcoal_huts;
            }
            if (n_charcoal_huts / 3) as f32 >= n_rock_mine as f32 && !is_material_overflow(){
                CHARCOAL -= 1 * n_rock_mine;
                ROCKS += 1 * n_rock_mine; 
            }
        }
    }
    pub fn draw_selected_hut(&self, mut selected_index: u16) -> u16{
        let build_preview_square_size: f32 = 200.0;
        let x: f32 = screen_width() / 2.0 - build_preview_square_size as f32 / 2.0;
        let y: f32 = screen_height() - 300.0;
        let color: Color = Color::new(0.0, 0.0, 0.0, 0.6); 
        draw_rectangle(x, y, build_preview_square_size, build_preview_square_size, color);
        //n tot of huts is 4
        //1 wood
        //2 charcoal
        //3 stone mine
        //TODO find a way to not use infinite if statements
        let params = DrawTextureParams { dest_size: Some(Vec2::new(150.0, 150.0)),
        ..Default::default()
        };
        self.draw_wood_hut(false);  //false -> does not draw new hut but draws previews
        self.draw_stone_mine(false);
        self.draw_charcoal_hut(false);
        if is_key_pressed(KeyCode::Key1){
            selected_index = 1;
            
        }
        if is_key_pressed(KeyCode::Key2){
            selected_index = 2;
        }
        if is_key_pressed(KeyCode::Key3){
            selected_index = 3;
        }
        if selected_index == 1 {
            draw_texture_ex(self.wood_hut_texture, x, y, color, params.clone());
            if is_mouse_button_pressed(MouseButton::Left){
                self.draw_wood_hut(true);
            }  
        }
        if selected_index == 2 {
            draw_texture_ex(self.charcoal_hut_texture, x, y, color, params.clone());
            if is_mouse_button_pressed(MouseButton::Left){
                self.draw_charcoal_hut(true);
            }
        }
        if selected_index == 3 {
            draw_texture_ex(self.rock_mine_texture, x, y, color, params.clone());
            if is_mouse_button_pressed(MouseButton::Left){
                self.draw_stone_mine(true);
            }
        }
        selected_index
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new().await;
    let mut selected_index = 1; 
    let interactable_tree = Tree::new().await;
    let rock = Rock::new().await;
    let inventory_bar = InventoryBar::new().await;
    let huts = Huts::new().await;
    let main_ost = load_sound("ost/pim.wav").await.unwrap();
    let mut volume: f32 = 0.2;
    let tree_position = interactable_tree.fill();
    let rock_positions = rock.fill();
    let tree_texture: Texture2D = interactable_tree.texture;
    let rock_texture: Texture2D = rock.tetxure;
    play_sound(main_ost, macroquad::audio::PlaySoundParams {looped: true, volume: 0.0});
    let huts_cloned = Huts::new().await;

    // meanging that lef mouse has been pressed and now we can draw a new hut

     thread::spawn(move || {
        loop {
            huts_cloned.produce_materials();
            thread::sleep(Duration::from_millis(500));
        }
    }); 

    loop {

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        clear_background(LIGHTGRAY);
        
        
        selected_index = huts.draw_selected_hut(selected_index);

        if is_key_pressed(KeyCode::K) {
            volume += 0.1;
        }
        if is_key_pressed(KeyCode::L) {
            volume -= 0.05;
        }
        if is_key_pressed(KeyCode::J){
            volume  = 0.0;
        }
        if volume < 0. && volume > 1. {
            if volume >= 0.05 {
                volume = 1.0
            }
            else {
                volume = 0.0
            }
        }
        
        set_sound_volume(main_ost, volume);
        
        for position in rock_positions.clone() {   //performance hit maybe
            draw_texture(rock_texture, position.x, position.y, WHITE);
            if rock.is_player_nearby(position, player.rect.point()) && is_key_pressed(KeyCode::Space) {
                rock.interact();
            }
        }
        for position in tree_position.clone() {   //performance hit maybe
            draw_texture(tree_texture, position.x, position.y, WHITE);
            if interactable_tree.is_player_nearby(position, player.rect.point()) && is_key_pressed(KeyCode::Space) {
                interactable_tree.interact();
            }
        }


        player.draw();
        player.update_position(get_frame_time());
        draw_text(&format!("Stamina: {}", unsafe { STAMINA }), 20.0, 20.0, 30.0, BLACK);
        draw_text(&format!("Volume : {},  Volume + : k, - : L", volume), 20.0, 80.0, 30.0, BLACK);
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 120.0, 30.0, RED);
        draw_text("buildings only produce if you have enough to mantain upkeep", 20.0, 160.0, 40.0, RED);
        inventory_bar.draw_inventory();
        next_frame().await
    }
}



