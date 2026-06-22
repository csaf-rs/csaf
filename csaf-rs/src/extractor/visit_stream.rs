use std::fmt;
use std::io::Read;

use serde::de::{DeserializeSeed, IgnoredAny, MapAccess, SeqAccess, Visitor};

use crate::extractor::traits::Extractor;

struct RootObjectSeed<'v, 'w> {
    visitors: &'v mut [&'w mut dyn Extractor],
}

impl<'de, 'v, 'w> DeserializeSeed<'de> for RootObjectSeed<'v, 'w> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de, 'v, 'w> Visitor<'de> for RootObjectSeed<'v, 'w> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a JSON object")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        visit_object(&mut map, self.visitors)
    }
}

fn visit_object<'de, A: MapAccess<'de>>(map: &mut A, visitors: &mut [&mut dyn Extractor]) -> Result<(), A::Error> {
    while let Some(key) = map.next_key::<String>()? {
        map.next_value_seed(KeyedValueSeed {
            key: &key,
            visitors: &mut *visitors,
        })?;
    }
    Ok(())
}

fn drain_map<'de, A: MapAccess<'de>>(map: &mut A) -> Result<(), A::Error> {
    while map.next_key::<IgnoredAny>()?.is_some() {
        map.next_value::<IgnoredAny>()?;
    }
    Ok(())
}

fn visit_array<'de, A: SeqAccess<'de>>(seq: &mut A, visitors: &mut [&mut dyn Extractor]) -> Result<(), A::Error> {
    let mut index = 0usize;
    loop {
        let element = seq.next_element_seed(IndexedValueSeed {
            index,
            visitors: &mut *visitors,
        })?;
        if element.is_none() {
            break;
        }
        index += 1;
    }
    Ok(())
}

fn drain_array<'de, A: SeqAccess<'de>>(seq: &mut A) -> Result<(), A::Error> {
    while seq.next_element::<IgnoredAny>()?.is_some() {}
    Ok(())
}

fn emit_keyed(key: &str, value: &serde_json::Value, visitors: &mut [&mut dyn Extractor]) {
    for v in visitors.iter_mut() {
        v.keyed_primitive(key, value);
    }
}

fn emit_indexed(index: usize, value: &serde_json::Value, visitors: &mut [&mut dyn Extractor]) {
    for v in visitors.iter_mut() {
        v.indexed_primitive(index, value);
    }
}

struct KeyedValueSeed<'k, 'v, 'w> {
    key: &'k str,
    visitors: &'v mut [&'w mut dyn Extractor],
}

impl<'de, 'k, 'v, 'w> DeserializeSeed<'de> for KeyedValueSeed<'k, 'v, 'w> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de, 'k, 'v, 'w> Visitor<'de> for KeyedValueSeed<'k, 'v, 'w> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a JSON value")
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<(), E> {
        emit_keyed(self.key, &serde_json::Value::Null, self.visitors);
        Ok(())
    }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<(), E> {
        emit_keyed(self.key, &serde_json::Value::Bool(v), self.visitors);
        Ok(())
    }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<(), E> {
        emit_keyed(self.key, &serde_json::Value::Number(v.into()), self.visitors);
        Ok(())
    }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<(), E> {
        emit_keyed(self.key, &serde_json::Value::Number(v.into()), self.visitors);
        Ok(())
    }
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<(), E> {
        // serde_json never produces NaN/Inf from valid JSON input
        let num = serde_json::Number::from_f64(v).unwrap_or_else(|| 0i64.into());
        emit_keyed(self.key, &serde_json::Value::Number(num), self.visitors);
        Ok(())
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<(), E> {
        emit_keyed(self.key, &serde_json::Value::String(v.to_owned()), self.visitors);
        Ok(())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        let mut interesting = false;
        for v in self.visitors.iter_mut() {
            interesting |= v.enter_keyed_object(self.key);
        }
        let result = if interesting {
            visit_object(&mut map, self.visitors)
        } else {
            drain_map(&mut map)
        };
        for v in self.visitors.iter_mut() {
            v.leave_keyed_object(self.key);
        }
        result
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        let mut interesting = false;
        for v in self.visitors.iter_mut() {
            interesting |= v.enter_keyed_array(self.key);
        }
        let result = if interesting {
            visit_array(&mut seq, self.visitors)
        } else {
            drain_array(&mut seq)
        };
        for v in self.visitors.iter_mut() {
            v.leave_keyed_array(self.key);
        }
        result
    }
}

struct IndexedValueSeed<'v, 'w> {
    index: usize,
    visitors: &'v mut [&'w mut dyn Extractor],
}

impl<'de, 'v, 'w> DeserializeSeed<'de> for IndexedValueSeed<'v, 'w> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de, 'v, 'w> Visitor<'de> for IndexedValueSeed<'v, 'w> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a JSON value")
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<(), E> {
        emit_indexed(self.index, &serde_json::Value::Null, self.visitors);
        Ok(())
    }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<(), E> {
        emit_indexed(self.index, &serde_json::Value::Bool(v), self.visitors);
        Ok(())
    }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<(), E> {
        emit_indexed(self.index, &serde_json::Value::Number(v.into()), self.visitors);
        Ok(())
    }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<(), E> {
        emit_indexed(self.index, &serde_json::Value::Number(v.into()), self.visitors);
        Ok(())
    }
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<(), E> {
        let num = serde_json::Number::from_f64(v).unwrap_or_else(|| 0i64.into());
        emit_indexed(self.index, &serde_json::Value::Number(num), self.visitors);
        Ok(())
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<(), E> {
        emit_indexed(self.index, &serde_json::Value::String(v.to_owned()), self.visitors);
        Ok(())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        let mut interesting = false;
        for v in self.visitors.iter_mut() {
            interesting |= v.enter_indexed_object(self.index);
        }
        let result = if interesting {
            visit_object(&mut map, self.visitors)
        } else {
            drain_map(&mut map)
        };
        for v in self.visitors.iter_mut() {
            v.leave_indexed_object(self.index);
        }
        result
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        let mut interesting = false;
        for v in self.visitors.iter_mut() {
            interesting |= v.enter_indexed_array(self.index);
        }
        let result = if interesting {
            visit_array(&mut seq, self.visitors)
        } else {
            drain_array(&mut seq)
        };
        for v in self.visitors.iter_mut() {
            v.leave_indexed_array(self.index);
        }
        result
    }
}

/// Traverses a streaming JSON reader and applies the provided extractors to it.
pub fn visit_stream<R: Read>(reader: R, visitors: &mut [&mut dyn Extractor]) -> Result<(), serde_json::Error> {
    let mut de = serde_json::Deserializer::from_reader(reader);
    let seed = RootObjectSeed { visitors };

    DeserializeSeed::deserialize(seed, &mut de)
}
