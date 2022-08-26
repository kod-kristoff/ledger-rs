use crate::Session;
use std::sync::Arc;

pub struct GlobalScope {
    session: Arc<Session>,
}

impl GlobalScope {
    pub fn new() -> Self {
        log::trace!("created GlobalScope ...");
        let session = Arc::new(Session::new());
        Self { session }
    }
}
