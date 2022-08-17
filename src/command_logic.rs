// #![no_std]
use std::thread::sleep;
use core::time::{Duration};
use crate::cli_def::*;
use crate::win32_sound::*;

pub fn eq_logic (_: EqArgs) {
    let device = default_device().unwrap();
    let device_name = device_name(&device).unwrap();
    let sess_iter = SessionIterator::new(&device);
    let sess_iter_size = sess_iter.size;
    
    for sess in sess_iter {
        set_session_volume(&sess, 1.0).unwrap();
    }
    
    println!("{} sound sessions equalized for {}.", sess_iter_size, device_name);
}

pub fn mute_logic (args: MuteArgs) {
    let title = args.title;
    let duration = args.duration;
    
    let device = default_device().unwrap();
    let sess_iter = SessionIterator::new(&device);
    
    let mut found = false;
    for sess2 in sess_iter {
        let matches = unwrap_or_continue!(session_matches(&title, &sess2), "matches");
        
        if matches {
            let d = Duration::from_secs(duration);
            println!("{}, shut it for {} seconds!", title, duration);
            mute_session(&sess2);
            sleep(d);
            unmute_session(&sess2);
            found = true;
        }
    }
    
    
    if !found {
        println!("Could not find {title} to mute.");
    }
}

