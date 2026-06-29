#[cfg(target_arch = "x86_64")]
use alloc::string::String;
#[cfg(target_arch = "x86_64")]
use crate::{print, println};
#[cfg(target_arch = "x86_64")]
use crate::task::keyboard::ScancodeStream;
#[cfg(target_arch = "x86_64")]
use crate::fs::FS;
#[cfg(target_arch = "x86_64")]
use futures_util::stream::StreamExt;
#[cfg(target_arch = "x86_64")]
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};

#[cfg(target_arch = "x86_64")]
pub async fn run() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );
    let mut line = String::new();

    print!("> ");

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode('\n') => {
                        println!();
                        dispatch(line.trim());
                        line.clear();
                        print!("> ");
                    }
                    DecodedKey::Unicode('\x08') => {
                        if !line.is_empty() {
                            line.pop();
                            crate::platform::backspace();
                        }
                    }
                    DecodedKey::Unicode(c) if c.is_ascii() => {
                        line.push(c);
                        print!("{}", c);
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(target_arch = "x86_64")]
fn dispatch(input: &str) {
    let mut parts = input.splitn(2, ' ');
    let cmd = parts.next().unwrap_or("");
    let arg = parts.next().unwrap_or("").trim();

    match cmd {
        "echo" => println!("{}", arg),
        "ls" => {
            let fs = FS.lock();
            for path in fs.list() {
                println!("{}", path);
            }
        }
        "cat" => {
            let fs = FS.lock();
            match fs.read(arg) {
                Some(data) => match core::str::from_utf8(data) {
                    Ok(s) => println!("{}", s),
                    Err(_) => println!("cat: {}: binary file", arg),
                },
                None => println!("cat: {}: not found", arg),
            }
        }
        "clear" => {
            for _ in 0..25 {
                println!();
            }
        }
        "help" => println!("commands: echo  ls  cat  clear  help"),
        "" => {}
        _ => println!("{}: command not found", cmd),
    }
}
