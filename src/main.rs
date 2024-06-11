use libloading::{Library, Symbol};
use bytemuck::{Pod, Zeroable};
use std::thread::sleep;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode}; // Listens to m+kb inputs

// Define the structure representing the tracking data
#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
struct TirData {
    frame: i32,
    status: i32,
    roll: f32,
    pitch: f32,
    yaw: f32,
    tx: f32,
    ty: f32,
    tz: f32,
    padding: [f32; 9],
    cksum: i32,
}

fn main() {
    // Load the modified NPClient DLL for reading tracking data
    let lib_nj = match unsafe { Library::new("protocols/NPClient64_NJ.dll") } {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("Failed to load modified DLL: {}", e);
            return;
        }
    };

    // Load the original NPClient DLL for writing tracking data
    let lib = match unsafe { Library::new("protocols/NPClient64.dll") } {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("Failed to load original DLL: {}", e);
            return;
        }
    };

    unsafe {
        // Define the function signature for NP_GetData from the modified DLL
        let np_get_data: Symbol<unsafe extern "C" fn(*mut TirData) -> i32> =
            match lib_nj.get(b"NP_GetData") {
                Ok(symbol) => symbol,
                Err(e) => {
                    eprintln!("Failed to load NP_GetData function: {}", e);
                    return;
                }
            };

        // Define the function signatures for the original NPClient DLL
        let np_start_data_transmission: Symbol<unsafe extern "C" fn() -> i32> =
            match lib.get(b"NP_StartDataTransmission") {
                Ok(symbol) => symbol,
                Err(e) => {
                    eprintln!("Failed to load NP_StartDataTransmission function: {}", e);
                    return;
                }
            };

        let np_stop_data_transmission: Symbol<unsafe extern "C" fn() -> i32> =
            match lib.get(b"NP_StopDataTransmission") {
                Ok(symbol) => symbol,
                Err(e) => {
                    eprintln!("Failed to load NP_StopDataTransmission function: {}", e);
                    return;
                }
            };

        let np_set_parameter: Symbol<unsafe extern "C" fn(i32, i32) -> i32> =
            match lib.get(b"NP_SetParameter") {
                Ok(symbol) => symbol,
                Err(e) => {
                    eprintln!("Failed to load NP_SetParameter function: {}", e);
                    return;
                }
            };

        // Start data transmission
        np_start_data_transmission();

        // Create a buffer to store the tracking data
        let mut data = TirData {
            frame: 0,
            status: 0,
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
            tx: 0.0,
            ty: 0.0,
            tz: 0.0,
            padding: [0.0; 9],
            cksum: 0,
        };

        // Sets up m+kb device listening
        let device_state = DeviceState::new();

        loop {

            // Reads keyboard state
            let kb_in = device_state.get_keys();

            // Break loop if ctrl+c is pressed
            if kb_in.contains(&Keycode::LControl) && kb_in.contains(&Keycode::K) { 
                println!("Stopping program");
                break 
            }

            // Call the NP_GetData function to get the tracking data
            let result = np_get_data(&mut data as *mut TirData);
            if result == 0 {
                // Print the data to verify its content
                println!("{:?}", data);

                // Call NP_SetParameter to write the tracking data to shared memory
                np_set_parameter(0, data.frame); // Example: replace with actual data parameter settings
                np_set_parameter(1, data.status);
                np_set_parameter(2, data.roll.to_bits() as i32);  // converting float to bits and then to i32
                np_set_parameter(3, data.pitch.to_bits() as i32);
                np_set_parameter(4, data.yaw.to_bits() as i32);
                np_set_parameter(5, data.tx.to_bits() as i32);
                np_set_parameter(6, data.ty.to_bits() as i32);
                np_set_parameter(7, data.tz.to_bits() as i32);
            } else {
                eprintln!("Failed to get data from NP_GetData, error code: {}", result);
            }

            // Sleep for a while before reading again
            sleep(Duration::from_millis(1));
        }

        // Stop data transmission
        np_stop_data_transmission();
        println!("Stopped data transmission");

    }
}