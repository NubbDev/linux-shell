use zbus::{
    Result, proxy,
    zvariant::{OwnedObjectPath, OwnedValue, Value},
};
#[proxy(
    interface = "org.kde.StatusNotifierItem",
    default_path = "/StatusNotifierItem"
)]
pub trait StatusNotifierItem {
    #[zbus(property)]
    fn id(&self) -> Result<String>;
    #[zbus(property)]
    fn title(&self) -> Result<String>;
    #[zbus(property)]
    fn icon_name(&self) -> Result<String>;
    #[zbus(property)]
    fn menu(&self) -> zbus::Result<OwnedObjectPath>;
    fn activate(&self, x: i32, y: i32) -> zbus::Result<()>;
}

#[proxy(interface = "com.canonical.dbusmenu", default_path = "/MenuBar")]
pub trait DBusMenu {
    /// Returns: (revision, (id, properties, [children]))
    fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        property_names: &[&str],
    ) -> zbus::Result<(
        u32,
        (
            i32,
            std::collections::HashMap<String, OwnedValue>,
            Vec<OwnedValue>,
        ),
    )>;

    fn event(&self, id: i32, event_id: &str, data: &Value<'_>, timestamp: u32) -> zbus::Result<()>;
}
