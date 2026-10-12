use std::{thread, time::Duration};

use chrono::{DateTime, Local, Timelike};
use iced::{
    Subscription, Task,
    futures::{
        SinkExt, StreamExt,
        channel::mpsc::{Sender, unbounded},
        stream::BoxStream,
    },
    widget::svg::Handle,
};
use tokio::runtime::Runtime;
use typemap_rev::{TypeMap, TypeMapKey};
use zbus::{
    connection::Builder as BusConnBuilder,
    fdo::{DBusProxy, RequestNameFlags},
};

use crate::{backend::app_tray::proxy::StatusNotifierItemProxy, event::AppEvent};
mod proxy;
mod watch;

pub fn subscription() -> Subscription<AppEvent> {
    Subscription::run(system_tray)
}

fn system_tray() -> BoxStream<'static, AppEvent> {
    let (tx, mut rx) = unbounded::<()>();

    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        let (event_tx, mut event_rx) = unbounded::<watch::TrayEvent>();
        rt.block_on(async move {
            let connection = watch::create_connection(event_tx).await.unwrap();
            let name = connection.unique_name().unwrap().to_owned();
            let dbus_proxy = watch::dbus_proxy(connection.clone()).await.unwrap();

            let dbus_proxy_clone = dbus_proxy.clone();
            tokio::spawn(async move {
                watch::reset_ownership(name.into(), dbus_proxy_clone).await;
            });
            println!("Running System Tray...");
            while let Some(event) = event_rx.next().await {
                match event {
                    watch::TrayEvent::Added(service) => {
                        let (dest, path) = match service.split_once('/') {
                            Some((d, p)) => (d, format!("/{}", p)),
                            None => (service.as_str(), "/StatusNotifierItem".to_string()),
                        };

                        if let Ok(proxy) = StatusNotifierItemProxy::builder(&connection)
                            .destination(dest)
                            .unwrap()
                            .path(path)
                            .unwrap()
                            .build()
                            .await
                        {
                            let title = proxy.title().await.unwrap_or_default();
                            let id = proxy.id().await.unwrap_or_default();
                            println!(
                                "Tray item added: {} - Title: {}, Id: {}",
                                service, title, id
                            );
                        }
                    }
                    watch::TrayEvent::Removed(service) => {
                        println!("Tray item removed: {}", service);
                        // Handle the removed tray item
                    }
                }
            }
        });
    });

    Box::pin(iced::stream::channel::<AppEvent>(
        1000,
        move |mut sender: Sender<AppEvent>| async move {
            while let Some(event) = rx.next().await {
                // if let Err(e) = sender.send(AppEvent::update_data(event)).await {
                //     eprintln!("Failed to send event: {}", e);
                //     break;
                // }
            }
        },
    ))
}
pub struct TrayItem {
    pub id: String,
    pub title: String,
    pub icon: Handle,
}
