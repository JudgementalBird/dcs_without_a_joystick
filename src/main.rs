use std::thread::sleep; // Sleep
use std::time::{Duration,Instant}; // Time shit
use display_info::DisplayInfo; // Gets monitor info
use device_query::{DeviceQuery, DeviceState, Keycode}; // Listens to m+kb inputs
use mouse_rs::Mouse; // Sets mouse inputs
use vjoy::{VJoy, ButtonState, Error}; // Sets up vjoy feeder

fn map_range(x: i32, from_range: (i32,i32), to_range: (i32,i32)) -> i32 { // Maps an x within one numeric range to another
    to_range.0 + (x - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

#[derive(Debug, Copy, Clone)]
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
    let display_center = Position{x: display_info[0].width as i32 / 2, y: display_info[0].height as i32 / 2,}; // Calculates center of main display

    let device_state = DeviceState::new(); // Sets up m+kb device listening
    let mouse_out = Mouse::new(); // Sets up mouse output

    let mut mouse_toggle = Button {button_type: ButtonType::Toggle, ..Default::default()}; // Mouse toggle button
    let mut mouse_saved_xy = Position {x: display_center.x, y: display_center.y}; // Mouse saved position

    loop {

        let mouse_in = device_state.get_mouse(); // Reads mouse state
        let kb_in = device_state.get_keys(); // Reads keyboard state
        let mouse_toggle_last_state = mouse_toggle.state; // Gets last mouse toggle state

        // If LWin is pressed
        mouse_toggle.update(kb_in.contains(&Keycode::LMeta)); // Toggle mouse
        joystick.set_button(64, if kb_in.contains(&Keycode::LMeta) {ButtonState::Pressed} else {ButtonState::Released})?; // Toggle VJoy button 64 (For disabling TrackIR)

        if !mouse_toggle.state & mouse_toggle_last_state { // If mouse has been toggled off
            mouse_saved_xy = Position { x: mouse_in.coords.0, y: mouse_in.coords.1} // Save mouse coordinates
        } else if mouse_toggle.state & !mouse_toggle_last_state { // If mouse has been toggled on
            mouse_out.move_to(mouse_saved_xy.x,mouse_saved_xy.y).expect("Mouse couldn't be moved"); // Move mouse to saved position
        };

        let joystick_xy = Position { // Map mouse inside monitor to joystick's range
        x: map_range(mouse_in.coords.0, (display_center.x-display_center.y,display_center.x+display_center.y), (0,32768)),
        y: map_range(mouse_in.coords.1, (0,display_info[0].height as i32), (0,32768)),
        };

        // Set x and y axis on joystick
        joystick.set_axis(1, joystick_xy.x)?;
        joystick.set_axis(2, joystick_xy.y)?;
        
        vjoy.update_device_state(&joystick)?; // Update vjoy device

        sleep(Duration::from_millis(1)); // Wait 1ms
    }

}