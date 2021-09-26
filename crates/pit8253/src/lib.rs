//! TODO(BSFishy): document this

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]

use core::{
    convert,
    fmt::{Debug, Formatter},
};
use spin::Mutex;
use x86_64::instructions::{interrupts::without_interrupts, port::Port};

static CHANNEL0: Mutex<Port<u8>> = Mutex::new(Port::new(0x40));
static CHANNEL1: Mutex<Port<u8>> = Mutex::new(Port::new(0x41));
static CHANNEL2: Mutex<Port<u8>> = Mutex::new(Port::new(0x42));
static MODE_REGISTER: Mutex<Port<u8>> = Mutex::new(Port::new(0x43));

const SIGNAL_HERTZ: f32 = 105f32 / 88f32 * 1_000_000f32;
const MIN_HERTZ: f32 = SIGNAL_HERTZ / (u16::MAX as f32);
const MAX_HERTZ: f32 = SIGNAL_HERTZ / 1f32;

#[inline]
fn to_reload_value(frequency: f32) -> u16 {
    (SIGNAL_HERTZ / frequency.clamp(MIN_HERTZ, MAX_HERTZ)).clamp(MIN_HERTZ, MAX_HERTZ) as u16
}

#[inline]
fn get_port(channel: Channel) -> &'static Mutex<Port<u8>> {
    match channel {
        Channel::Channel0 => &CHANNEL0,
        Channel::Channel1 => &CHANNEL1,
        Channel::Channel2 => &CHANNEL2,
    }
}

/// TODO(BSFishy): document this
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Channel {
    /// TODO(BSFishy): document this
    Channel0 = 0b00,
    /// TODO(BSFishy): document this
    Channel1 = 0b01,
    /// TODO(BSFishy): document this
    Channel2 = 0b10,
}

impl Default for Channel {
    #[inline]
    fn default() -> Self {
        Channel::from(0)
    }
}

impl<T> convert::From<T> for Channel
where
    T: Into<u8>,
{
    #[inline]
    fn from(value: T) -> Self {
        match value.into() {
            x if x == Channel::Channel0 as u8 => Channel::Channel0,
            x if x == Channel::Channel1 as u8 => Channel::Channel1,
            x if x == Channel::Channel2 as u8 => Channel::Channel2,
            x => panic!("Unknown channel: {}", x),
        }
    }
}

/// TODO(BSFishy): document this
pub struct PIT;

impl PIT {
    /// TODO(BSFishy): document this
    #[inline]
    pub fn read(&self, channel: Channel) -> u16 {
        self.set_mode_register(channel as u8, 0u8, 0u8);

        let mut port = get_port(channel).lock();
        without_interrupts(|| {
            let mut count: u16 = unsafe { port.read() as u16 };
            count |= unsafe { (port.read() as u16) << 8 };

            count
        })
    }

    /// TODO(BSFishy): document this
    #[inline]
    pub fn interrupt_on_terminal_count<T: Into<f32>>(&self, channel: Channel, frequency: T) {
        self.set_mode_register_access(channel as u8, 0b000);
        self.set_counter(channel, to_reload_value(frequency.into()));
    }

    /// TODO(BSFishy): document this
    #[inline]
    pub fn hardware_one_shot<T: Into<f32>>(&self, channel: Channel, frequency: T) {
        self.set_mode_register_access(channel as u8, 0b001);
        self.set_counter(channel, to_reload_value(frequency.into()));
    }

    /// TODO(BSFishy): document this
    #[inline]
    pub fn rate_generator<T: Into<f32>>(&self, channel: Channel, frequency: T) {
        self.set_mode_register_access(channel as u8, 0b010);
        self.set_counter(channel, to_reload_value(frequency.into()));
    }

    /// TODO(BSFishy): document this
    #[inline]
    pub fn square_wave_generator<T: Into<f32>>(&self, channel: Channel, frequency: T) {
        self.set_mode_register_access(channel as u8, 0b011);
        self.set_counter(channel, to_reload_value(frequency.into()));
    }

    /// TODO(BSFishy): document this
    #[inline]
    pub fn software_triggered_strobe<T: Into<f32>>(&self, channel: Channel, frequency: T) {
        self.set_mode_register_access(channel as u8, 0b100);
        self.set_counter(channel, to_reload_value(frequency.into()));
    }

    /// TODO(BSFishy): document this
    #[inline]
    pub fn hardware_triggered_strobe<T: Into<f32>>(&self, channel: Channel, frequency: T) {
        self.set_mode_register_access(channel as u8, 0b101);
        self.set_counter(channel, to_reload_value(frequency.into()));
    }

    #[inline]
    fn set_mode_register_access(&self, channel: u8, operating_mode: u8) {
        const ACCESS_MODE: u8 = 0b11;

        self.set_mode_register(channel, ACCESS_MODE, operating_mode);
    }

    #[inline]
    fn set_mode_register(&self, channel: u8, access_mode: u8, operating_mode: u8) {
        const BINARY_MODE: u8 = 0b0;

        let value: u8 = (channel << 6) | (access_mode << 4) | (operating_mode << 1) | BINARY_MODE;
        without_interrupts(|| unsafe {
            MODE_REGISTER.lock().write(value);
        });
    }

    #[inline]
    fn set_counter(&self, channel: Channel, reload_value: u16) {
        without_interrupts(|| {
            let mut port = get_port(channel).lock();

            unsafe {
                port.write((reload_value & 0x00FF) as u8);
                port.write(((reload_value & 0xFF00) >> 8) as u8);
            }
        });
    }
}

impl Debug for PIT {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PIT")
            .field("channel0", &self.read(Channel::Channel0))
            .field("channel1", &self.read(Channel::Channel1))
            .field("channel2", &self.read(Channel::Channel2))
            .finish()
    }
}
