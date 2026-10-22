use crate::{app::AppState, colors};
use dyn_clone::DynClone;
use iced::{
    Border, Color,
    widget::{Id, container},
    window::Id as WindowId,
};
use iced_layershell::reexport::Anchor;
use std::fmt::Debug;

mod date;
mod spotify;
mod system;
mod volume;
mod workspace;

pub use date::*;
pub use spotify::*;
pub use system::*;
pub use volume::*;
pub use workspace::WorkspacePill;

pub trait WindowComponent: DynClone + Debug + Send + Sync + 'static {
    fn new(display: String) -> Self
    where
        Self: Sized;
    fn id(&self, id: WindowId) -> Id {
        Id::from(format!("{}-{}", self.namespace(), id))
    }
    fn namespace(&self) -> String;
    fn view<'a>(
        &self,
        id: WindowId,
        state: &'a AppState,
    ) -> iced::Element<'a, crate::event::AppEvent>;
    fn display(&self) -> &String;
    fn anchor(&self) -> Anchor;
    fn identifiers(&self) -> Vec<Identifiers> {
        Vec::new()
    }
}

dyn_clone::clone_trait_object!(WindowComponent);

pub(self) fn pills_style(color: Color) -> container::Style {
    container::Style {
        background: Some(colors::BACKGROUND.scale_alpha(0.8).into()),
        border: Border {
            color: color,
            width: 2.0,
            radius: 2.into(),
        },
        ..Default::default()
    }
}

pub enum Identifiers {
    Marquee(String),
}

impl Identifiers {
    pub fn marquee(name: &str) -> Self {
        Identifiers::Marquee(name.into())
    }
}
