use downcast_rs::Downcast;
use std::time::Instant;
use typemap_rev::TypeMap;

pub mod hover;
pub mod marquee;
pub mod progress;
pub mod workspace;

pub fn setup_animations() -> Vec<Box<dyn Animationable>> {
    vec![
        Box::new(workspace::WorkspaceActive::default()),
        Box::new(marquee::Marquees::default()),
        Box::new(hover::HoverAnimations::default()),
    ]
}

pub trait Animationable: Downcast {
    fn update(&mut self, data: &TypeMap);
    fn is_animating(&self, t: Instant) -> bool;
}

pub trait AnimationableProgress<T>: Animationable {
    fn progress(&self, id: &T, now: Instant) -> Option<f32>;
}
