use std::io::Write;

use super::fmt::*;

use log::kv::Source;

impl Formatter {
    fn property_style(&self) -> Style {
        let mut property_style = self.style();
        property_style.set_bold(true);

        property_style
    }

    /// Write key value pairs.
    pub fn write_key_values<KVS>(&mut self, kvs: KVS)
    where
        KVS: Source,
    {
        let _ = writeln!(self);
        let _ = kvs.try_for_each(|k, v| {
            let property_style = self.property_style();
            writeln!(self, "{}: {:?}", property_style.value(k), v)
        });
    }
}
