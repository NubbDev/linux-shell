use std::any::Any;

use iced::{Rectangle, widget::Id};
use iced_runtime::core::widget::{
    Operation,
    operation::{Focusable, Outcome, Scrollable, TextInput},
};

pub struct FindBounds {
    target: Id,
    found: Option<Bounds>,
}

#[derive(Debug, Clone)]
pub enum Bounds {
    Container(Rectangle),
    Window(Rectangle),
    Scrollable(Rectangle, Rectangle),
}

impl FindBounds {
    pub fn new(target: Id) -> Self {
        Self {
            target,
            found: None,
        }
    }
}

impl Operation<Bounds> for FindBounds {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<Bounds>)) {
        operate(self);
    }

    fn container(&mut self, id: Option<&Id>, bounds: Rectangle) {
        if id == Some(&self.target) {
            let bound = Bounds::Container(bounds);
            self.found = Some(bound);
        }
    }

    fn scrollable(
        &mut self,
        id: Option<&Id>,
        bounds: Rectangle,
        content_bounds: Rectangle,
        _translation: iced::Vector,
        _state: &mut dyn Scrollable,
    ) {
        if id == Some(&self.target) {
            let bound = Bounds::Scrollable(bounds, content_bounds);
            self.found = Some(bound);
        }
    }

    fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, _state: &mut dyn Focusable) {}

    fn text_input(&mut self, _id: Option<&Id>, _bounds: Rectangle, _state: &mut dyn TextInput) {}

    fn text(&mut self, _id: Option<&Id>, _bounds: Rectangle, _text: &str) {}

    fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, _state: &mut dyn Any) {
        if id == Some(&self.target) && self.found.is_none() {
            self.found = Some(Bounds::Window(bounds));
        }
    }

    fn finish(&self) -> Outcome<Bounds> {
        if let Some(bounds) = &self.found {
            return Outcome::Some(bounds.clone());
        }
        Outcome::None
    }
}
