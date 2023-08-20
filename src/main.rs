
extern crate windows_service;

mod services;
use services::bt_service::*;

// define_windows_service!(ffi_service_main, bt_service_main);

#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    use std::{ffi::OsString};
    use windows_service::{
        service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType},
        service_manager::{ServiceManager, ServiceManagerAccess},
    };

    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;
    
    let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;
    let service = service_manager.open_service("bt_always_on_service", service_access);
    
    match service {
        Ok(_value) => {
            // Service is installed, we can run it.
            let _ = bt_always_on_service::run();
        },
        Err(_error) => {
            println!("raw OS error: {_error:?}");
            // TODO: We need to check that the error is because the service doesn't exist.
            // Winapi(Os { code: 1060, kind: Uncategorized, message: "The specified service does not exist as an installed service." })
            // let err: Error = _error.source()

            // let err = _error.source().unwrap();

            // if let Some(raw_os_err) = err.raw_os_error() {
            //     println!("raw OS error: {raw_os_err:?}");
            // }
            // println!("{:#?}", err.raw_os_error.unwrap());
            // Service is not installed, let's install it.
            // This example installs the service defined in `examples/bt_always_on_service.rs`.
            // In the real world code you would set the executable path to point to your own binary
            // that implements windows service.
            let service_binary_path = ::std::env::current_exe()
                .unwrap()
                .with_file_name("win_bt_always_on.exe");

            let service_info = ServiceInfo {
                name: OsString::from("bt_always_on_service"),
                display_name: OsString::from("Bluetooth Always-On"),
                service_type: ServiceType::OWN_PROCESS,
                start_type: ServiceStartType::AutoStart,
                error_control: ServiceErrorControl::Normal,
                executable_path: service_binary_path,
                launch_arguments: vec![],
                dependencies: vec![],
                account_name: None, // run as System
                account_password: None,
            };
            let service = service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)?;
            service.set_description("Service to keep the Bluetooth adapter turned on, even when it is turned off.")?;
        }
    }
    
    Ok(())
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
