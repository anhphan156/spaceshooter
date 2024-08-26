#[derive(Clone, Debug)]
pub struct CInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Default for CInput {
    fn default() -> Self {
        CInput {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}
