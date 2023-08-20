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
    // } else {
    //     // let error = Error::new(ErrorKind::Other.into(), "No radios found!".into());
    //     Err(Error::new(ErrorKind::Other.into(), "No radios found!".into()));
    // }
    // if bluetooth_radios.len() == 0 {
    //     // Err(String::from("Expected a bluetooth adapter."));
    //     let error = Error::new(ErrorKind::Other, "oh no!");
    //     Err(error);
    // }
    

    // Get the first bluetooth radio, assuming there is only one.
    // let bluetooth: Radio = bluetooth_radios[0];
    // bluetooth;

    // println!("Bluetooth state is currently: {:#?}", bluetooth.State()?);
    // println!("Listening for event...");

    // // Listen for StateChanged on the bluetooth adapter.
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

    // loop {
    //     continue;
    // }

    // Ok(())
}