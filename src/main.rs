use evdev::{AbsInfo, AbsoluteAxisType, EventType, InputEvent, InputEventKind, Key, uinput::VirtualDeviceBuilder, UinputAbsSetup};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let abs_setup = AbsInfo::new(0, 0, 1, 0, 0, 1);
    let btn1 = UinputAbsSetup::new(AbsoluteAxisType::ABS_HAT0X, abs_setup);
    let btn2 = UinputAbsSetup::new(AbsoluteAxisType::ABS_HAT0Y, abs_setup);

    let mut output_device = VirtualDeviceBuilder::new()?
        .name("Virtual analog buttons")
        .with_absolute_axis(&btn1)?
        .with_absolute_axis(&btn2)?
        .build()
        .unwrap();

    for path in output_device.enumerate_dev_nodes_blocking()? {
        let path = path?;
        println!("Virtual device created as {}", path.display());
    }

    let (input_path, mut input_device) = evdev::enumerate().find(|(_path, device)| {
        device.supported_keys().is_some_and(|keys| {
            keys.contains(Key::BTN_EXTRA) && keys.contains(Key::BTN_SIDE)
        })
    }).ok_or("Could not find input device")?;

    println!("Input device found on path {}", input_path.display());
    if let Some(name) = input_device.name() {
        println!("\t Selected device: {}", name);
    }

    loop {
        for event in input_device.fetch_events()? {
            match (event.kind(), event.value()) {
                (InputEventKind::Key(Key::BTN_EXTRA), 0) => output_device.emit(&[InputEvent::new(EventType::ABSOLUTE, btn1.code(), 0)])?,
                (InputEventKind::Key(Key::BTN_EXTRA), 1) => output_device.emit(&[InputEvent::new(EventType::ABSOLUTE, btn1.code(), 1)])?,
                (InputEventKind::Key(Key::BTN_SIDE), 0) => output_device.emit(&[InputEvent::new(EventType::ABSOLUTE, btn2.code(), 0)])?,
                (InputEventKind::Key(Key::BTN_SIDE), 1) => output_device.emit(&[InputEvent::new(EventType::ABSOLUTE, btn2.code(), 1)])?,
                _ => {}
            }
        }
    }
}
