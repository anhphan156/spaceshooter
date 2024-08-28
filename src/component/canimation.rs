use crate::animation::Animation;

#[derive(Clone, Debug)]
pub struct CAnimation {
    pub enabled: bool,
    pub animation: Animation,
}

impl Default for CAnimation {
    fn default() -> Self {
        CAnimation {
            enabled: false,
            animation: Animation::new(0, 0),
        }
    }
}
