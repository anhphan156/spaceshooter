#[derive(Clone, Copy)]
pub struct CState {
    pub on_ground: bool,
}

impl Default for CState {
    fn default() -> Self {
        CState { on_ground: false }
    }
}
