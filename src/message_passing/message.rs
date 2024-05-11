use chrono::{DateTime, Local};
use std::any::Any;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use chrono::prelude::*;


#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(serialize_with = "serialize_box_any", deserialize_with = "deserialize_box_any")]
    content: Box<dyn Any>,
    #[serde(serialize_with = "serialize_hashmap_box_any", deserialize_with = "deserialize_hashmap_box_any")]
    params: HashMap<String, Box<dyn Any>>,
    #[serde(with = "date_time_serializer")]
    last_update: DateTime<Local>,
}

mod date_time_serializer {
    use chrono::{DateTime, Local};
    use serde::{Serializer, Deserializer};

    pub fn serialize<S>(date_time: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date_time.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map_err(serde::de::Error::custom)
            .map(|dt| dt.with_timezone(&Local))
    }
}

fn serialize_box_any<S>(value: &Box<dyn Any>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // Attempt to downcast the inner value to a concrete type
    match value.downcast_ref::<serde_json::Value>() {
        Some(v) => v.serialize(serializer),
        None => Err(serde::ser::Error::custom("Box<dyn Any> cannot be serialized")),
    }
}

fn deserialize_box_any<'de, D>(deserializer: D) -> Result<Box<dyn Any>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    serde_json::Value::deserialize(deserializer)
        .map(|v| Box::new(v) as Box<dyn Any>)
        .map_err(serde::de::Error::custom)
}

// Serialization and Deserialization functions for HashMap<String, Box<dyn Any>>

fn serialize_hashmap_box_any<S>(value: &HashMap<String, Box<dyn Any>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut map = serde_json::Map::new();
    for (key, val) in value {
        map.insert(key.clone(), serde_json::to_value(val).unwrap());
    }
    map.serialize(serializer)
}

fn deserialize_hashmap_box_any<'de, D>(deserializer: D) -> Result<HashMap<String, Box<dyn Any>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let map: HashMap<String, serde_json::Value> = Deserialize::deserialize(deserializer)?;
    let mut new_map = HashMap::new();
    for (key, val) in map {
        let any_val: Box<dyn Any> = Box::new(val);
        new_map.insert(key, any_val);
    }
    Ok(new_map)
}


impl Message {
    pub fn new<T: Any>(data: T) -> Self {
        Message {
            content: Box::new(data),
            params: HashMap::new(),
            last_update: Local::now(),
        }
    }

    pub fn get_content<T: Any>(&mut self) -> Option<&mut T> {
        self.content.downcast_mut()
    }

    pub fn set_content<T: Any>(&mut self, value: T) {
        self.content = Box::new(value);
        self.last_update = Local::now();
    }

    pub fn set_param<T: Any>(&mut self, key: &str, val: T) {
        self.params.insert(key.to_string(), Box::new(val));
        self.last_update = Local::now();
    }

    pub fn get_param<T: Any>(&mut self, key: &str) -> Option<&mut T> {
        if let Some(par) = self.params.get_mut(key) {
            if let Some(inner) = par.downcast_mut::<T>() {
                Some(inner)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_keys(&mut self) -> Vec<String> {
        let keys = self.params.keys().cloned().collect();
        return keys;
    }

    pub fn get_last_update_time(&self) -> String {
        self.last_update.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
