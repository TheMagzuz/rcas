use std::{fs::File, io::BufReader, path::Path};
use std::collections::HashMap;
use std::time::Duration;
use anyhow::anyhow;
use quick_xml::events::BytesStart;
use quick_xml::{events::Event, reader::Reader, name::QName};

use crate::levels::{Chapter, Side};

pub type TimeMap = HashMap<Chapter, AreaModeStats>;

pub fn load_save(path: &Path) -> anyhow::Result<TimeMap> {
    let mut data = HashMap::new();
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let mut reader = Reader::from_reader(buf_reader);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut side_index = 0;
    let mut chapter_index: Option<u8> = None;
    let mut in_areas = false;

    loop {
        let event = reader.read_event_into(&mut buf)?;
        match event {
            Event::Eof => break,
            Event::Start(tag) => {
                match tag.name().as_ref() {
                    b"Areas" => in_areas = true,
                    b"AreaStats" if in_areas => {
                        side_index = 0;
                        let chapter = find_attr(b"ID", &tag)?;
                        chapter_index = Some(chapter.parse::<u8>()?);
                    }
                    b"AreaModeStats" if in_areas => {
                        let chapter_index = chapter_index.ok_or(anyhow!("Reached an AreaModeStats tag without a chapter index being set"))?;
                        let chapter = Chapter::from_index(chapter_index, Side::from_index(side_index)?)?;
                        side_index += 1;

                        let time_played = find_attr(b"TimePlayed", &tag)?.parse::<u64>()?;
                        if time_played == 0 {
                            continue;
                        }
                        let best_time = find_attr(b"BestTime", &tag)?.parse::<u64>()?;

                        data.insert(chapter, AreaModeStats::from_u64(time_played, best_time));
                    }
                    _ => (),
                }
            },
            Event::End(tag) => {
                if tag.name().as_ref() == b"Areas" {
                    in_areas = false;
                }
            }
            _ => (),
        }
    }
    buf.clear();
    Ok(data)
}

fn find_attr(name: &[u8], tag: &BytesStart) -> anyhow::Result<String> {
    let target_name = QName(name);

    let attr = tag.attributes().find(|a| {
        if let Ok(a) = a {
            a.key == target_name
        } else {
            false
        }
    });

    if let Some(attr) = attr {
        return Ok(std::str::from_utf8(attr?.value.as_ref())?.to_owned());
    } else {
        return Err(anyhow!("Could not find attribute {} on tag {}", std::str::from_utf8(name).unwrap(), std::str::from_utf8(tag.name().as_ref()).unwrap()));
    }
}

#[derive(Debug, Clone)]
pub struct AreaModeStats {
    pub time_played: Duration,
    pub best_time: Duration,
}

impl AreaModeStats {
    pub fn from_u64(time_played: u64, best_time: u64) -> Self {
        Self {
            time_played: Duration::from_micros(time_played/10),
            best_time: Duration::from_micros(best_time/10),
        }
    }
}
