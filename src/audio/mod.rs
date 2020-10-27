use log::debug;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::iter::FromIterator;
use std::{
    cmp::min,
    collections::HashMap,
    fs::File,
    sync::{mpsc::Receiver, Arc},
    thread,
    time::Duration,
    time::Instant,
};
use std::{collections::HashSet, collections::VecDeque, io::BufReader};

mod sound;
use sound::Sound;

// 1m nanos per 1k ms per 60 minutes
const NANOS_PER_MINUTE: usize = 1_000_000 * 1000 * 60;

const NANOS_PER_BEAT: usize = NANOS_PER_MINUTE / 120;

const NOTES_PER_BEAT: usize = 4;
const NANOS_PER_TICK: usize = NANOS_PER_BEAT / NOTES_PER_BEAT;

type Queue = VecDeque<HashSet<AudioEvent>>;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum AudioEvent {
    NewStation,
    ShipArrived,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Sample {
    Blip,
    Bass,
    Kick,
}

pub fn start(recv: Receiver<AudioEvent>) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let start_time = Instant::now();
    let mut queue = init_queue();

    let sounds: HashMap<Sample, Sound> = (vec![
        (
            Sample::Bass,
            Sound::load(&include_bytes!("../../assets/bass_jab.wav")[..]),
        ),
        (
            Sample::Kick,
            Sound::load(&include_bytes!("../../assets/kick_1.wav")[..]),
        ),
        (
            Sample::Blip,
            Sound::load(&include_bytes!("../../assets/blip.wav")[..]),
        ),
    ])
    .into_iter()
    .collect();

    loop {
        let event = recv.recv().unwrap();

        if queue.pop_front().unwrap().contains(&event) {
            queue_item(&mut queue, event);
        } else {
            play_event(&event, start_time, &stream_handle, &sounds)
        }

        thread::sleep(time_until_tick(start_time));
        for events in queue.pop_front() {
            for event in events.iter() {
                play_event(event, start_time, &stream_handle, &sounds);
            }
        }

        queue.push_back(HashSet::<AudioEvent>::new());
    }
}

fn play_event(
    event: &AudioEvent,
    start_time: Instant,
    stream_handle: &OutputStreamHandle,
    sounds: &HashMap<Sample, Sound>,
) {
    let sound = match event {
        AudioEvent::NewStation => sounds.get(&Sample::Bass),
        AudioEvent::ShipArrived => sounds.get(&Sample::Blip),
    }
    .unwrap();

    let src = sound.decoder().delay(time_until_tick(start_time));

    stream_handle.play_raw(src.convert_samples()).unwrap();
}

fn queue_item(queue: &mut Queue, event: AudioEvent) {
    for set in queue.iter_mut() {
        if !set.contains(&event) {
            set.insert(event.clone());
            return;
        }
    }
}

fn init_queue() -> Queue {
    let mut queue = VecDeque::with_capacity(NOTES_PER_BEAT);
    [0..NOTES_PER_BEAT].iter().for_each(|_| {
        queue.push_back(HashSet::<AudioEvent>::new());
    });
    queue
}

fn time_until_tick(start_time: Instant) -> Duration {
    let since_start = Instant::now().duration_since(start_time).as_nanos() as usize;
    let since_last_tick = since_start % NANOS_PER_TICK;
    let until_next_tick = NANOS_PER_TICK - since_last_tick;

    debug!("{:>8} since last tick", since_last_tick / 1000);
    debug!("{:>8} until next tick", until_next_tick / 1000);
    debug!("");

    return Duration::from_nanos(until_next_tick as u64);
}
