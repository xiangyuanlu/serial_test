use std::sync::{Arc, Weak};

use crate::channel::Channel;

pub(crate) struct Strip {
    channel: Weak<Channel>,
}

impl Strip {
    pub fn work(&self) {}
}

struct StripManager {
    strips: dashmap::DashMap<String, Arc<Strip>>,
}

impl StripManager {
    fn add_strip(&mut self, addr: u32, name: &str) -> anyhow::Result<Arc<Strip>> {
        todo!()
    }
}
