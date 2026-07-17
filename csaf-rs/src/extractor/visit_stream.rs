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
        write!(f, "a JSON value")
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<(), E> {
        emit_init(&serde_json::Value::Null, self.visitors);
        Ok(())
    }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<(), E> {
        emit_init(&serde_json::Value::Bool(v), self.visitors);
        Ok(())
    }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<(), E> {
        emit_init(&serde_json::Value::Number(v.into()), self.visitors);
        Ok(())
    }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<(), E> {
        emit_init(&serde_json::Value::Number(v.into()), self.visitors);
        Ok(())
    }
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<(), E> {
        // serde_json never produces NaN/Inf from valid JSON input
        let num = serde_json::Number::from_f64(v).unwrap_or_else(|| 0i64.into());
        emit_init(&serde_json::Value::Number(num), self.visitors);
        Ok(())
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<(), E> {
        emit_init(&serde_json::Value::String(v.to_owned()), self.visitors);
        Ok(())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        for v in self.visitors.iter_mut() {
            v.init_object("");
        }
        visit_object(&mut String::with_capacity(256), &mut map, self.visitors)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        for v in self.visitors.iter_mut() {
            v.init_array("");
        }
        visit_array(&mut String::with_capacity(256), &mut seq, self.visitors)
    }
}

fn visit_object<'de, A: MapAccess<'de>>(
    json_pointer: &mut String,
    map: &mut A,
    visitors: &mut [&mut dyn Extractor],
) -> Result<(), A::Error> {
    while let Some(key) = map.next_key::<String>()? {
        let old_len = json_pointer.len();
        json_pointer.push('/');
        json_pointer.push_str(&key);
        let result = map.next_value_seed(KeyedValueSeed {
            json_pointer,
            key: &key,
            visitors: &mut *visitors,
        });
        json_pointer.truncate(old_len);
        result?;
    }
    Ok(())
}

fn drain_map<'de, A: MapAccess<'de>>(map: &mut A) -> Result<(), A::Error> {
    while map.next_key::<IgnoredAny>()?.is_some() {
        map.next_value::<IgnoredAny>()?;
    }
    Ok(())
}

fn visit_array<'de, A: SeqAccess<'de>>(
    json_pointer: &mut String,
    seq: &mut A,
    visitors: &mut [&mut dyn Extractor],
) -> Result<(), A::Error> {
    let mut index = 0usize;
    loop {
        let old_len = json_pointer.len();
        json_pointer.push('/');
        json_pointer.push_str(&index.to_string());
        let element = seq.next_element_seed(IndexedValueSeed {
            json_pointer,
            index,
            visitors: &mut *visitors,
        });
        json_pointer.truncate(old_len);
        if element?.is_none() {
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

fn emit_init(value: &serde_json::Value, visitors: &mut [&mut dyn Extractor]) {
    for v in visitors.iter_mut() {
        v.init_primitive("", value);
    }
}

fn emit_keyed(json_pointer: &str, key: &str, value: &serde_json::Value, visitors: &mut [&mut dyn Extractor]) {
    for v in visitors.iter_mut() {
        v.keyed_primitive(json_pointer, key, value);
    }
}

fn emit_indexed(json_pointer: &str, index: usize, value: &serde_json::Value, visitors: &mut [&mut dyn Extractor]) {
    for v in visitors.iter_mut() {
        v.indexed_primitive(json_pointer, index, value);
    }
}

struct KeyedValueSeed<'k, 'p, 'v, 'w> {
    json_pointer: &'p mut String,
    key: &'k str,
    visitors: &'v mut [&'w mut dyn Extractor],
}

impl<'de, 'k, 'p, 'v, 'w> DeserializeSeed<'de> for KeyedValueSeed<'k, 'p, 'v, 'w> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de, 'k, 'p, 'v, 'w> Visitor<'de> for KeyedValueSeed<'k, 'p, 'v, 'w> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a JSON value")
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<(), E> {
        emit_keyed(self.json_pointer, self.key, &serde_json::Value::Null, self.visitors);
        Ok(())
    }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<(), E> {
        emit_keyed(self.json_pointer, self.key, &serde_json::Value::Bool(v), self.visitors);
        Ok(())
    }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<(), E> {
        emit_keyed(
            self.json_pointer,
            self.key,
            &serde_json::Value::Number(v.into()),
            self.visitors,
        );
        Ok(())
    }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<(), E> {
        emit_keyed(
            self.json_pointer,
            self.key,
            &serde_json::Value::Number(v.into()),
            self.visitors,
        );
        Ok(())
    }
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<(), E> {
        // serde_json never produces NaN/Inf from valid JSON input
        let num = serde_json::Number::from_f64(v).unwrap_or_else(|| 0i64.into());
        emit_keyed(
            self.json_pointer,
            self.key,
            &serde_json::Value::Number(num),
            self.visitors,
        );
        Ok(())
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<(), E> {
        emit_keyed(
            self.json_pointer,
            self.key,
            &serde_json::Value::String(v.to_owned()),
            self.visitors,
        );
        Ok(())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        let mut interesting = false;
        for v in self.visitors.iter_mut() {
            interesting |= v.enter_keyed_object(self.json_pointer, self.key);
        }
        let result = if interesting {
            visit_object(self.json_pointer, &mut map, self.visitors)
        } else {
            drain_map(&mut map)
        };
        for v in self.visitors.iter_mut() {
            v.leave_keyed_object(self.json_pointer, self.key);
        }
        result
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        let mut interesting = false;
        for v in self.visitors.iter_mut() {
            interesting |= v.enter_keyed_array(self.json_pointer, self.key);
        }
        let result = if interesting {
            visit_array(self.json_pointer, &mut seq, self.visitors)
        } else {
            drain_array(&mut seq)
        };
        for v in self.visitors.iter_mut() {
            v.leave_keyed_array(self.json_pointer, self.key);
        }
        result
    }
}

struct IndexedValueSeed<'p, 'v, 'w> {
    json_pointer: &'p mut String,
    index: usize,
    visitors: &'v mut [&'w mut dyn Extractor],
}

impl<'de, 'p, 'v, 'w> DeserializeSeed<'de> for IndexedValueSeed<'p, 'v, 'w> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de, 'p, 'v, 'w> Visitor<'de> for IndexedValueSeed<'p, 'v, 'w> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a JSON value")
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<(), E> {
        emit_indexed(self.json_pointer, self.index, &serde_json::Value::Null, self.visitors);
        Ok(())
    }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<(), E> {
        emit_indexed(
            self.json_pointer,
            self.index,
            &serde_json::Value::Bool(v),
            self.visitors,
        );
        Ok(())
    }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<(), E> {
        emit_indexed(
            self.json_pointer,
            self.index,
            &serde_json::Value::Number(v.into()),
            self.visitors,
        );
        Ok(())
    }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<(), E> {
        emit_indexed(
            self.json_pointer,
            self.index,
            &serde_json::Value::Number(v.into()),
            self.visitors,
        );
        Ok(())
    }
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<(), E> {
        let num = serde_json::Number::from_f64(v).unwrap_or_else(|| 0i64.into());
        emit_indexed(
            self.json_pointer,
            self.index,
            &serde_json::Value::Number(num),
            self.visitors,
        );
        Ok(())
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<(), E> {
        emit_indexed(
            self.json_pointer,
            self.index,
            &serde_json::Value::String(v.to_owned()),
            self.visitors,
        );
        Ok(())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        let mut interesting = false;
        for v in self.visitors.iter_mut() {
            interesting |= v.enter_indexed_object(self.json_pointer, self.index);
        }
        let result = if interesting {
            visit_object(self.json_pointer, &mut map, self.visitors)
        } else {
            drain_map(&mut map)
        };
        for v in self.visitors.iter_mut() {
            v.leave_indexed_object(self.json_pointer, self.index);
        }
        result
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        let mut interesting = false;
        for v in self.visitors.iter_mut() {
            interesting |= v.enter_indexed_array(self.json_pointer, self.index);
        }
        let result = if interesting {
            visit_array(self.json_pointer, &mut seq, self.visitors)
        } else {
            drain_array(&mut seq)
        };
        for v in self.visitors.iter_mut() {
            v.leave_indexed_array(self.json_pointer, self.index);
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

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::extractor::{
        extract::{ExtractJsonValue, ExtractPrimitive},
        navigate::AtPath,
        traits::CanExtract,
    };

    use super::*;

    #[test]
    fn json_object_at_top_level() {
        let interesting_object = json!({"p": null, "o": {}, "a": [null, {}, []]});
        let json = serde_json::to_string(&interesting_object).unwrap();

        let mut collector = ExtractJsonValue::new();
        let parse_result = visit_stream(json.as_bytes(), &mut [&mut collector]);
        parse_result.expect("parsing should succeed");

        let result = collector.extract();
        assert_eq!(result, Some(("".into(), interesting_object)));
    }

    #[test]
    fn json_object_at_path() {
        let interesting_object = json!({"p": null, "o": {}, "a": [null, {}, []]});
        let json = serde_json::to_string(&json!({"x": interesting_object, "y": false})).unwrap();

        let mut collector = AtPath::new("x", ExtractJsonValue::new());
        let parse_result = visit_stream(json.as_bytes(), &mut [&mut collector]);
        parse_result.expect("parsing should succeed");

        let result = collector.extract();
        assert_eq!(result, Some(("/x".into(), interesting_object)));
    }

    #[test]
    fn json_array_at_top_level() {
        let interesting_object = json!([{"p": null, "o": {}, "a": [null, {}, []]}]);
        let json = serde_json::to_string(&interesting_object).unwrap();

        let mut collector = ExtractJsonValue::new();
        let parse_result = visit_stream(json.as_bytes(), &mut [&mut collector]);
        parse_result.expect("parsing should succeed");

        let result = collector.extract();
        assert_eq!(result, Some(("".into(), interesting_object)));
    }

    #[test]
    fn json_primitive_at_top_level() {
        let interesting_object = json!("hello");
        let json = serde_json::to_string(&interesting_object).unwrap();

        let mut collector = ExtractJsonValue::new();

        let parse_result = visit_stream(json.as_bytes(), &mut [&mut collector]);
        parse_result.expect("parsing should succeed");

        let result = collector.extract();
        assert_eq!(result, Some(("".into(), interesting_object)));
    }

    #[test]
    fn two_primitives() {
        let mut x = AtPath::new("x", ExtractPrimitive::new_string_with_path());
        let mut y = AtPath::new("y", ExtractPrimitive::new_bool_with_path());

        let parse_result = visit_stream(&br#"{"x": "a", "y": true}"#[..], &mut [&mut x, &mut y]);
        parse_result.expect("parsing should succeed");

        let result = (x.extract(), y.extract());
        assert_eq!(result, (Some(("/x".into(), "a".into())), Some(("/y".into(), true))));
    }

    #[test]
    fn truncated_json_object_at_top_level() {
        let interesting_object = json!({"p": "x", "a": false, "b": []});
        let json = br#"{"p": "x", "a":false, "b": [], "c": //"#;

        let mut collector = ExtractJsonValue::new();
        let parse_result = visit_stream(&json[..], &mut [&mut collector]);
        parse_result.expect_err("parsing should fail");

        let result = collector.extract();
        assert_eq!(result, Some(("".into(), interesting_object)));
    }
}
