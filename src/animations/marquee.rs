use iced::{Animation, animation::Easing, window::Id};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::animations::{Animationable, AnimationableProgress};

const PAUSE: Duration = Duration::from_millis(1500);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarqueeKey(pub Id, pub String);

impl MarqueeKey {
    pub fn new(window_id: Id, text: &str) -> Self {
        Self(window_id, text.into())
    }
}

pub struct MarqueeSlot {
    pub anim: Animation<bool>,
    pub content_width: f32,
    pub container_width: f32,
    finished_at: Option<Instant>,
}

impl MarqueeSlot {
    pub fn new(content_width: f32, container_width: f32) -> Self {
        Self {
            anim: Animation::new(false).easing(Easing::EaseOut),
            content_width,
            container_width,
            finished_at: None,
        }
    }

    pub fn is_animating(&self, t: Instant) -> bool {
        self.anim.is_animating(t)
    }

    pub fn value(&self) -> bool {
        self.anim.value()
    }

    pub fn go_mut(&mut self, value: bool, now: Instant) {
        self.anim.go_mut(value, now);
    }

    pub fn progress(&self, now: Instant) -> f32 {
        self.anim
            .interpolate_with(|i| if i { 1.0 } else { 0.0 }, now)
    }

    pub fn overflow(&self) -> f32 {
        (self.content_width - self.container_width).max(0.0)
    }
}

#[derive(Default)]
pub struct Marquees(HashMap<MarqueeKey, MarqueeSlot>);

impl Animationable for Marquees {
    fn update(&mut self, _: &typemap_rev::TypeMap) {
        let now = Instant::now();
        for slot in self.0.values_mut() {
            if slot.content_width <= slot.container_width {
                continue;
            }
            if slot.anim.is_animating(now) {
                slot.finished_at = None;
                continue;
            }
            match slot.finished_at {
                None => slot.finished_at = Some(now),
                Some(at) if now.duration_since(at) >= PAUSE => {
                    let current = slot.value();
                    slot.go_mut(!current, now);
                    slot.finished_at = None;
                }
                Some(_) => {} // still waiting
            }
        }
    }

    fn is_animating(&self, t: Instant) -> bool {
        self.0
            .values()
            .any(|slot| slot.is_animating(t) || slot.finished_at.is_some())
    }
}

impl AnimationableProgress<MarqueeKey> for Marquees {
    fn progress(&self, id: &MarqueeKey, now: Instant) -> Option<f32> {
        self.0.get(&id).map(|slot| slot.progress(now))
    }
}

impl Marquees {
    pub fn register(&mut self, key: MarqueeKey) {
        self.0
            .entry(key)
            .or_insert_with(|| MarqueeSlot::new(0.0, 0.0));
    }

    pub fn set_size(
        &mut self,
        key: MarqueeKey,
        content_width: f32,
        container_width: f32,
        speed: f32,
    ) {
        if let Some(slot) = self.0.get_mut(&key) {
            let overflow = (content_width - container_width).max(1.0);
            let duration = Duration::from_secs_f32(overflow / speed); // 50 pixels per second

            if slot.content_width != content_width {
                slot.anim = Animation::new(slot.anim.value())
                    .duration(duration)
                    .easing(Easing::EaseOut);
            }

            slot.content_width = content_width;
            slot.container_width = container_width;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&MarqueeKey, &MarqueeSlot)> {
        self.0.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &MarqueeKey> {
        self.0.keys()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
