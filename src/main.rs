use std::process::{Command, Stdio};
use uinput::event::Keyboard::All;
use uinput::event::keyboard::Key;
use uinput::device::Device;
use std::thread;
use std::time::Duration;

fn main() {
    let mut clipboard = Command::new("wl-paste")
        .stderr(Stdio::null())
        .output().expect("Failed to get clipboard");
    clipboard.stdout.pop();
    let stdout = String::from_utf8(clipboard.stdout).expect("Invalid UTF-8 in stdout");


	let mut device = uinput::default().unwrap()
		.name("keyboard").unwrap()
		.event(All).unwrap()
		.create().unwrap();

	thread::sleep(Duration::from_secs(1));

    for c in stdout.chars() {
        simulate_key_click(&mut device, &c);
        thread::sleep(Duration::from_millis(1));
    }

	device.synchronize().unwrap();
}

fn simulate_key_click(device: &mut Device, c: &char) {
    match c {
        'a' => device.click(&Key::A).unwrap(),
        'b' => device.click(&Key::B).unwrap(),
        'c' => device.click(&Key::C).unwrap(),
        'd' => device.click(&Key::D).unwrap(),
        'e' => device.click(&Key::E).unwrap(),
        'f' => device.click(&Key::F).unwrap(),
        'g' => device.click(&Key::G).unwrap(),
        'h' => device.click(&Key::H).unwrap(),
        'i' => device.click(&Key::I).unwrap(),
        'j' => device.click(&Key::J).unwrap(),
        'k' => device.click(&Key::K).unwrap(),
        'l' => device.click(&Key::L).unwrap(),
        'm' => device.click(&Key::M).unwrap(),
        'n' => device.click(&Key::N).unwrap(),
        'o' => device.click(&Key::O).unwrap(),
        'p' => device.click(&Key::P).unwrap(),
        'q' => device.click(&Key::Q).unwrap(),
        'r' => device.click(&Key::R).unwrap(),
        's' => device.click(&Key::S).unwrap(),
        't' => device.click(&Key::T).unwrap(),
        'u' => device.click(&Key::U).unwrap(),
        'v' => device.click(&Key::V).unwrap(),
        'w' => device.click(&Key::W).unwrap(),
        'x' => device.click(&Key::X).unwrap(),
        'y' => device.click(&Key::Y).unwrap(),
        'z' => device.click(&Key::Z).unwrap(),
        'A' => with_shift_key(device, &Key::A),
        'B' => with_shift_key(device, &Key::B),
        'C' => with_shift_key(device, &Key::C),
        'D' => with_shift_key(device, &Key::D),
        'E' => with_shift_key(device, &Key::E),
        'F' => with_shift_key(device, &Key::F),
        'G' => with_shift_key(device, &Key::G),
        'H' => with_shift_key(device, &Key::H),
        'I' => with_shift_key(device, &Key::I),
        'J' => with_shift_key(device, &Key::J),
        'K' => with_shift_key(device, &Key::K),
        'L' => with_shift_key(device, &Key::L),
        'M' => with_shift_key(device, &Key::M),
        'N' => with_shift_key(device, &Key::N),
        'O' => with_shift_key(device, &Key::O),
        'P' => with_shift_key(device, &Key::P),
        'Q' => with_shift_key(device, &Key::Q),
        'R' => with_shift_key(device, &Key::R),
        'S' => with_shift_key(device, &Key::S),
        'T' => with_shift_key(device, &Key::T),
        'U' => with_shift_key(device, &Key::U),
        'V' => with_shift_key(device, &Key::V),
        'W' => with_shift_key(device, &Key::W),
        'X' => with_shift_key(device, &Key::X),
        'Y' => with_shift_key(device, &Key::Y),
        'Z' => with_shift_key(device, &Key::Z),
        '`' => device.click(&Key::Grave).unwrap(),
        '1' => device.click(&Key::_1).unwrap(),
        '2' => device.click(&Key::_2).unwrap(),
        '3' => device.click(&Key::_3).unwrap(),
        '4' => device.click(&Key::_4).unwrap(),
        '5' => device.click(&Key::_5).unwrap(),
        '6' => device.click(&Key::_6).unwrap(),
        '7' => device.click(&Key::_7).unwrap(),
        '8' => device.click(&Key::_8).unwrap(),
        '9' => device.click(&Key::_9).unwrap(),
        '0' => device.click(&Key::_0).unwrap(),
        '-' => device.click(&Key::Minus).unwrap(),
        '=' => device.click(&Key::Equal).unwrap(),
        '~' => with_shift_key(device, &Key::Grave),
        '!' => with_shift_key(device, &Key::_1),
        '@' => with_shift_key(device, &Key::_2),
        '#' => with_shift_key(device, &Key::_3),
        '$' => with_shift_key(device, &Key::_4),
        '%' => with_shift_key(device, &Key::_5),
        '^' => with_shift_key(device, &Key::_6),
        '&' => with_shift_key(device, &Key::_7),
        '*' => with_shift_key(device, &Key::_8),
        '(' => with_shift_key(device, &Key::_9),
        ')' => with_shift_key(device, &Key::_0),
        '_' => with_shift_key(device, &Key::Minus),
        '+' => with_shift_key(device, &Key::Equal),
        '[' => device.click(&Key::LeftBrace).unwrap(),
        ']' => device.click(&Key::RightBrace).unwrap(),
        '\\' => device.click(&Key::BackSlash).unwrap(),
        ';' => device.click(&Key::SemiColon).unwrap(),
        '\'' => device.click(&Key::Apostrophe).unwrap(),
        ',' => device.click(&Key::Comma).unwrap(),
        '.' => device.click(&Key::Dot).unwrap(),
        '/' => device.click(&Key::Slash).unwrap(),
        '{' => with_shift_key(device, &Key::LeftBrace),
        '}' => with_shift_key(device, &Key::RightBrace),
        '|' => with_shift_key(device, &Key::BackSlash),
        ':' => with_shift_key(device, &Key::SemiColon),
        '"' => with_shift_key(device, &Key::Apostrophe),
        '<' => with_shift_key(device, &Key::Comma),
        '>' => with_shift_key(device, &Key::Dot),
        '?' => with_shift_key(device, &Key::Slash),
        '\n' => device.click(&Key::Enter).unwrap(),
        _ => {
            let error_msg = format!("Character '{c}' is not implemented");
            Command::new("notify-send")
                .arg(error_msg.clone())
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn().unwrap();
            panic!("{error_msg}");
        }
    }
}

fn with_shift_key(device: &mut Device, key: &Key) {
    device.press(&Key::LeftShift).unwrap();
    device.click(key).unwrap();
    device.release(&Key::LeftShift).unwrap();
}
