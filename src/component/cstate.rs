#[derive(Clone, Copy)]
pub struct CState {
    pub on_ground: bool,
    pub forward: bool,
}

impl Default for CState {
    fn default() -> Self {
        CState {
            on_ground: false,
            forward: true,
        }
    }
}
