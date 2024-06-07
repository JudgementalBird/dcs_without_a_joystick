use std::thread::sleep; // Shortening
use std::time::Duration; // This mf up

use display_info::DisplayInfo; // Gets monitor info
use device_query::{DeviceQuery, DeviceState, Keycode}; // Listens to m+kb inputs
use mouse_rs::Mouse; // Sets mouse inputs

struct Position {
    x: i32,
    y: i32,
}
fn main(){

    let display_info = DisplayInfo::all().unwrap(); // Gets monitor info
    let display_center = Position{ // Calculates center of main display
        x: display_info[0].width as i32 / 2, 
        y: display_info[0].height as i32 / 2,
    };

    let device_state = DeviceState::new(); // Sets up m+kb device listening
    let mouse_out = Mouse::new(); // Sets up mouse output

    loop {

        let mouse_in = device_state.get_mouse(); // Reads mouse state
        let kb_in = device_state.get_keys(); // Reads keyboard state

        if kb_in.contains(&Keycode::X) { // If X is pressed, center mouse
            mouse_out.move_to(display_center.x,display_center.y).expect("gay");
        };

        println!("Mouse x,y: {:?}", mouse_in.coords); // Print mouse position

        sleep(Duration::from_millis(1)); // Wait 1ms
    }

}