use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc};
use std::thread;
use std::thread::JoinHandle;

const DEFAULT_NUM_GUESTS: i32 = 100;

// I chose to implement strategy 2

fn main() {
    let vase_occupied = Arc::new(AtomicBool::new(false));

    // Get number of threads from cli args or use default value
    let args: Vec<String> = std::env::args().collect();
    let num_guests = match args.len() {
        2 => match args[1].parse::<i32>() {
            Ok(n) => n,
            Err(_) => DEFAULT_NUM_GUESTS,
        },
        _ => DEFAULT_NUM_GUESTS,
    };

    println!("Running with {} guests", num_guests);

    let mut guests: Vec<JoinHandle<()>> = Vec::new();

    #[allow(unused_variables)]
    for i in 0..num_guests {
        let vase_occupied = vase_occupied.clone();
        #[allow(unused_variables)]
        let guest_id = i.clone();

        guests.push(thread::spawn(move || {
            loop {
                // Set sign to busy
                if !vase_occupied.swap(true, Ordering::Relaxed) {
                    #[cfg(not(feature = "suppress_guest_action_log"))]
                    println!("[Guest {}]: Entering the Vase Room", guest_id);
                    // Sleep thread to ensure a distance between occupied and unoccupied states
                    thread::sleep(std::time::Duration::from_millis(0_u64));
                    // Set sign to available
                    vase_occupied.store(false, Ordering::Relaxed);
                    #[cfg(not(feature = "suppress_guest_action_log"))]
                    println!("[Guest {}]: Leaving the Vase Room", guest_id);
                    break;
                }
            }
        }));
    }

    // Wait for guests to shutdown
    for guest in guests.into_iter() {
        guest.join().unwrap();
    }
}

