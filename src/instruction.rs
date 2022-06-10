/// ST7796S instructions.
#[repr(u8)]
pub enum Command {
    NOP = 0x00,
    SWRESET = 0x01,

    RDDID = 0x04,
    RDDST = 0x09,
    RDMODE = 0x0A,
    RDMADCTL = 0x0B,
    RDPIXFMT = 0x0C,
    RDIMGFMT = 0x0D,
    RDSELFDIAG = 0x0F,

    SLPIN = 0x10,
    SLPOUT = 0x11,
    PTLON = 0x12,
    NORON = 0x13,

    INVOFF = 0x20,
    INVON = 0x21, //   S_GAMMASET = 0x26,
    DISPOFF = 0x28,
    DISPON = 0x29,

    CASET = 0x2A,
    PASET = 0x2B,
    RAMWR = 0x2C,
    RAMRD = 0x2E,

    PTLAR = 0x30,
    VSCRDEF = 0x33,
    MADCTL = 0x36,
    VSCRSADD = 0x37,     /* Vertical Scrolling Start Address */
    PIXFMT = 0x3A,    /* COLMOD: Pixel Format Set */

    RGB_INTERFACE = 0xB0,     /* RGB Interface Signal Control */
    FRMCTR1 = 0xB1,
    FRMCTR2 = 0xB2,
    FRMCTR3 = 0xB3,
    INVCTR = 0xB4,
    DFUNCTR = 0xB6,     /* Display Function Control */

    PWCTR1 = 0xC0,
    PWCTR2 = 0xC1,
    PWCTR3 = 0xC2,
    PWCTR4 = 0xC3,
    PWCTR5 = 0xC4,
    VMCTR1 = 0xC5,

    RDID1 = 0xDA,
    RDID2 = 0xDB,
    RDID3 = 0xDC,
    RDID4 = 0xDD,

    GMCTRP1 = 0xE0,
    GMCTRN1 = 0xE1,
    DGCTR1 = 0xE2,
    DGCTR2 = 0xE3,
    MAD_RGB = 0x08,
    MAD_X_RIGHT = 0x40,
    MAD_Y_UP = 0x80,
}

   // MAD_BGR = 0x00,

   // MAD_VERTICAL = 0x20,
   // MAD_X_LEFT = 0x00,
   // MAD_Y_DOWN = 0x00,
