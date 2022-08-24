use crate::Journal;

pub struct Session {
    journal: Box<Journal>,
}

impl Session {
    pub fn new() -> Self {
        log::trace!("created Session ...");

        let journal = Box::new(Journal::new());
        Self { journal }
    }
}
