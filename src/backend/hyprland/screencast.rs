#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScreenCastOwner {
    Monitor(String),
    Window(String),
    Region
}
