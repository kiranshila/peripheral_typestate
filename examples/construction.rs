use peripheral_typestate::*;

struct MyBus {}
impl SomeEmbeddedBus for MyBus {}

fn main() {
    let bus = MyBus {};
    let dev = MyDevice::new(bus);
    dev.some_top_level_fn();

    let pins = dev.pins().unwrap();
    let p1 = pins.p1.into_configued();

    dev.some_top_level_fn();
    p1.something_only_configured_can_do();
    dev.some_top_level_fn();

    let _bus = dev.destroy();
    // Doesn't compile as dev was dropped
    // dev.some_top_level_fn();
    // p1.something_only_configured_can_do();
}
