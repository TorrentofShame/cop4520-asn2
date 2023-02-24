use rand::prelude::*;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{atomic::AtomicBool, atomic::Ordering, mpsc, Arc};
use std::thread;
use std::thread::JoinHandle;

const DEFAULT_NUM_GUESTS: i32 = 100;

fn main() {
    let is_cake_eaten = Arc::new(AtomicBool::new(false));
    let should_run = Arc::new(AtomicBool::new(true));
    let counter_exists = Arc::new(AtomicBool::new(false));

    // Channels used to "wake" guests
    let mut guest_wol: Vec<mpsc::Sender<()>> = Vec::new();

    // Channel so guests can notify the minotaur that they've left the labyrinth
    let (labyrinth_tx, labyrinth_rx): (Sender<()>, Receiver<()>) = mpsc::channel();

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
        let (wol_tx, wol_rx) = mpsc::channel();
        guest_wol.push(wol_tx);
        #[allow(unused_variables)]
        let guest_id = i.clone();
        let labyrinth_tx = labyrinth_tx.clone();
        let is_cake_eaten = is_cake_eaten.clone();
        let should_run = should_run.clone();
        let counter_exists = counter_exists.clone();
        guests.push(thread::spawn(move || {
            let mut is_counter = false;
            let mut has_eaten = false;
            let mut count = 0;
            //            let mut num_runthroughs = 0;
            loop {
                // Wait for wake signal (announcement)
                wol_rx.recv().unwrap();
                // Exit the loop if the state has changed
                if !should_run.load(Ordering::Relaxed) {
                    //                    println!("{} {} {} {} {}",
                    //                        guest_id, is_counter, count, has_eaten, num_runthroughs);
                    break;
                }

                #[cfg(not(feature = "suppress_guest_action_log"))]
                println!("[Guest {}]: Entering the Labyrinth", guest_id);

                if !counter_exists.load(Ordering::Relaxed) {
                    counter_exists.store(true, Ordering::Relaxed);
                    is_counter = true;
                    // We know that we've finished the maze.
                    // The counter gets no cake, the cake is a lie.
                    count += 1;
                }

                if is_counter && is_cake_eaten.load(Ordering::Relaxed) {
                    count += 1;

                    // Everyone has eated a cake and therefore everyone has made it through the
                    // maze
                    if count == num_guests {
                        should_run.store(false, Ordering::Relaxed);
                    } else {
                        // Since there is no cake, request a replacement for the next guy
                        is_cake_eaten.store(true, Ordering::Relaxed);
                    }
                } else if !has_eaten && !is_cake_eaten.load(Ordering::Relaxed) {
                    // For non-counters, just go through and eat cake if you haven't already
                    // but do not request more cake if there isn't any
                    is_cake_eaten.store(true, Ordering::Relaxed);
                    has_eaten = true;
                    #[cfg(not(feature = "suppress_guest_action_log"))]
                    println!("[Guest {}]: Ate the Cake", guest_id);
                }

                // Notify that we've left the labyrinth
                #[cfg(not(feature = "suppress_guest_action_log"))]
                println!("[Guest {}]: Left the Labyrinth", guest_id);
                //                num_runthroughs += 1;
                //                if !should_run.load(Ordering::Relaxed) {
                //                    println!("id is_counter count has_eaten num_runthroughs");
                //                }
                labyrinth_tx.send(()).unwrap();
            }
        }));
    }

    // While we are running, send a random guest into labyrinth one at a time.
    while should_run.load(Ordering::Relaxed) {
        let next_guest = thread_rng().gen_range(0..num_guests);
        guest_wol[next_guest as usize].send(()).unwrap();
        labyrinth_rx.recv().unwrap();
    }

    // Stop the guests
    for i in 0..num_guests {
        guest_wol[i as usize].send(()).unwrap();
    }

    // Wait for guests to shutdown
    for guest in guests.into_iter() {
        guest.join().unwrap();
    }
}
