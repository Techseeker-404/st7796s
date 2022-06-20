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

#[cfg(feature = "batch")]
mod batch;

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
 
// Trait Implementation of ST7796.

impl<DI, RST, BL, PinE> ST7796<DI, RST, BL>
where
    DI: WriteOnlyDataCommand,
    RST: OutputPin<Error=PinE>,
    BL: OutputPin<Error=PinE>,
{
    ///
    /// Creates a new ST7796 driver instance
    ///
    /// # Arguments.
    ///
    /// * `di` - Display Interface to communicate with display.
    /// * `rst` - Display hard reset pin.
    /// * `bl` - backlight pin.
    /// * `size_x` - x axis resolution of the display in pixels.
    /// * `size_y` - y axis resolution of the display in pixels.
    ///
    pub fn new(di: DI, rst: Option<RST>, bl: Option<BL>, size_x: u16, size_y: u16) -> Self {
        Self {
            di, rst, bl,
            size_x, size_y,
            orientation: Orientation::default(),
        }

    }

    /// 
    /// Runs commands to intialize the display
    ///
    /// # Arguments
    ///
    /// * `delay_source` - mutable reference to a delay provider.
    ///
    pub fn init(&mut self, delay_source: &mut impl DelayUs<u32>) -> Result<(), Error<PinE>> {
        self.hard_reset(delay_source)?;
        if let Some(bl) = self.bl.as_mut() {
            bl.set_low().map_err(Error::Pin)?;
            delay_source.delay_us(10_000);
            bl.set_high().map_err(Error::Pin)?;
        }

        //* TODO multiple write_commands *//
        self.write_command(Command::SWRESET)?; // Reset display
        delay_source.delay_us(150_000);
        self.write_command(Command::SLPOUT)?; // Turn OFF Sleep
        delay_source.delay_us(10_000);
        self.write_command(Command::INVOFF)?; // Turn OFF Invert
        self.write_command(Command::VSCRDER)?; // Vertical Scroll definition
        self.write_data(&[0u8, 0u8, 0x1Eu8, 0u8, 0u8, 0u8])?; // 0 TFA, 480 VSA, 0 BFA
        self.write_command(Command::MADCTL)?; // left -> right, bottom -> top RGB
        self.write_data(&[0b0000_0000])?;
        self.write_command(Command::PIXFMT)?; // 16bit 65k colors
        self.write_data(&[0b0101_0101])?;
        self.write_command(Command::INVON)?; // Turn ON Invert
        delay_source.delay_us(10_000);
        self.write_command(Command::NORON)?; // Turn ON Display
        delay_source.delay_us(10_000);
        self.write_command(Command::DISPON)?; // Turn ON Display
        delay_source.delay_us(10_000);

        Ok(())
    }

    ///
    /// Performs a hard reset using the RST pin sequence
    ///
    /// # Arguments
    ///
    /// * `delay_source` - mutable reference to a delay provider
    ///
    pub fn hard_reset(&mut self, delay_source: &mut impl DelayUs<u32>) -> Result<(), Error<PinE>> {
        if let Some(rst) = self.rst.as_mut() {
            rst.set_high().map_err(Error::Pin)?;
            delay_source.delay_us(10); // ensure the pin change will get registered
            rst.set_low().map_err(Error::Pin)?;
            delay_source.delay_us(10); // ensure the pin change will get registered
            rst.set_high().map_err(Error::Pin)?;
            delay_source.delay_us(10); // ensure the pin change will get registered
        }

        Ok(())
    }

    /// 
    /// Method to set the state of BacklightState
    ///
    pub fn set_backlight(
        &mut self, state: BacklightState, 
        delay_source: &mut impl DelayUs<u32>
    ) -> Result<(), Error<PinE>> {
        if let Some(bl) = self.bl.as_mut() {
            match state {
                BacklightState::ON => bl.set_high().map_err(Error::Pin)?,
                BacklightState::OFF => bl.set_low().map_err(Error::Pin)?,
            }
            delay_source.delay_us(10);
        }
    
        Ok(())
    }

    ///
    /// Returns the current state of display orientation.
    ///
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }
    
    ///
    /// Sets a new state of display orientation.
    ///
    pub fn set_orientation(&mut self, orientation: Orientation) -> Result<(), Error<PinE>> {
        self.write_command(Command::MADCTL)?;
        self.write_data(&[orientation as u8])?;
        self.orientation = orientation;

        Ok(())
    }
    
    /// 
    /// Sets a pixel color at the given coords.
    ///
    /// # Arguments
    ///
    /// * `x` - X  coordinate.
    /// * `y` - Y  coordinate.
    /// * `color` - the Rgb565 color value
    ///
    pub fn set_pixel(&mut self, x: u16, y:u16, color: u16) -> Result<(), Error<PinE>> {
        self.set_address_window(x, y, x, y)?;
        self.write_command(Command::RAMWR)?;
        self.di
            .send_data(U16BEIter(&mut once(color)))
            .map_err(|_| Error::DisplayError)?;

        Ok(())
    }
    
    ///
    /// Sets pixel colors in given rectangle bounds.
    ///
    /// # Arguments
    ///
    /// * `sx` - x coordinate start
    /// * `sy` - y coordinate start
    /// * `ex` - x coordinate end
    /// * `ey` - y coordinate end
    /// * `colors` - anything that can provide `IntoIterator<Item = u16>` to iterate over pixel data
    ///
    pub fn set_pixels<T>(
        &mut self,
        sx: u16, sy: u16,
        ex: u16, ey: u16,
        colors: T,
    ) -> Result<(), Error<PinE>>
    where
        T: IntoIterator<Item = u16>,
    {
        self.set_address_window(sx, sy, ex, ey)?;
        self.write_command(Command::RAMWR)?;
        self.di
            .send_data(U16BEIter(&mut colors.into_iter()))
            .map_err(|_| Error::DisplayError)
    }
    
    ///
    /// Sets scroll offset "shifting" the displayed picture
    /// # Arguments
    ///
    /// * `offset` - scroll offset in pixels
    ///
    pub fn set_scroll_offset(&mut self, offset: u16) -> Result<(), Error<PinE>> {
        self.write_command(Command::VSCRSADD)?;
        self.write_data(&offset.to_be_bytes())
    }

    ///
    /// Release resources allocated to this driver back.
    /// This returns the display interface and the RST pin; deconstructing the driver.
    ///
    pub fn release(self) -> (DI, Option<RST>, Option<BL>) {
        (self.di, self.rst, self.bl)
    }

    ///
    /// Configures the tearing effect output.
    ///
    pub fn set_tearing_effect(&mut self, tearing_effect: TearingEffect) -> Result<(), Error<PinE>> {
        match tearing_effect {
            TearingEffect::Off => self.write_command(Command::TEOFF),

            TearingEffect::Vertical => {
                self.write_command(Command::TEON)?;
                self.write_data(&[0])
            }

            TearingEffect::HorizontalVertical => {
                self.write_command(Command::TEON)?;
                self.write_data(&[1])
            }
        }

    }

    // --- Private Functions --- //

    /// Private method:Writing Data utilising the `send_commands` method of display_interface crate.
    fn write_command(&mut self, command: Command) -> Result<(), Error<PinE>> {
        self.di
            .send_commands(U8Iter(&mut once(command as u8)))
            .map_err(|_| Error::DisplayError)?;

        Ok(())
    }

    /// Private method:Writing Data utilising the `send_data` method of display_interface crate.
    fn write_data(&mut self, data: &[u8]) -> Result<(), Error<PinE>> {
        self.di
            .send_data(U8Iter(&mut data.iter().cloned()))
            .map_err(|_| Error::DisplayError)?;

        Ok(())
    }

    /// Private method:Sets the address window for the display.
    fn set_address_window(&mut self, sx: u16, sy: u16, ex: u16, ey: u16) -> Result<(), Error<PinE>> {
        self.write_command(Command::CASET)?;
        self.write_data(&sx.to_be_bytes())?;
        self.write_data(&ex.to_be_bytes())?;
        self.write_command(Command::RASET)?;
        self.write_data(&sy.to_be_bytes())?;
        self.write_data(&ey.to_be_bytes())?;

        Ok(())
    }

}
