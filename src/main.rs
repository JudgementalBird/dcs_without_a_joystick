use std::thread::sleep; // Shortening
use std::time::Duration; // This mf up

use device_query::{device_state, DeviceQuery, DeviceState, Keycode, MouseState}; // Listens to m+kb inputs
use mouse_rs::{types::keys::Keys, Mouse}; // Sets mouse inputs

fn main(){

    let device_state = DeviceState::new();
    let mouse_out = Mouse::new();

    loop {

        let mouse_in = device_state.get_mouse();
        let kb_in = device_state.get_keys();

        if kb_in.contains(&Keycode::X) {
            mouse_out.move_to(0,0).expect("gay");
        };

        println!("Mouse x,y: {:?}", mouse_in.coords);

        sleep(Duration::from_millis(1));
    }

}