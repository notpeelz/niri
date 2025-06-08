use hyprland_lock_notification_v1::HyprlandLockNotificationV1;
use hyprland_lock_notifier_v1::HyprlandLockNotifierV1;
use smithay::reexports::wayland_server::{
    Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource,
};
use wayland_backend::server::ClientId;

use super::raw::hyprland_lock_notify::v1::server::{
    hyprland_lock_notification_v1, hyprland_lock_notifier_v1,
};

const VERSION: u32 = 1;

pub struct HyprlandLockNotifierState {}

pub struct HyprlandLockNotifierGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

pub trait HyprlandLockNotifierHandler {
    fn new_notification(&mut self, notification: HyprlandLockNotificationV1);
    fn notification_destroyed(&mut self, notification: HyprlandLockNotificationV1);
}

impl HyprlandLockNotifierState {
    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> Self
    where
        D: GlobalDispatch<HyprlandLockNotifierV1, HyprlandLockNotifierGlobalData>,
        D: Dispatch<HyprlandLockNotifierV1, ()>,
        D: HyprlandLockNotifierHandler,
        D: 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let global_data = HyprlandLockNotifierGlobalData {
            filter: Box::new(filter),
        };
        display.create_global::<D, HyprlandLockNotifierV1, _>(VERSION, global_data);

        Self {}
    }
}

impl<D> GlobalDispatch<HyprlandLockNotifierV1, HyprlandLockNotifierGlobalData, D>
    for HyprlandLockNotifierState
where
    D: GlobalDispatch<HyprlandLockNotifierV1, HyprlandLockNotifierGlobalData>,
    D: Dispatch<HyprlandLockNotifierV1, ()>,
    D: HyprlandLockNotifierHandler,
    D: 'static,
{
    fn bind(
        _state: &mut D,
        _handle: &DisplayHandle,
        _client: &Client,
        manager: New<HyprlandLockNotifierV1>,
        _manager_state: &HyprlandLockNotifierGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(manager, ());
    }

    fn can_view(client: Client, global_data: &HyprlandLockNotifierGlobalData) -> bool {
        (global_data.filter)(&client)
    }
}

impl<D> Dispatch<HyprlandLockNotifierV1, (), D> for HyprlandLockNotifierState
where
    D: Dispatch<HyprlandLockNotifierV1, ()>,
    D: Dispatch<HyprlandLockNotificationV1, ()>,
    D: HyprlandLockNotifierHandler,
    D: 'static,
{
    fn request(
        state: &mut D,
        _client: &Client,
        _resource: &HyprlandLockNotifierV1,
        request: <HyprlandLockNotifierV1 as Resource>::Request,
        _data: &(),
        _dhandle: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            hyprland_lock_notifier_v1::Request::Destroy => (),
            hyprland_lock_notifier_v1::Request::GetLockNotification { id } => {
                let notification = data_init.init(id, ());
                state.new_notification(notification);
            }
        }
    }
}

impl<D> Dispatch<HyprlandLockNotificationV1, (), D> for HyprlandLockNotifierState
where
    D: Dispatch<HyprlandLockNotifierV1, ()>,
    D: Dispatch<HyprlandLockNotificationV1, ()>,
    D: HyprlandLockNotifierHandler,
    D: 'static,
{
    fn request(
        _state: &mut D,
        _client: &Client,
        _resource: &HyprlandLockNotificationV1,
        request: <HyprlandLockNotificationV1 as Resource>::Request,
        _data: &(),
        _dhandle: &DisplayHandle,
        _data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            hyprland_lock_notification_v1::Request::Destroy => {}
        }
    }

    fn destroyed(
        state: &mut D,
        _client: ClientId,
        resource: &HyprlandLockNotificationV1,
        _data: &(),
    ) {
        state.notification_destroyed(resource.clone());
    }
}

#[macro_export]
macro_rules! delegate_hyprland_lock_notify {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::raw::hyprland_lock_notify::v1::server::hyprland_lock_notifier_v1::HyprlandLockNotifierV1: $crate::protocols::hyprland_lock_notify::HyprlandLockNotifierGlobalData
        ] => $crate::protocols::hyprland_lock_notify::HyprlandLockNotifierState);

        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::raw::hyprland_lock_notify::v1::server::hyprland_lock_notifier_v1::HyprlandLockNotifierV1: ()
        ] => $crate::protocols::hyprland_lock_notify::HyprlandLockNotifierState);

        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::raw::hyprland_lock_notify::v1::server::hyprland_lock_notification_v1::HyprlandLockNotificationV1: ()
        ] => $crate::protocols::hyprland_lock_notify::HyprlandLockNotifierState);
    };
}
