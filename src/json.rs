use std::io::{self, Write};
use serde_json;
use log::{Level, Record};
use log::key_values::Source;

use super::Builder;
use super::fmt::Formatter;

impl Builder {
    /// Format logs as json.
    pub fn json_format(&mut self) -> &mut Self {
        self.format(json)
    }
}

fn json(mut f: &mut Formatter, r: &Record) -> io::Result<()> {
    let r = SerializeRecord {
        level: r.level(),
        timestamp: f.timestamp().to_string(),
        msg: r.args().to_string(),
        props: r.key_values().serialize_as_map(),
    };

    serde_json::to_writer(&mut f, &r)?;
    writeln!(&mut f)?;

    Ok(())
}

#[derive(Serialize)]
struct SerializeRecord<KVS> {
    level: Level,
    timestamp: String,
    msg: String,
    #[serde(flatten)]
    props: KVS,
}
