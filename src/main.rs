use std::{thread, time::Duration};

mod framebuffer;
mod gameoflife;
mod patterns; 

use framebuffer::Framebuffer;
use gameoflife::GameOfLife;

fn main() {
    let window_width = 1600;
    let window_height = 960;
    
    let game_width = 200;
    let game_height = 120;

    let cell_size = window_width / game_width;

    let (mut raylib_handle, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Game of Life: Ecosistema de Vida")
        .build();

    let mut framebuffer = Framebuffer::new(window_width as usize, window_height as usize);
    let mut game = GameOfLife::new(game_width as usize, game_height as usize);


    game.spawn_pattern(110, 54, &patterns::pulsar(), 2);
        game.spawn_pattern(150, 34, &patterns::mwss(), 2);

    game.spawn_pattern(70, 54, &patterns::pulsar(), 4);
    game.spawn_pattern(100, 50, &patterns::pentadecathlon(),2);


    game.spawn_pattern(60, 59, &patterns::pentadecathlon(),2);
    game.spawn_pattern(132, 59, &patterns::pentadecathlon(),2);

    for i in 0..8 {
        game.spawn_pattern(2 + i * 5, 2 + i * 3, &patterns::glider(),3);
        game.spawn_pattern(180 - i * 5, 2 + i * 3, &patterns::lwss(),4);
    }
    game.spawn_pattern(5, 60, &patterns::hwss(),3);
    game.spawn_pattern(5, 50, &patterns::hwss(),3);
    game.spawn_pattern(5, 40, &patterns::hwss(),3);
    game.spawn_pattern(5, 30, &patterns::hwss(),3);
    game.spawn_pattern(5, 20, &patterns::hwss(),3);

    
    for i in 0..10 {
        game.spawn_pattern(5, 100-i, &patterns::hwss(),3);
        game.spawn_pattern(100, 0+i, &patterns::hwss(),3);
    } 
    


    for i in 0..20 {
        game.spawn_pattern(50 + i * 5, 20, &patterns::blinker(),2);
        game.spawn_pattern(50 + i * 5, 100, &patterns::blinker(),2);
    }
    game.spawn_pattern(80, 8, &patterns::toad(),3);
    game.spawn_pattern(115, 8, &patterns::toad(),3);
    game.spawn_pattern(80, 110, &patterns::beacon(),4);
    game.spawn_pattern(115, 110, &patterns::beacon(),4);
    game.spawn_pattern(80, 200, &patterns::toad(),1);
    game.spawn_pattern(115, 45, &patterns::toad(),2);
    game.spawn_pattern(80, 200, &patterns::beacon(),3);
    game.spawn_pattern(115, 45, &patterns::beacon(),4);

    game.spawn_pattern(10, 10, &patterns::block(),3);
    game.spawn_pattern(185, 10, &patterns::block(),3);
    game.spawn_pattern(10, 105, &patterns::block(),3);
    game.spawn_pattern(185, 105, &patterns::block(),3);

    game.spawn_pattern(30, 40, &patterns::beehive(),4);
    game.spawn_pattern(165, 40, &patterns::beehive(),4);
    game.spawn_pattern(30, 80, &patterns::loaf(), 4);
    game.spawn_pattern(165, 80, &patterns::loaf(), 4);

    game.spawn_pattern(5, 30, &patterns::boat(),1);
    game.spawn_pattern(180, 30, &patterns::boat(),1);
    game.spawn_pattern(5, 90, &patterns::tub(),1);
    game.spawn_pattern(180, 90, &patterns::tub(),1);
    
    let mut generation = 0;
    
    while !raylib_handle.window_should_close() {
        if generation % 3 == 0 { 
            game.update();
        }
        
        framebuffer.clear();
        game.render(&mut framebuffer, cell_size, cell_size);
        framebuffer.swap_buffers(&mut raylib_handle, &raylib_thread);
        
        generation += 1;
        thread::sleep(Duration::from_millis(8));
    }
}