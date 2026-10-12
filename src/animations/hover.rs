use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

use iced::{Animation, animation::Easing, widget::Id};
use typemap_rev::TypeMapKey;

pub struct HoverSlot {
    pub value: bool,
    pub anim: iced::Animation<bool>,
}

impl HoverSlot {
    pub fn new(value: bool) -> Self {
        Self {
            value,
            anim: Animation::new(value)
                .duration(Duration::from_millis(400))
                .easing(Easing::EaseOut),
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
}

#[derive(Default)]
pub struct HoverAnimations(HashMap<Id, HoverSlot>);

impl super::Animationable for HoverAnimations {
    fn update(&mut self, data: &typemap_rev::TypeMap) {
        let Some(active_hovers) = data.get::<Hovers>() else {
            return;
        };
        let now = Instant::now();

        for slot in active_hovers {
            let slot = self.0.entry(slot.clone()).or_insert(HoverSlot::new(false));
            slot.go_mut(true, now);
        }

        for (id, slot) in self.0.iter_mut() {
            if !active_hovers.contains(id) {
                slot.go_mut(false, now);
            }
        }
    }

    fn is_animating(&self, t: Instant) -> bool {
        self.0.values().any(|slot| slot.is_animating(t))
    }
}

impl super::AnimationableProgress<Id> for HoverAnimations {
    fn progress(&self, id: &Id, now: Instant) -> Option<f32> {
        (self.0.get(id)).map(|s| s.anim.interpolate_with(|i| if i { 1.0 } else { 0.0 }, now))
    }
}

pub struct Hovers;

impl TypeMapKey for Hovers {
    type Value = HashSet<Id>;
}
