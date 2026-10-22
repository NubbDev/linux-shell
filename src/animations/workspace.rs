use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use iced::{Animation, animation::Easing};
use typemap_rev::TypeMap;

use crate::{animations::AnimationableProgress, data::Workspace};

pub struct WorkspaceActiveSlot {
    pub anim: Animation<bool>,
}

impl WorkspaceActiveSlot {
    pub fn new(is_active: bool) -> Self {
        Self {
            anim: Animation::new(is_active)
                .duration(Duration::from_millis(400))
                .easing(Easing::EaseOut),
        }
    }

    pub fn is_animating(&self, t: Instant) -> bool {
        self.anim.is_animating(t)
    }

    pub fn go_mut(&mut self, value: bool, now: Instant) {
        self.anim.go_mut(value, now);
    }
}

#[derive(Default)]
pub struct WorkspaceActive(HashMap<i32, WorkspaceActiveSlot>);

impl super::Animationable for WorkspaceActive {
    fn update(&mut self, data: &TypeMap) {
        let Some(workspaces) = data.get::<Workspace>() else {
            return;
        };
        let now = Instant::now();

        for (id, ws) in workspaces {
            let anim = (self.0.entry(*id)).or_insert(WorkspaceActiveSlot::new(ws.is_active));
            anim.go_mut(ws.is_active, now);
        }

        let live_ids: std::collections::HashSet<_> = workspaces.iter().map(|(id, _)| *id).collect();
        self.0.retain(|id, _| live_ids.contains(id));
    }

    fn is_animating(&self, t: Instant) -> bool {
        self.0.values().any(|anim| anim.is_animating(t))
    }
}

impl AnimationableProgress<i32> for WorkspaceActive {
    fn progress(&self, id: &i32, now: Instant) -> Option<f32> {
        self.0.get(&id).map(|anim| {
            anim.anim
                .interpolate_with(|is_active| if is_active { 1.0 } else { 0.0 }, now)
        })
    }
}
