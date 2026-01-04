use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::Write,
    os::fd::AsRawFd,
};

use crate::types::TemperatureEntry;

mod types;

fn main() {
    let mut resp: HashMap<String, TemperatureEntry> = HashMap::with_capacity(10000);
    let filename = "measurements.txt";
    let reader = File::open(filename).expect("Failed to open file");
    let buf_reader = use_memory_mapping(&reader);

    for line in buf_reader.split(|c| *c == b'\n') {
        let city_temp: TemperatureEntry =
            unsafe { str::from_utf8_unchecked(line) }.parse().unwrap();
        let curr_city_temp = match resp.get_mut(&city_temp.city_name) {
            Some(curr_city_temp) => curr_city_temp,
            None => resp
                .entry(city_temp.city_name.clone())
                .or_insert_with(|| city_temp.clone()),
        };
        *curr_city_temp = city_temp + &curr_city_temp;
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

fn use_memory_mapping(f: &File) -> &'_ [u8] {
    let len = f.metadata().unwrap().len();
    unsafe {
        let ptr = libc::mmap(
            std::ptr::null_mut(),
            len as libc::size_t,
            libc::PROT_READ,
            libc::MAP_SHARED,
            f.as_raw_fd(),
            0,
        );

        if ptr == libc::MAP_FAILED {
            panic!("{:?}", std::io::Error::last_os_error());
        }
        if libc::madvise(ptr, len as libc::size_t, libc::MADV_SEQUENTIAL) != 0 {
            panic!("{:?}", std::io::Error::last_os_error())
        }
        // ignore last new line
        std::slice::from_raw_parts(ptr as *const u8, (len - 1) as usize)
    }
}
