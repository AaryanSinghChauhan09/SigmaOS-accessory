// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-accessory: accessory component model.
//!
//! Real implementation: accessory_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessoryItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessoryError {
    EmptyAccessoryItemName,
    DuplicateAccessoryItem(String),
    EmptyEventSubject,
}

impl fmt::Display for AccessoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAccessoryItemName => write!(f, "accessory_item name must not be empty"),
            Self::DuplicateAccessoryItem(id) => write!(f, "duplicate accessory_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AccessoryError {}

pub struct Accessory {
    pub name: String,
    accessory_items: BTreeMap<String, AccessoryItem>,
    events: BTreeMap<String, Event>,
}

impl Accessory {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            accessory_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_accessory_item(&mut self, item: AccessoryItem) -> Result<(), AccessoryError> {
        if item.name.trim().is_empty() {
            return Err(AccessoryError::EmptyAccessoryItemName);
        }
        let id = item.id.clone();
        if self.accessory_items.contains_key(&id) {
            return Err(AccessoryError::DuplicateAccessoryItem(id.clone()));
        }
        self.accessory_items.insert(id, item);
        Ok(())
    }

    pub fn accessory_item_count(&self) -> usize {
        self.accessory_items.len()
    }

    pub fn get_accessory_item(&self, id: &str) -> Option<&AccessoryItem> {
        self.accessory_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AccessoryError> {
        if event.subject.trim().is_empty() {
            return Err(AccessoryError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_accessory_item(id: &str, name: &str) -> AccessoryItem {
        AccessoryItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Accessory::new("Accessory");
        assert_eq!(m.name, "Accessory");
        assert_eq!(m.accessory_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_accessory_item() {
        let mut m = Accessory::new("Accessory");
        m.add_accessory_item(sample_accessory_item("a1", "Alpha")).unwrap();
        assert_eq!(m.accessory_item_count(), 1);
        assert!(m.get_accessory_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Accessory::new("Accessory");
        assert!(m.add_accessory_item(sample_accessory_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Accessory::new("Accessory");
        m.add_accessory_item(sample_accessory_item("x", "One")).unwrap();
        assert!(m.add_accessory_item(sample_accessory_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Accessory::new("Accessory");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
