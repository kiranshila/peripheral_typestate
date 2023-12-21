#![no_std]

use core::cell::RefCell;

use pins::Pins;

pub struct MyDevice<BUS> {
    bus: RefCell<BUS>,
    has_pins: bool,
}

pub trait SomeEmbeddedBus {}

impl<BUS> MyDevice<BUS>
where
    BUS: SomeEmbeddedBus,
{
    pub fn new(bus: BUS) -> Self {
        Self {
            bus: RefCell::new(bus),
            has_pins: true,
        }
    }

    pub fn some_top_level_fn(&self) {
        let bus = &mut *self.bus.borrow_mut();
        // Do something with bus
    }

    pub fn destroy(self) -> BUS {
        self.bus.into_inner()
    }
}

pub mod pins {
    use core::{cell::RefCell, marker::PhantomData};

    pub mod typestate {
        pub struct DefaultState;
        pub struct ConfiguredState;
    }
    use typestate::*;

    pub struct Pin<'parent, BUS, MODE, const N: u8> {
        _mode: PhantomData<MODE>,
        bus: &'parent RefCell<BUS>,
    }

    // Specific implementations for the states
    impl<'parent, BUS, const N: u8> Pin<'parent, BUS, DefaultState, N> {
        pub fn something_only_default_can_do(&self) {
            let bus = &mut *self.bus.borrow_mut();
            // Do something with bus
        }
    }

    impl<'parent, BUS, const N: u8> Pin<'parent, BUS, ConfiguredState, N> {
        pub fn something_only_configured_can_do(&self) {
            let bus = &mut *self.bus.borrow_mut();
            // Do something with bus
        }
    }

    impl<'parent, BUS, MODE, const N: u8> Pin<'parent, BUS, MODE, N> {
        // Private constructor to ensure uniqueness of the pins
        fn new(bus: &'parent RefCell<BUS>) -> Self {
            Self {
                _mode: PhantomData,
                bus,
            }
        }

        // Consuming transformations maintain uniqueness
        pub fn into_default(self) -> Pin<'parent, BUS, DefaultState, N> {
            let bus = &mut *self.bus.borrow_mut();
            // Do something with bus
            Pin::new(self.bus)
        }

        pub fn into_configued(self) -> Pin<'parent, BUS, ConfiguredState, N> {
            let bus = &mut *self.bus.borrow_mut();
            // Do something with bus
            Pin::new(self.bus)
        }
    }

    pub struct Pins<'parent, BUS> {
        pub p0: Pin<'parent, BUS, DefaultState, 0>,
        pub p1: Pin<'parent, BUS, DefaultState, 1>,
    }

    #[allow(clippy::new_without_default)]
    impl<'parent, BUS> Pins<'parent, BUS> {
        pub fn new(bus: &'parent RefCell<BUS>) -> Self {
            Self {
                p0: Pin::new(bus),
                p1: Pin::new(bus),
            }
        }
    }
}

impl<BUS> MyDevice<BUS>
where
    BUS: SomeEmbeddedBus,
{
    pub fn pins(&mut self) -> Option<Pins<'_, BUS>> {
        if self.has_pins {
            self.has_pins = false;
            Some(pins::Pins::new(&self.bus))
        } else {
            None
        }
    }
}
