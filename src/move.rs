extern crate gilrs;

use gilrs::{Gilrs, Axis, Event};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    loop {
        // Process events
        while let Some(Event { id, .. }) = gilrs.next_event() {
            println!("New event from {:?}", id);
        }

        // Assuming we're only interested in the first connected gamepad
        if let Some(gamepad) = gilrs.gamepads().next().map(|(_, gamepad)| gamepad) {
            let left_stick_x = gamepad.value(Axis::LeftStickX);
            let left_stick_y = gamepad.value(Axis::LeftStickY);

            // Print the values of the left stick
            println!("Left Stick X: {:.2}, Left Stick Y: {:.2}", left_stick_x, left_stick_y);
        }

        // Small delay to make the output readable
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
