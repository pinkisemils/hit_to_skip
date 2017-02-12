extern crate mpv;
extern crate serial;
// use mpv::mpv;
use std::env;
use std::path::Path;
use std::thread::sleep_ms;
use std::thread::spawn;
use std::sync::mpsc;
use std::io;
use std::time;
use std::io::prelude::*;
use serial::prelude::*;

// fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
//    try!(port.reconfigure(&|settings| {
//        try!(settings.set_baud_rate(serial::Baud9600));
//        settings.set_char_size(serial::Bits8);
//        settings.set_parity(serial::ParityNone);
//        settings.set_stop_bits(serial::Stop1);
//        settings.set_flow_control(serial::FlowNone);
//        Ok(())
//    }));
//
//    try!(port.set_timeout(Duration::from_millis(1000)));
//
//    let mut buf: Vec<u8> = (0..255).collect();
//
//    try!(port.write(&buf[..]));
//    try!(port.read(&mut buf[..]));
//
//    Ok(())
// }

fn read_serial(path: String, tx: mpsc::Sender<PlayState>) {
    let mut port = serial::open(&path).expect("What the heckin' heck, can't even open up a TTY");
    port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud9600).expect("Couldn't even set the baud rate");
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })
        .expect("Couldn't even configure the damn tty");
    port.set_timeout(time::Duration::from_millis(1 * 1000)).expect("Couldn't even set the timeout");
    loop {
        let mut input_buf = vec![0x0u8];
        match port.read(&mut input_buf) {
            Ok(_) => {
                match input_buf[0] as char {
                    'S' => {
                        println!("Received an 's'");
                        match tx.send(PlayState::Skip) {
                            Ok(_) => continue,
                            Err(e) => return,
                        }
                    }
                    anything => {
                        //println!("Received a '{}'",anything );
                        continue;
                    },
                }
            }
            Err(e) => {
                println!("Encountered e: {}", e);
                println!("It's time to stop");
                return;

            }

        }
    }

}


enum PlayState {
    PauseResume,
    Skip,
    Stop,
}

fn simple_example(file_path: &Path, rx: &mpsc::Receiver<PlayState>) -> bool {
    let mut mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");
    if file_path.is_file() {
        let file_path = file_path.to_str().expect("Expected a string for Path, got None");

        // set option "sid" to "no" (no subtitles)
        // mpv options should be set before initializing
        mpv_builder.set_option("sid", "no").unwrap();

        let mut mpv = mpv_builder.build().expect("Failed to build MPV handler");

        mpv.command(&["loadfile", file_path as &str])
            .expect("Error loading file");

        // loop twice, send parameter as a string

        // set speed to 100%, send parameter as a f64
        mpv.set_property("speed", 1.0).unwrap();

        // get how many loops are playing as an i64
        // sleep_ms(3000);
        // mpv.set_property_async("pause", true, 1).expect("Failed to pause player");

        let mut playing = true;
        let mut stop = false;
        'main: loop {
            while let Some(event) = mpv.wait_event(0.100) {
                // even if you don't do anything with the events, it is still necessary to empty
                // the event loop
                match event {
                    // Shutdown will be triggered when the window is explicitely closed,
                    // while Idle will be triggered when the queue will end
                    // mpv::Event::Shutdown | mpv::Event::Idle => {
                    mpv::Event::EndFile(_) => {
                        stop = false;
                        break 'main;
                    }
                    mpv::Event::Shutdown => {
                        break 'main;
                    }
                    _ => {}
                };
            }
            match rx.try_recv() {
                Err(mpsc::TryRecvError::Empty) => continue,
                Err(mpsc::TryRecvError::Disconnected) => break,
                Ok(command) => {
                    match command {
                        PlayState::PauseResume => {
                            mpv.set_property("pause", playing).expect("Failed to pause/resume");
                            playing = !playing;
                        }
                        PlayState::Skip => {
                            break;
                        }
                        PlayState::Stop => {
                            stop = true;
                            break;
                        }
                    }
                }
            };
        }
        match mpv.command(&["quit", "0"]) {
            Err(e) => println!("Failed to quit current mpv instance: {}", e),
            _ => (),
        }
        return stop;

    } else {
        println!("A file is required; {} is not a valid file",
                 file_path.to_str().unwrap());
        return false;
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let (mpv_major_api_v, mpv_minor_api_v) = mpv::client_api_version();
    println!("MPV_API_VERSION : v{}.{}", mpv_major_api_v, mpv_minor_api_v);
    if args.len() < 2 {
        println!("Usage: ./simple [any mp4, avi, mkv, ... file]");
    } else {
        let files: Vec<String> = args[1..].to_vec();
        let (tx, rx) = mpsc::channel();

        let player_thread_handle = spawn(move || {

            loop {
                for path in files.iter() {
                    let path: &Path = Path::new(path);
                    if simple_example(path, &rx) {
                        return;
                    };

                }
            }

        });
        //let mut input = String::new();
        //let tty_path = std::io::stdin().read_line(&mut input).expect("Failed to read tty path");
        let tty_path = "/dev/ttyACM1".to_string();

        let s_tx = tx.clone();
        let s_handle = spawn(move || loop {read_serial(tty_path.clone(), s_tx.clone())});
        // code that will wait for logic shit to come in and tell if it should skip or not
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim();
                    match input.as_ref() {
                        "q" => {
                            tx.send(PlayState::Stop)
                                .expect("failed to send command to player chan");
                            break;
                        }
                        "p" => {
                            tx.send(PlayState::PauseResume)
                                .expect("failed to send command to player chan")
                        }
                        _ => {
                            println!("Received {:?}", input);
                            println!("Expected commands are \n s - skip\n q - quit\n p - \
                                      pause/play\n");
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to read stdin because {}", e);
                    break;
                }

            }

        }
        player_thread_handle.join().expect("failed to join player thread");
        s_handle.join().expect("failed to join player thread");

    }
}
