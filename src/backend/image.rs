use dyn_clone::DynClone;

use iced::widget::image::Handle;
use indexmap::IndexMap;
use reqwest::Url;

use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
pub struct ImageCache {
    entries: IndexMap<String, Handle>,
}

impl ImageCache {
    pub fn get(&self, source: impl Into<String>) -> Option<&Handle> {
        // self.in_use.insert(url.to_string());
        self.entries.get(&source.into())
    }

    pub fn insert(&mut self, hash: impl Into<String>, handle: Handle) {
        self.entries.insert(hash.into(), handle);
    }
}

pub trait ImageSource: DynClone + Debug + Send + 'static {
    fn url(&self) -> Url;
    fn hash(&self) -> String;
}
dyn_clone::clone_trait_object!(ImageSource);
