pub struct Journal {}

impl Journal {
    pub fn new() -> Self {
        log::trace!("created Journal ...");

        Self {}
    }
}
