use log::debug;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::io::BufReader;
use std::{
    cmp::min,
    fs::File,
    sync::{mpsc::Receiver, Arc},
    thread,
    time::Duration,
    time::Instant,
};

mod sound;
use sound::Sound;

// 1m nanos per 1k ms per 60 minutes
const NANOS_PER_MINUTE: u64 = 1_000_000 * 1000 * 60;

const NANOS_PER_BEAT: u64 = NANOS_PER_MINUTE / 120;

const NANOS_PER_TICK: u64 = NANOS_PER_BEAT / 16;

pub enum AudioEvent {
    Bass,
    Kick,
}

struct Pattern {
    note_slots: Vec<u8>,
}

pub fn start(recv: Receiver<AudioEvent>) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let start_time = Instant::now();

    let bass_jab = Sound::load(&include_bytes!("../../assets/bass_jab.wav")[..]);
    let kick = Sound::load(&include_bytes!("../../assets/kick_1.wav")[..]);
    let blip = Sound::load(&include_bytes!("../../assets/blip.wav")[..]);

    let mut current_note = 0;
    let mut current_beat = 0;
    let mut current_bar = 0;

    loop {
        wait_for_tick(start_time);
        let msg = recv.try_recv();
        match msg {
            Ok(AudioEvent::Bass) => {
                stream_handle
                    .play_raw(bass_jab.decoder().convert_samples())
                    .unwrap();
            }
            _ => {}
        }

        if current_note == 0 {
            stream_handle
                .play_raw(kick.decoder().convert_samples())
                .unwrap();
        }

        if current_note == 8 {
            stream_handle
                .play_raw(bass_jab.decoder().convert_samples())
                .unwrap()
        }

        if current_beat == 3 && current_note == 8 {
            let source = blip
                .decoder()
                .buffered()
                .reverb(Duration::from_nanos(NANOS_PER_TICK * 2), 0.1);

            stream_handle.play_raw(source.convert_samples()).unwrap();
        }

        current_note = (current_note + 1) % 16;
        if current_note == 0 {
            current_beat = (current_beat + 1) % 4;
        }
        if current_note == 0 && current_beat == 0 {
            current_bar = (current_bar + 1) % 4;
        }
        debug!("{} note", current_note);
    }
}

fn wait_for_tick(start_time: Instant) {
    let since_start: u64 = Instant::now().duration_since(start_time).as_nanos() as u64;
    let since_last_tick: u64 = since_start % NANOS_PER_TICK;
    let until_next_tick = NANOS_PER_TICK - since_last_tick;

    debug!("{:>8} since last tick", since_last_tick / 1000);
    debug!("{:>8} until next tick", until_next_tick / 1000);
    debug!("");

    thread::sleep(Duration::from_nanos(until_next_tick));
}
