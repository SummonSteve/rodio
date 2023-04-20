use cpal::traits::{DeviceTrait, HostTrait};
use rodio::OutputStream;
use std::io::BufReader;

fn main() {
    let device = cpal::available_hosts();

    let host = cpal::host_from_id(cpal::HostId::Asio).unwrap();
    let device = host.default_output_device().unwrap();

    let (_stream, handle) = OutputStream::try_from_device(&device).unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let file = std::fs::File::open("assets/music.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}
