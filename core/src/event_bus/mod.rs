#[cfg(test)]
mod tests;

use std::{
    any::Any,
    collections::HashMap,
    pin::Pin,
    sync::{Arc, LazyLock, RwLock},
};

pub type EventFuture<'a> = Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

pub type EventCallback =
    Arc<dyn for<'a> Fn(&'a [Box<dyn Any + Send + Sync>]) -> EventFuture<'a> + Send + Sync>;

static EVENT_BUS: LazyLock<RwLock<EventBus>> = LazyLock::new(|| RwLock::new(EventBus::new()));

pub struct EventBus {
    callbacks: HashMap<String, Vec<EventCallback>>,
}

impl EventBus {
    fn new() -> Self {
        Self {
            callbacks: HashMap::new(),
        }
    }

    pub fn on(event: &str, callback: EventCallback) {
        EVENT_BUS
            .write()
            .unwrap()
            .callbacks
            .entry(event.to_string())
            .or_default()
            .push(callback);
    }

    pub async fn emit(event: &str, data: &[Box<dyn Any + Send + Sync>]) {
        let callbacks = {
            let bus = EVENT_BUS.read().unwrap();
            bus.callbacks.get(event).cloned()
        };
        if let Some(callbacks) = callbacks {
            for cb in callbacks {
                cb(data).await
            }
        }
    }
}
