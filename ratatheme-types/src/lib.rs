use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Style {
    #[cfg_attr(feature = "serde", serde(alias = "fg"))]
    pub foreground: Option<String>,
    #[cfg_attr(feature = "serde", serde(alias = "bg"))]
    pub background: Option<String>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Colors {
    #[cfg_attr(feature = "serde", serde(flatten))]
    map: HashMap<String, String>,
}

impl Style {
    #[must_use]
    pub fn new<S1, S2>(foreground: S1, background: S2) -> Self
    where
        S1: Into<Option<String>>,
        S2: Into<Option<String>>,
    {
        Self {
            foreground: foreground.into(),
            background: background.into(),
        }
    }

    #[must_use]
    pub fn foreground<S>(mut self, foreground: S) -> Self
    where
        S: Into<String>,
    {
        self.foreground = Some(foreground.into());
        self
    }

    #[must_use]
    pub fn background<S>(mut self, background: S) -> Self
    where
        S: Into<String>,
    {
        self.background = Some(background.into());
        self
    }
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "foreground: {:?} background: {:?}",
            self.foreground, self.background
        )
    }
}

impl Colors {
    #[must_use]
    pub fn new<K, V>(entries: Vec<(K, V)>) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        let map = entries
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();

        Self { map }
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(std::string::String::as_str)
    }
}

impl std::fmt::Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for (key, value) in &self.map {
            result.push_str(&format!("{key}: {value}\n"));
        }

        write!(f, "{result}")
    }
}
