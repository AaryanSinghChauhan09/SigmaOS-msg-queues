// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-msg-queues: msg queues component model.
//!
//! Real implementation: queues, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Queue {
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
pub enum MsgQueuesError {
    EmptyQueueName,
    DuplicateQueue(String),
    EmptyEventSubject,
}

impl fmt::Display for MsgQueuesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyQueueName => write!(f, "queue name must not be empty"),
            Self::DuplicateQueue(id) => write!(f, "duplicate queue {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MsgQueuesError {}

pub struct MsgQueues {
    pub name: String,
    queues: BTreeMap<String, Queue>,
    events: BTreeMap<String, Event>,
}

impl MsgQueues {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            queues: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_queue(&mut self, item: Queue) -> Result<(), MsgQueuesError> {
        if item.name.trim().is_empty() {
            return Err(MsgQueuesError::EmptyQueueName);
        }
        let id = item.id.clone();
        if self.queues.contains_key(&id) {
            return Err(MsgQueuesError::DuplicateQueue(id.clone()));
        }
        self.queues.insert(id, item);
        Ok(())
    }

    pub fn queue_count(&self) -> usize {
        self.queues.len()
    }

    pub fn get_queue(&self, id: &str) -> Option<&Queue> {
        self.queues.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MsgQueuesError> {
        if event.subject.trim().is_empty() {
            return Err(MsgQueuesError::EmptyEventSubject);
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

    fn sample_queue(id: &str, name: &str) -> Queue {
        Queue {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MsgQueues::new("MsgQueues");
        assert_eq!(m.name, "MsgQueues");
        assert_eq!(m.queue_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_queue() {
        let mut m = MsgQueues::new("MsgQueues");
        m.add_queue(sample_queue("a1", "Alpha")).unwrap();
        assert_eq!(m.queue_count(), 1);
        assert!(m.get_queue("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MsgQueues::new("MsgQueues");
        assert!(m.add_queue(sample_queue("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MsgQueues::new("MsgQueues");
        m.add_queue(sample_queue("x", "One")).unwrap();
        assert!(m.add_queue(sample_queue("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MsgQueues::new("MsgQueues");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
