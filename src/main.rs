use std::thread::sleep; // Shortening
use std::time::Duration; // This mf up

use display_info::DisplayInfo; // Gets monitor info
use device_query::{DeviceQuery, DeviceState, Keycode}; // Listens to m+kb inputs
use mouse_rs::Mouse; // Sets mouse inputs
use vjoy::VJoy; // Sets up vjoy feeder

fn map_range(x: i32, from_range: (i32,i32), to_range: (i32,i32)) -> i32 { // Maps an x within one numeric range to another
    to_range.0 + (x - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

struct Position {
    x: i32,
    y: i32,
}

fn main(){

    let mut vjoy = VJoy::from_default_dll_location().unwrap(); // Gets VJoy dll
    let mut joystick = vjoy.get_device_state(1).unwrap(); // Set current controlled joystick to vjoy device 1

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

        let joystick_xy = Position{ // Map mouse inside monitor to joystick's range
            x: map_range(mouse_in.coords.0, (display_center.x-display_center.y,display_center.x+display_center.y), (0,32768)),
            y: map_range(mouse_in.coords.1, (0,display_info[0].height as i32), (32768,0)),
        };

        /* Simpler mapping calculation for this case
        let joystick_xy = Position{ // Offset mouse to monitor center, normalize, and offset to joystick range center
            x: (mouse_in.coords.0 - display_center.x) * 32768 / display_info[0].height as i32 + 16384,
            y: (-mouse_in.coords.1 + display_center.y) * 32768 / display_info[0].height as i32 + 16384 - 15,
        };
        */

        // Set x and y axis on joystick
        joystick.set_axis(1, joystick_xy.x).unwrap();
        joystick.set_axis(2, joystick_xy.y).unwrap();
        
        vjoy.update_device_state(&joystick).unwrap(); // Update vjoy device

        sleep(Duration::from_millis(1)); // Wait 1ms
    }

}