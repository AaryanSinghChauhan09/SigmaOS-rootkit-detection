// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-rootkit-detection: rootkit detection component model.
//!
//! Real implementation: detections, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detection {
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
pub enum RootkitDetectionError {
    EmptyDetectionName,
    DuplicateDetection(String),
    EmptyEventSubject,
}

impl fmt::Display for RootkitDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDetectionName => write!(f, "detection name must not be empty"),
            Self::DuplicateDetection(id) => write!(f, "duplicate detection {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for RootkitDetectionError {}

pub struct RootkitDetection {
    pub name: String,
    detections: BTreeMap<String, Detection>,
    events: BTreeMap<String, Event>,
}

impl RootkitDetection {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            detections: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_detection(&mut self, item: Detection) -> Result<(), RootkitDetectionError> {
        if item.name.trim().is_empty() {
            return Err(RootkitDetectionError::EmptyDetectionName);
        }
        let id = item.id.clone();
        if self.detections.contains_key(&id) {
            return Err(RootkitDetectionError::DuplicateDetection(id.clone()));
        }
        self.detections.insert(id, item);
        Ok(())
    }

    pub fn detection_count(&self) -> usize {
        self.detections.len()
    }

    pub fn get_detection(&self, id: &str) -> Option<&Detection> {
        self.detections.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), RootkitDetectionError> {
        if event.subject.trim().is_empty() {
            return Err(RootkitDetectionError::EmptyEventSubject);
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

    fn sample_detection(id: &str, name: &str) -> Detection {
        Detection {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = RootkitDetection::new("RootkitDetection");
        assert_eq!(m.name, "RootkitDetection");
        assert_eq!(m.detection_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_detection() {
        let mut m = RootkitDetection::new("RootkitDetection");
        m.add_detection(sample_detection("a1", "Alpha")).unwrap();
        assert_eq!(m.detection_count(), 1);
        assert!(m.get_detection("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = RootkitDetection::new("RootkitDetection");
        assert!(m.add_detection(sample_detection("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = RootkitDetection::new("RootkitDetection");
        m.add_detection(sample_detection("x", "One")).unwrap();
        assert!(m.add_detection(sample_detection("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = RootkitDetection::new("RootkitDetection");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
