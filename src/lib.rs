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
    Portrait = 0b0000_0000, // No Inverting.
    Landscape = 0b0110_0000, // Invert column and Row/column order.
    PortraitSwapped = 0b1100_0000, // Invert Row and column order.
    LandscapeSwapped = 0b1010_0000, // Invert Row and Row/column order.
}

