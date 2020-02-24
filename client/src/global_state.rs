#[derive(Copy, Clone)]
pub struct GlobalState {
    pub running: bool,
}
impl Default for GlobalState {
    fn default() -> Self {
        Self {
            running: true,
        }
    }
}
impl GlobalState {
    pub fn shutdown(&mut self) {
        self.running = false;
    }
    pub fn is_running(self) -> bool {
        self.running
    }
}