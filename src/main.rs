use std::sync::Arc;

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::peripherals::Peripherals,
    timer::EspTaskTimerService,
    wifi::{AsyncWifi, EspWifi},
};

type ArcAsyncMutex<T> = std::sync::Arc<tokio::sync::Mutex<T>>;
type Wifi = AsyncWifi<EspWifi<'static>>;
type ShareableWifi = ArcAsyncMutex<Wifi>;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let sys_loop = EspSystemEventLoop::take().unwrap();

    let wifi: ShareableWifi = Arc::new(tokio::sync::Mutex::new(
        AsyncWifi::wrap(
            EspWifi::new(peripherals.modem, sys_loop.clone(), None).unwrap(),
            sys_loop,
            EspTaskTimerService::new().unwrap(),
        )
        .unwrap(),
    ));

    let wifi = wifi.clone();

    tokio::spawn(async move {
        let mut guard = wifi.lock_owned().await;

        guard.start().await.unwrap();
    });

    log::info!("Hello, world!");
}
