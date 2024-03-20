use std::{thread, time};
use enigo::{Enigo, KeyboardControllable, MouseControllable};

fn move_mouse_for_crafting(base_x: i32, base_y: i32, enigo: &mut Enigo) {
    let final_product_x = 1015;  // Adjust according to your screen layout
    let final_product_y = 420;   // Adjust according to your screen layout

    enigo.key_down(enigo::Key::Shift);
    for j in 0..3 {
        let mouse_y = base_y + j * 45;
        for i in 0..10 {
            let mouse_x = base_x + i * 50;
            thread::sleep(time::Duration::from_millis(200));
            enigo.mouse_move_to(mouse_x as i32, mouse_y as i32);
            thread::sleep(time::Duration::from_millis(50)); // Wait a bit before clicking
            enigo.mouse_click(enigo::MouseButton::Left);
            enigo.mouse_move_to(mouse_x as i32, mouse_y as i32); // Move back to original position
        }
        thread::sleep(time::Duration::from_millis(600));
        enigo.mouse_move_to(final_product_x as i32, final_product_y as i32);
        thread::sleep(time::Duration::from_millis(0)); // Wait a bit before clicking
        enigo.mouse_click(enigo::MouseButton::Left);
        enigo.mouse_move_to(final_product_x as i32, final_product_y as i32); // Move back to original position
    }
    enigo.key_up(enigo::Key::Shift);
}

fn get_backpack(enigo: &mut Enigo) {
    // Press 'E' key
    enigo.key_down(enigo::Key::Layout('E'));
    thread::sleep(time::Duration::from_millis(100));
    enigo.key_up(enigo::Key::Layout('E'));

    // Press 'T' key
    enigo.key_click(enigo::Key::Layout('T'));
    thread::sleep(time::Duration::from_millis(100));

    // Type "/backpack 9"
    enigo.key_sequence("/backpack 9");
    thread::sleep(time::Duration::from_millis(100));

    // Click on (745, 685), (795, 685), and (845, 685)
    enigo.mouse_move_to(745, 685);
    enigo.mouse_click(enigo::MouseButton::Left);
    thread::sleep(time::Duration::from_millis(100));

    enigo.mouse_move_to(795, 685);
    enigo.mouse_click(enigo::MouseButton::Left);
    thread::sleep(time::Duration::from_millis(100));

    enigo.mouse_move_to(845, 685);
    enigo.mouse_click(enigo::MouseButton::Left);
}

fn get_bazaar(enigo: &mut Enigo) {
    // Press 'E' key
    enigo.key_down(enigo::Key::Layout('E'));
    thread::sleep(time::Duration::from_millis(100));
    enigo.key_up(enigo::Key::Layout('E'));

    // Press 'T' key
    enigo.key_click(enigo::Key::Layout('T'));
    thread::sleep(time::Duration::from_millis(100));

    // Type "/bz"
    enigo.key_sequence("/bz");
    thread::sleep(time::Duration::from_millis(100));

    // Click on (1045, 820)
    enigo.mouse_move_to(1045, 820);
    enigo.mouse_click(enigo::MouseButton::Left);
    thread::sleep(time::Duration::from_millis(100));

    // Click on (790, 900)
    enigo.mouse_move_to(790, 900);
    enigo.mouse_click(enigo::MouseButton::Left);
}

fn craft_menu(enigo: &mut Enigo) {
    // Press 'E' key
    enigo.key_down(enigo::Key::Layout('E'));
    thread::sleep(time::Duration::from_millis(100));
    enigo.key_up(enigo::Key::Layout('E'));

    // Press 'T' key
    enigo.key_click(enigo::Key::Layout('T'));
    thread::sleep(time::Duration::from_millis(100));

    // Type "/craft"
    enigo.key_sequence("/craft");
    thread::sleep(time::Duration::from_millis(100));
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
        get_backpack(&mut enigo);
        thread::sleep(time::Duration::from_secs(3));
        get_bazaar(&mut enigo);
        thread::sleep(time::Duration::from_secs(3));
        craft_menu(&mut enigo);
        thread::sleep(time::Duration::from_secs(3));    
        craft_item(&mut enigo);
        println!("Crafting finished. Press Enter to go again or Ctrl+C to exit.");
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);

        if input.trim() == "" {
            get_backpack(&mut enigo);
            thread::sleep(time::Duration::from_secs(3));
            get_bazaar(&mut enigo);
            thread::sleep(time::Duration::from_secs(3));
            craft_menu(&mut enigo);
            thread::sleep(time::Duration::from_secs(3));    
            craft_item(&mut enigo);
        } else {
            break; // Exit loop if input is not empty
        }
    }
}
