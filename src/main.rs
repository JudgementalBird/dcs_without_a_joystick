use libloading::{Library, Symbol};
use bytemuck::{Pod, Zeroable};
use std::thread::sleep;
use std::time::Duration;

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
    // Load the DLL containing the NP_GetData function
    let lib = match unsafe { Library::new("protocols/NPClient64_NJ.dll") } {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("Failed to load DLL: {}", e);
            return;
        }
    };

    unsafe {
        // Define the function signature for NP_GetData
        let np_get_data: Symbol<unsafe extern "C" fn(*mut TirData) -> i32> =
            match lib.get(b"NP_GetData") {
                Ok(symbol) => symbol,
                Err(e) => {
                    eprintln!("Failed to load NP_GetData function: {}", e);
                    return;
                }
            };

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

        loop {
            // Call the NP_GetData function to get the tracking data
            let result = np_get_data(&mut data as *mut TirData);
            if result == 0 {
                // Print the data to verify its content
                println!("{:?}", data);
            } else {
                eprintln!("Failed to get data from NP_GetData, error code: {}", result);
            }

            // Sleep for a while before reading again
            sleep(Duration::from_secs(1));
        }
    }
}
