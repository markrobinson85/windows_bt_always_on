use std::io::ErrorKind;

use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
    Devices::Radios::{Radio, RadioKind, RadioState, RadioAccessStatus},
    Foundation::Collections::IVectorView,
    Foundation::TypedEventHandler,
    // Devices::Radios::Radios
};

pub async fn bt_check() -> std::result::Result<Radio, std::io::Error> {
    println!("Searching for Bluetooth adapters...");
    
    // Get the radios on the machine.
    let radios = Radio::GetRadiosAsync()?.await?;

    // Filter the bluetooth radios into a vector.
    let bluetooth_radios = radios.into_iter().filter(|radio| {
        radio.Kind().unwrap() == RadioKind::Bluetooth
    }).collect::<Vec<_>>();
    
    
    Ok(bluetooth_radios[0].to_owned())

}