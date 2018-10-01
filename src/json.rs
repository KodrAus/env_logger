use std::io::{self, Write};
use serde::ser::{Serialize, Serializer, SerializeMap, Error};
use serde_json;
use log::{Level, Record};
use log::key_values::KeyValueSource;

use super::Builder;
use super::fmt::{Formatter, Timestamp};

impl Builder {
    /// Format logs as json.
    pub fn json_format(&mut self) -> &mut Self {
        self.format(json)
    }
}

fn json(mut f: &mut Formatter, r: &Record) -> io::Result<()> {
    let r = SerializeRecord {
        level: r.level(),
        timestamp: f.timestamp(),
        msg: r.args().to_string(),
        props: r.key_values(),
    };

    serde_json::to_writer(&mut f, &r)?;
    writeln!(&mut f)?;

    Ok(())
}

struct SerializeRecord<KVS> {
    level: Level,
    timestamp: Timestamp,
    msg: String,
    props: KVS,
}

impl<KVS> Serialize for SerializeRecord<KVS>
where
    KVS: KeyValueSource,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("timestamp", &self.timestamp.to_string())?;
        map.serialize_entry("level", &self.level)?;
        map.serialize_entry("msg", &self.msg)?;

        self.props
            .as_ref()
            .sort_retain_last()
            .try_for_each(|k, v| map.serialize_entry(&k, &v))
            .map_err(S::Error::custom)?;

        map.end()
    }
}