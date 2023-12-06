extern crate clap;

use std::process::exit;
use std::error::Error;

use log::error;

use qlog::QlogSeq;

use std::io::BufReader;

use qlog_dancing_baby::config::AppConfig;

fn main() {
    let rc = run();

    exit(rc);
}

fn run() -> i32 {
    env_logger::builder().init();

    let config = match AppConfig::from_clap() {
        Ok(v) => v,

        Err(e) => {
            error!("Error loading configuration, exiting: {}", e);
            return 1;
        },
    };

    println!("loading qlog as JSON-SEQ...");
    let mark = std::time::Instant::now();

    let file = std::fs::File::open(config.file.clone()).expect("failed to open file");
    let mut reader = BufReader::new(file);

    // Load the file and advance passed the "header" line at the top
    let _qlog = qlog_seq_with_reader(&mut reader).expect("reading qlog failed");
    println!("\tcomplete in {:?}", std::time::Instant::now() - mark);

    println!("parsing qlog events...");
    let mark = std::time::Instant::now();
    // Iterate all remaining qlog events
    while let Some(ev) = read_sqlog_record(&mut reader) {
        let res: Result<qlog::events::Event, serde_json::Error> =
                serde_json::from_slice(&ev);

            match res {
                Ok(event) => {
                    //println!("{:?}", event);
                },

                Err(e) => {
                    error!("Error deserializing: {}", e);
                    error!("input value {}", String::from_utf8_lossy(&ev));

                    // Just swallow the failure and move on
                },
            }
    }
    println!("\tcomplete in {:?}", std::time::Instant::now() - mark);


    0
}

pub fn qlog_seq_with_reader<R: std::io::BufRead>(
    reader: &mut R,
) -> Result<QlogSeq, Box<dyn Error>> {
    // "null record" skip it
    read_sqlog_record(reader);

    let header = read_sqlog_record(reader).ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "oh no!")
    })?;

    let res: Result<QlogSeq, serde_json::Error> = serde_json::from_slice(&header);
    match res {
        Ok(qlog) => Ok(qlog),

        Err(e) => {
            println!("Error deserializing: {}", e);
            println!("input value {}", String::from_utf8_lossy(&header));

            // Just swallow the failure and move on

            Err(e.into())
        },
    }
}

fn read_sqlog_record<R: std::io::BufRead>(reader: &mut R) -> Option<Vec<u8>> {
    let mut buf = Vec::<u8>::new();
    let size = reader.read_until(b'', &mut buf).unwrap();
    if size <= 1 {
        return None;
    }

    buf.truncate(buf.len() - 1);
    /*println!(
        "read record={}",
        String::from_utf8(buf.clone()).expect("from_utf8 failed")
    );*/

    Some(buf)
}