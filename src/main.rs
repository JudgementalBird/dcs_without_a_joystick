use std::thread::sleep; // Sleep
use std::time::{Duration,Instant}; // Time shit
use display_info::DisplayInfo; // Gets monitor info
use device_query::{DeviceQuery, DeviceState, Keycode}; // Listens to m+kb inputs
use mouse_rs::Mouse; // Sets mouse inputs
use vjoy::{VJoy, Error}; // Sets up vjoy feeder

fn map_range(x: i32, from_range: (i32,i32), to_range: (i32,i32)) -> i32 { // Maps an x within one numeric range to another
    to_range.0 + (x - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

struct Position { // i32 x,y position
    x: i32,
    y: i32,
}

enum ButtonType { // Button types
    Toggle,
    Hold(Instant),
}

struct Button { // Button struct
    state: bool,
    currently_pressed: bool,
    was_pressed: bool,
    button_type: ButtonType,
}

impl Default for Button { // Default Button settings
    fn default() -> Self {
        Button {
            state: false,
            currently_pressed: false,
            was_pressed: false,
            button_type: ButtonType::Toggle,
        }
    }
}

impl Button {
    fn update(&mut self, pressed: bool) { // Update Button state
        self.was_pressed = self.currently_pressed;
        self.currently_pressed = pressed;
        match self.button_type {
            ButtonType::Toggle => 
                self.state = self.state ^ (pressed & (!self.was_pressed)),
            ButtonType::Hold(instant) => {
                if pressed & (!self.was_pressed) {
                    self.button_type = ButtonType::Hold(Instant::now());
                };
                self.state = pressed & (instant.elapsed() >= Duration::from_millis(500));
            }
        }
    }

}

fn main() -> Result<(), Error> {

    let mut vjoy = VJoy::from_default_dll_location()?; // Gets VJoy dll
    let mut joystick = vjoy.get_device_state(1)?; // Set current controlled joystick to vjoy device 1

    let display_info = DisplayInfo::all().expect("Display information couldn't be obtained"); // Gets monitor info
    let display_center = Position{ // Calculates center of main display
        x: display_info[0].width as i32 / 2, 
        y: display_info[0].height as i32 / 2,
    };

    let device_state = DeviceState::new(); // Sets up m+kb device listening
    let mouse_out = Mouse::new(); // Sets up mouse output

    let mut xy_toggle = Button {button_type: ButtonType::Toggle, ..Default::default()}; // XY Joystick toggle button
    let mut test_hold = Button {button_type: ButtonType::Hold(Instant::now()), ..Default::default()}; // Test hold button

    loop {

        let mouse_in = device_state.get_mouse(); // Reads mouse state
        let kb_in = device_state.get_keys(); // Reads keyboard state

        let joystick_xy = Position{ // Map mouse inside monitor to joystick's range
            x: map_range(mouse_in.coords.0, (display_center.x-display_center.y,display_center.x+display_center.y), (0,32768)),
            y: map_range(mouse_in.coords.1, (0,display_info[0].height as i32), (32768,0)),
        };

        xy_toggle.update(kb_in.contains(&Keycode::X)); // If X is pressed, toggle center mouse

        test_hold.update(kb_in.contains(&Keycode::Z)); // If Z is held for 500ms, toggle test button
        println!("Hold key is {}", test_hold.state);


        if xy_toggle.state { // If X is pressed, center mouse
            mouse_out.move_to(display_center.x,display_center.y).expect("Mouse couldn't be moved");
        };

        // Set x and y axis on joystick
        joystick.set_axis(1, if xy_toggle.state {16384} else {joystick_xy.x})?;
        joystick.set_axis(2, if xy_toggle.state {16384} else {joystick_xy.y})?;
        
        vjoy.update_device_state(&joystick)?; // Update vjoy device

        sleep(Duration::from_millis(1)); // Wait 1ms
    }

}