use std::{collections::HashSet, sync::Arc};

use enumflags2::BitFlags;
use iced::futures::{StreamExt, channel::mpsc::UnboundedSender};
use zbus::{
    Connection, Result,
    connection::Builder as BusConnBuilder,
    fdo::{self, DBusProxy, RequestNameFlags},
    interface,
    names::{UniqueName, WellKnownName},
    zvariant::Optional,
};

#[derive(Clone)]
pub struct StatusNotifierWatcher(UnboundedSender<TrayEvent>);

#[interface(name = "org.kde.StatusNotifierWatcher")]
impl StatusNotifierWatcher {
    async fn register_status_notifier_item(
        &self,
        service: &str,
        #[zbus(header)] header: zbus::message::Header<'_>,
    ) -> fdo::Result<()> {
        let sender = header
            .sender()
            .map(|s| s.as_str())
            .unwrap_or(service)
            .to_string();

        let path = if service.starts_with('/') {
            format!("{}{}", sender, service)
        } else {
            format!("{}/StatusNotifierItem", service)
        };

        let _ = self.0.unbounded_send(TrayEvent::Added(path));
        Ok(())
    }
}

pub async fn create_connection(tx: UnboundedSender<TrayEvent>) -> Result<Connection> {
    let watcher = StatusNotifierWatcher(tx);
    BusConnBuilder::session()?
        .serve_at("/StatusNotifierWatcher", watcher)?
        .build()
        .await
}

pub async fn dbus_proxy<'a>(conn: Connection) -> fdo::Result<fdo::DBusProxy<'a>> {
    let dbus_proxy = DBusProxy::new(&conn).await?;
    if let Err(e) = dbus_proxy.request_name(prox_name(), flags()).await {
        eprintln!("Failed to request name: {}", e);
    }
    Ok(dbus_proxy)
}

pub async fn reset_ownership<'a>(name: UniqueName<'a>, proxy: DBusProxy<'a>) {
    let mut stream = proxy.receive_name_owner_changed().await.unwrap();
    while let Some(signal) = stream.next().await {
        if let Ok(args) = signal.args()
            && args.name.as_str() == "org.kde.StatusNotifierWatcher"
        {
            if let Some(new_owner) = args.new_owner.as_ref()
                && new_owner.as_str() != name.as_str()
            {
                println!(
                    "Ownership of StatusNotifierWatcher changed to: {}",
                    new_owner
                );
                if let Err(e) = proxy.request_name(prox_name(), flags()).await {
                    eprintln!("Failed to request name: {}", e);
                }
            }
        }
    }
}

pub fn flags() -> BitFlags<RequestNameFlags> {
    RequestNameFlags::ReplaceExisting | RequestNameFlags::AllowReplacement
}

pub fn prox_name() -> WellKnownName<'static> {
    "org.kde.StatusNotifierWatcher".try_into().unwrap()
}

pub enum TrayEvent {
    Added(String),
    Removed(String),
}
