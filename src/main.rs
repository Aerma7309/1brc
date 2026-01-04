use std::{
    collections::{self, BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader, Write},
};

use crate::types::TemperatureEntry;

mod types;

fn main() {
    let mut resp: HashMap<String, TemperatureEntry> = HashMap::new();

    let filename = "measurements.txt";
    let mut buf = vec![];
    let mut line = String::new();

    let reader = File::open(filename).expect("Failed to open file");
    let mut buf_reader = BufReader::new(reader);

    loop {
        let cr = buf_reader.read_until(b'\n', &mut buf).unwrap();
        if cr != 0 {
            line = String::from_utf8(buf[..cr - 1].to_vec()).unwrap();
            buf.clear();
            let city_temp: TemperatureEntry = line.parse().expect("Failed to parse line");
            resp.entry(city_temp.city_name.clone())
                .and_modify(|oct| *oct = oct.clone() + &city_temp)
                .or_insert(city_temp);
        } else {
            break;
        }
    }
    let outfile = "measurements.out";
    let mut writer = File::create(outfile).expect("Failed to create file");
    writer.write_all(b"{").expect("Failed to write to file");
    let mut first = true;
    let final_resp: BTreeMap<String, TemperatureEntry> = BTreeMap::from_iter(resp);
    for (_, val) in final_resp.iter() {
        if !first {
            writer.write_all(b", ").expect("Failed to write to file");
        }
        first = false;
        writer
            .write_all(
                format!(
                    "{}={:.1}/{:.1}/{:.1}",
                    val.city_name,
                    val.min_temperature,
                    val.temperature / val.count as f32,
                    val.max_temperature,
                )
                .as_bytes(),
            )
            .expect("Failed to write to file");
    }
    writer.write_all(b"}\n").expect("Failed to write to file");
    writer.flush().expect("Failed to flush writer");
}
