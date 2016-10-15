//! The [mathematical typesetting table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/math.htm

mod constants;
mod glyphinfo;
mod variants;

pub use self::constants::Constants;
pub use self::glyphinfo::Glyphs;
pub use self::variants::Variants;

use layout::Device;

table! {
    @position
    #[doc = "The mathematical typesetting table."] 
    pub Mathematics { // Math Header Table
        header (Header), // Math Header Table 

        constants (Constants) |this, tape, position| {
            jump_take!(tape, position, this.header.constants_offset)
        },

        glyphs (Glyphs) |this, tape, position| {
            jump_take!(tape, position, this.header.glyph_info_offset)
        },

        variants (Variants) |this, tape, position| {
            jump_take!(tape, position, this.header.variants_offset)
        },
    }
}

table! {
    #[doc = "The mathematics header table."]
    #[derive(Copy)]
    pub Header { // Math Header Table
        major_version     (u16) = { 1 }, // MajorVersion
        minor_version     (u16) = { 0 }, // MinorVersion
        constants_offset  (u16), // MathConstants
        glyph_info_offset (u16), // MathGlyphInfo
        variants_offset   (u16), // MathVariants
    }
}

table! {
    @position
    #[doc = "A unit of measurement, in design units, along with an \
             optional corrections for various device resolutions."]
    pub Quantity { // MathValueRecord
        value         (i16),
        device_offset (u16), // DeviceTable

        device (Option<Device>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.device_offset)
        },
    }
}