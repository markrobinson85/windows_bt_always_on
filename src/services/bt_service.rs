// Ping service example.
//
// You can install and uninstall this service using other example programs.
// All commands mentioned below shall be executed in Command Prompt with Administrator privileges.
//
// Service installation: `install_service.exe`
// Service uninstallation: `uninstall_service.exe`
//
// Start the service: `net start bt_always_on_service`
// Stop the service: `net stop bt_always_on_service`
//
// Ping server sends a text message to local UDP port 1234 once a second.
// You can verify that service works by running netcat, i.e: `ncat -ul 1234`.

// #[cfg(windows)]
// fn main() -> windows_service::Result<()> {
//     bt_always_on_service::run()
// }

// #[cfg(not(windows))]
// fn main() {
//     panic!("This program is only intended to run on Windows.");
// }

// #[cfg(windows)]
// pub mod bt_always_on_service {
//     use std::{
//         ffi::OsString,
//         net::{IpAddr, SocketAddr, UdpSocket},
//         sync::mpsc,
//         time::Duration,
//     };
//     use windows_service::{
//         define_windows_service,
//         service::{
//             ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
//             ServiceType,
//         },
//         service_control_handler::{self, ServiceControlHandlerResult},
//         service_dispatcher, Result,
//     };
//     // use windows::{
//     //     core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
//     //     Win32::UI::WindowsAndMessaging::*,
//     //     Devices::Radios::{Radio, RadioKind, RadioState, RadioAccessStatus},
//     //     Foundation::Collections::IVectorView,
//     //     Foundation::TypedEventHandler,
//     //     // Devices::Radios::Radios
//     // };

//     use crate::services::hw;

//     const SERVICE_NAME: &str = "bt_always_on_service";
//     const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

//     // const LOOPBACK_ADDR: [u8; 4] = [127, 0, 0, 1];
//     // const RECEIVER_PORT: u16 = 1234;
//     // const PING_MESSAGE: &str = "ping\n";

//     pub fn run() -> Result<()> {
//         // Register generated `ffi_service_main` with the system and start the service, blocking
//         // this thread until the service is stopped.
//         service_dispatcher::start(SERVICE_NAME, ffi_service_main)
//     }

//     // Generate the windows service boilerplate.
//     // The boilerplate contains the low-level service entry function (ffi_service_main) that parses
//     // incoming service arguments into Vec<OsString> and passes them to user defined service
//     // entry (bt_service_main).
//     define_windows_service!(ffi_service_main, bt_service_main);

//     // Service entry function which is called on background thread by the system with service
//     // parameters. There is no stdout or stderr at this point so make sure to configure the log
//     // output to file if needed.
//     pub async fn bt_service_main(_arguments: Vec<OsString>) {
//         if let Err(_e) = run_service().await {
//             // Handle the error, by logging or something.
//         }
//     }

//     pub async fn run_service() -> Result<()> {
//         // Create a channel to be able to poll a stop event from the service worker loop.
//         let (shutdown_tx, shutdown_rx) = mpsc::channel();
        
//         // Define system service event handler that will be receiving service events.
//         let event_handler = move |control_event| -> ServiceControlHandlerResult {
//             match control_event {
//                 // Notifies a service to report its current status information to the service
//                 // control manager. Always return NoError even if not implemented.
//                 ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

//                 // Handle stop
//                 ServiceControl::Stop => {
//                     shutdown_tx.send(()).unwrap();
//                     ServiceControlHandlerResult::NoError
//                 }

//                 _ => ServiceControlHandlerResult::NotImplemented,
//             }
//         };

//         // Register system service event handler.
//         // The returned status handle should be used to report service status changes to the system.
//         let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

//         // Tell the system that service is running
//         status_handle.set_service_status(ServiceStatus {
//             service_type: SERVICE_TYPE,
//             current_state: ServiceState::Running,
//             controls_accepted: ServiceControlAccept::STOP,
//             exit_code: ServiceExitCode::Win32(0),
//             checkpoint: 0,
//             wait_hint: Duration::default(),
//             process_id: None,
//         })?;

//         // For demo purposes this service sends a UDP packet once a second.
//         // let loopback_ip = IpAddr::from(LOOPBACK_ADDR);
//         // let sender_addr = SocketAddr::new(loopback_ip, 0);
//         // let receiver_addr = SocketAddr::new(loopback_ip, RECEIVER_PORT);
//         // let msg = PING_MESSAGE.as_bytes();
//         // let socket = UdpSocket::bind(sender_addr).unwrap();

//         loop {
//             // Poll shutdown event.
//             match shutdown_rx.recv_timeout(Duration::from_secs(1)) {
//                 // Break the loop either upon stop or channel disconnect
//                 Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

//                 // Continue work if no events were received within the timeout
//                 Err(mpsc::RecvTimeoutError::Timeout) => (),
//             };
//         }

//         // Tell the system that service has stopped.
//         status_handle.set_service_status(ServiceStatus {
//             service_type: SERVICE_TYPE,
//             current_state: ServiceState::Stopped,
//             controls_accepted: ServiceControlAccept::empty(),
//             exit_code: ServiceExitCode::Win32(0),
//             checkpoint: 0,
//             wait_hint: Duration::default(),
//             process_id: None,
//         })?;

//         Ok(())
//     }
// }

#[cfg(windows)]
pub mod bt_always_on_service {
    use std::{
        ffi::OsString,
        net::{IpAddr, SocketAddr, UdpSocket},
        sync::mpsc,
        time::Duration,
    };
    use windows::{Foundation::TypedEventHandler, Devices::Radios::{Radio, RadioState}};
    use windows_service::{
        define_windows_service,
        service::{
            ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
            ServiceType,
        },
        service_control_handler::{self, ServiceControlHandlerResult},
        service_dispatcher, Result,
    };
    use crate::services::hw;
    use async_std::task;

    const SERVICE_NAME: &str = "bt_always_on_service";
    const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

    const LOOPBACK_ADDR: [u8; 4] = [127, 0, 0, 1];
    const RECEIVER_PORT: u16 = 1234;
    const PING_MESSAGE: &str = "ping\n";

    pub fn run() -> Result<()> {
        // Register generated `ffi_service_main` with the system and start the service, blocking
        // this thread until the service is stopped.
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
    }

    // Generate the windows service boilerplate.
    // The boilerplate contains the low-level service entry function (ffi_service_main) that parses
    // incoming service arguments into Vec<OsString> and passes them to user defined service
    // entry (my_service_main).
    define_windows_service!(ffi_service_main, my_service_main);

    // Service entry function which is called on background thread by the system with service
    // parameters. There is no stdout or stderr at this point so make sure to configure the log
    // output to file if needed.
    pub fn my_service_main(_arguments: Vec<OsString>) {
        if let Err(_e) = run_service() {
            // Handle the error, by logging or something.
        }
    }

    pub fn run_service() -> Result<()> {
        // Create a channel to be able to poll a stop event from the service worker loop.
        let (shutdown_tx, shutdown_rx) = mpsc::channel();

        // Define system service event handler that will be receiving service events.
        let event_handler = move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                // Notifies a service to report its current status information to the service
                // control manager. Always return NoError even if not implemented.
                ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

                // Handle stop
                ServiceControl::Stop => {
                    shutdown_tx.send(()).unwrap();
                    ServiceControlHandlerResult::NoError
                }

                _ => ServiceControlHandlerResult::NotImplemented,
            }
        };

        // Register system service event handler.
        // The returned status handle should be used to report service status changes to the system.
        let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

        // Tell the system that service is running
        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        // For demo purposes this service sends a UDP packet once a second.
        let loopback_ip = IpAddr::from(LOOPBACK_ADDR);
        let sender_addr = SocketAddr::new(loopback_ip, 0);
        let receiver_addr = SocketAddr::new(loopback_ip, RECEIVER_PORT);
        let msg = PING_MESSAGE.as_bytes();
        let socket = UdpSocket::bind(sender_addr).unwrap();

        // task::block_on(async {
        
        // });       
        
        // bluetooth.StateChanged(&TypedEventHandler::<Radio, _>::new(move |bt, _| {
        //     // Unrap the Option<Radio>, then we'll check if the new state is Off.
        //     let mut bt_radio: Radio = bt.as_ref().unwrap().clone();
        //     if bt_radio.State()? == RadioState::Off {
        //         // Turn the radio back on.
        //         bt_radio.SetStateAsync(RadioState::On).expect("Failed to turn Bluetooth back on.");
        //     }

        //     println!("Bluetooth state changed.");
        //     println!("{:#?}", bt_radio);
        //     Ok(())

        // }));
        
        loop {
            task::block_on(async {
                let mut bluetooth = hw::bt_check().await.as_ref().unwrap().clone();
                if bluetooth.State() == Ok(RadioState::Off) {
                    bluetooth.SetStateAsync(RadioState::On).expect("Failed to turn Bluetooth back on.");
                }
            });       
            
            // let _ = socket.send_to(msg, receiver_addr);

            // Poll shutdown event.
            match shutdown_rx.recv_timeout(Duration::from_secs(1)) {
                // Break the loop either upon stop or channel disconnect
                Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

                // Continue work if no events were received within the timeout
                Err(mpsc::RecvTimeoutError::Timeout) => (),
            };
        }

        // Tell the system that service has stopped.
        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        Ok(())
    }
}