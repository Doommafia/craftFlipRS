use std::{thread, time};
use enigo::{Enigo, MouseControllable, KeyboardControllable};

fn move_mouse_for_crafting(base_x: i32, base_y: i32, enigo: &mut Enigo) {
    let final_product_x = 1015;  // Adjust according to your screen layout
    let final_product_y = 420;   // Adjust according to your screen layout
    
    for j in 0..3 {
        let mouse_y = base_y + j * 45;
        for i in 0..10 {
            let mouse_x = base_x + i * 50;
            thread::sleep(time::Duration::from_millis(60));
            enigo.mouse_move_to(mouse_x as i32, mouse_y as i32);
            enigo.key_down(enigo::Key::Shift);
            enigo.mouse_click(enigo::MouseButton::Left);
            enigo.key_up(enigo::Key::Shift);
        }
        thread::sleep(time::Duration::from_millis(120));
        enigo.mouse_move_to(final_product_x as i32, final_product_y as i32);
        enigo.key_down(enigo::Key::Shift);
        enigo.mouse_click(enigo::MouseButton::Left);
        enigo.key_up(enigo::Key::Shift);
    }
}

fn craft_item(enigo: &mut Enigo) {
    let base_x = 745;  // Adjust according to your screen
    let base_y = 685;  // Adjust according to your screen
    
    println!("Press Enter to start the macro. Make sure the game window is active and ready. (You have 2 seconds)");
    let _ = std::io::stdin().read_line(&mut String::new());
    thread::sleep(time::Duration::from_secs(2));
    
    move_mouse_for_crafting(base_x, base_y, enigo);
}

fn main() {
    let mut enigo = Enigo::new();
    
    loop {
        craft_item(&mut enigo);
        println!("Crafting finished. Press Enter to restart or Ctrl+C to exit.");
        let _ = std::io::stdin().read_line(&mut String::new());
    }
}
