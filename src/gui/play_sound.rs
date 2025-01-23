use std::{fs::File, thread};

use rodio::{Decoder, OutputStream, Sink};

use crate::types::item_filter::PlaySound;



pub fn play_sound(play_sound: Option<PlaySound>) {
    let file_name = play_sound.unwrap().custom_sound_file;
    match file_name {
        Some(file_name) => {
            log::info!("Playing custom sound: {:?}", file_name);
            thread::spawn(move || {
                // Create an output stream for audio playback
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let sink = Sink::try_new(&stream_handle).unwrap();
        
                match File::open(file_name) {
                    Ok(file) => {
                        match Decoder::new(file) {
                            Ok(source) => {
                                sink.append(source);
                                sink.sleep_until_end();
                            }
                            Err(e) => {
                                log::info!("Error decoding audio file: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        log::info!("Error opening sound file: {}", e);
                    }
                }
            });
        }
        None => {
            log::info!("Playing no sound");
        }
    }
}