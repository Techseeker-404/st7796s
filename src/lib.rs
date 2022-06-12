#![no_std]
// associated re-typing not supported in rust yet
#![allow(clippy::type_complexity)]
#![allow(non_camel_case_types)]

//! This crate provides a ST7796S driver to connect to TFT displays.

pub mod instruction;

use crate::instruction::Command;
use core::iter::once;

use display_interface::DataFormat::{U16BEIter, U8Iter};
use display_interface::WriteOnlyDataCommand;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::v2::OutputPin;

// TODO #[cfg(feature = "graphics")]
// mod graphics;

// TODO #[cfg(feature = "batch")]
// mod batch;

///
/// ST7796S driver to connect with TFT Display.
/// Using SPI protocol.
///
pub struct ST7796<DI, RST, BL>
where
    DI: WriteOnlyDataCommand,
    RST: OutputPin,
    BL: OutputPin,
{
    // Display Interface.
    di: DI,
    // Reset Pin.
    rst: Option<RST>,
    // Backlight Pin.
    bl: Option<BL>,
    // Visible size (x, y)
    size_x: u16,
    size_y: u16,
    // current orientation.
    orientation: Orientation,
}

/// Display Orientation to switch between 
/// Landscape, Portrait Modes.
#[repr(u8)] 
#[derive(Copy, Clone)]
pub enum Orientation {
    //D7=0,D6=0,D5=0,D4=0,D3=0,D2=0,D1=0,D0=0 (For More refer on Datasheet MADCTL pg.no:183)
    Portrait = 0b0000_0000, // No Inverting. Writing on none of the pins 
    //D7=0,D6=1,D5=1,D4=0,D3=0,D2=0,D1=0,D0=0
    Landscape = 0b0110_0000, // Invert column and Row/column order.
    //D7=1,D6=1,D5=0,D4=0,D3=0,D2=0,D1=0,D0=0
    PortraitSwapped = 0b1100_0000, // Invert Row and column order.
    //D7=1,D6=0,D5=1,D4=0,D3=0,D2=0,D1=0,D0=0
    LandscapeSwapped = 0b1010_0000, // Invert Row and Row/column order.
}

/// Default Screen orientation set as 
///  Landscape
impl Default for Orientation {
    fn default() -> Self {
        Self::Landscape
    }
}

///
/// Tearing Effect Setting.
///

#[derive(Copy, Clone)]
pub enum TearingEffect {
    /// Disable Output.
    Off,
    /// Output Vertical blanking information.
    Vertical,
    /// Output Horizontal and vertical blanking information.
    HorizontalVertical,
}

/// 
/// Backlight State Setting.
/// 
#[derive(Copy, Clone, Debug)]
pub enum BacklightState {
    ON,
    OFF,
}

///
/// Error Referring to its source (pins or SPI)
///
#[derive(Debug)]
pub enum Error<PinE> {
    DisplayError,
    Pin(PinE),
}
