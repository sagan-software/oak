use std::collections::HashSet;
use web_sys::Document;

pub struct Events {
    configured: HashSet<String>
}

pub fn setup_synthetic_event(document: &Document, events: &mut Events, name: &str) {
    if events.configured.contains(name.to_owned()) {
        return;
    }
}