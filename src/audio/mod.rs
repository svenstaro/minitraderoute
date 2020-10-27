use std::{time::Duration, fs::File, sync::{Arc, mpsc::Receiver}, thread, time::Instant};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

mod sound;
use sound::Sound;

// 1m nanos per 1k ms per 60 minutes
const NANOS_PER_MINUTE: u64 = 1_000_000 * 1000 * 60;

// 124 bpm
const NANOS_PER_BEAT: u64 = NANOS_PER_MINUTE / 124;

pub enum AudioEvent {
    Bass
}

pub fn start(recv: Receiver<AudioEvent>) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    let start_time = Instant::now();

    let bass_jab = 
        sound::Sound::load(&include_bytes!("../../assets/bass_jab.wav")[..]);

    loop {
        wait_for_tick(start_time);
        let msg = recv.try_recv();
        match msg {
            Ok(AudioEvent::Bass) => {
                sink.append(bass_jab.decoder())
            },
            _ => {}
        }

        sink.append(bass_jab.decoder());
    }
}

fn wait_for_tick(start_time: Instant) {
    let since_start: u64 = Instant::now().duration_since(start_time).as_nanos() as u64;
    let since_last_beat: u64 = since_start % NANOS_PER_BEAT;
    let until_next_beat = NANOS_PER_BEAT - since_last_beat;
    println!("{:>8} since start", since_start / 1000);
    println!("{:>8} since last tick", since_last_beat / 1000);
    println!("{:>8} until next tick", until_next_beat / 1000);
    println!("");
    thread::sleep(Duration::from_nanos(until_next_beat));
}
