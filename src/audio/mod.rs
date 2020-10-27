use std::{fs::File, sync::{Arc, mpsc::Receiver}};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

mod sound;
use sound::Sound;

pub enum AudioEvent {
    Bass
}

pub fn start(recv: Receiver<AudioEvent>) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    let bass_jab = sound::Sound::load("assets/bass_jab.wav").unwrap();

    loop {
        let msg = recv.recv().unwrap();
        match msg {
            AudioEvent::Bass => {
                sink.append(bass_jab.decoder())
            }
        }
    }
}
