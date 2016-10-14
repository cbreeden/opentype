use truetype::{Result, Tape, Walue};
use std::io::SeekFrom;

use layout::Coverage;
use super::Quantity;

table! {
    @position
    #[doc = "A table of positioning information defined on a per-glyph basis."]
    pub Glyphs { // MathGlyphInfo
        italics_corrections_offset     (u16), // MathItalicsCorrectionInfo
        top_accent_attachments_offset  (u16), // MathTopAccentAttachment
        extended_shape_coverage_offset (u16), // ExtendedShapeCoverage
        kernings_offset                (u16), // MathKernInfo

        corrections (Corrections) |this, tape, position| {
            jump_take!(tape, position, this.italics_corrections_offset)        
        },

        attachments (Attachments) |this, tape, position| {
            jump_take!(tape, position, this.top_accent_attachments_offset)
        },

        extended_shape_coverage (Option<Coverage>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.extended_shape_coverage_offset)
        },

        // According to the specification, kernings are not optional.  However,
        // some popular OpenType fonts do not support kernings.
        kernings (Option<Kernings>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.kernings_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table of italics corrections."]
    pub Corrections { // MathItalicsCorrectionInfo
        coverage_offset (u16), // Coverage
        count           (u16), // ItalicsCorrectionCount

        values (Vec<Quantity>) |this, tape, _| { // ItalicsCorrection
            tape.take_given(this.count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table of horizontal positioning for top accents."]
    pub Attachments { // MathTopAccentAttachment
        coverage_offset (u16), // TopAccentCoverage
        count           (u16), // TopAccentAttachmentCount

        values (Vec<Quantity>) |this, tape, _| {    
           tape.take_given(this.count as usize)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table of kerning information for mathematical glyphs."]
    pub Kernings {
        coverage_offset (u16), // MathKernCoverage
        count           (u16), // MathKernCount

        records (Vec<Kerning>) |this, tape, position| {
            let mut values: Vec<Kerning> = Vec::with_capacity(this.count as usize);
            println!("Kern count: {}",  this.count);
            for count in 0..(this.count as u64) {
                values.push(try!(tape.take_given((position, count))));
            }
            Ok(values)
        },

        coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.coverage_offset)
        },
    }
}

table! {
    @define
    #[doc = "Kerning information for a mathematical glyph."]
    pub Kerning { // MathKernInfoRecord
        top_right_offset    (u16), // TopRightMathKern
        top_left_offset     (u16), // TopLeftMathKern
        bottom_right_offset (u16), // BottomRightMathKern
        bottom_left_offset  (u16), // BottomLeftMathKern

        top_right           (Option<KerningValues>),
        top_left            (Option<KerningValues>),
        bottom_right        (Option<KerningValues>),
        bottom_left         (Option<KerningValues>),
    }
}

table! {
    #[doc = "A table of kerning values for a various glyph heights."]
    pub KerningValues { // MathKern
        count (u16), // HeightCount

        correction_heights (Vec<Quantity>) |this, tape| {
            tape.take_given(this.count as usize)
        },

        values (Vec<Quantity>) |this, tape| {
            tape.take_given(this.count as usize + 1)
        },
    }
}

impl Walue<'static> for Kerning {
    type Parameter = (u64, u64);
    
    fn read<T: Tape>(tape: &mut T, (position, count): Self::Parameter) -> Result<Self> {
        let top_right_offset = try!(tape.take());
        let top_left_offset = try!(tape.take());
        let bottom_right_offset = try!(tape.take());
        let bottom_left_offset = try!(tape.take());
        let top_right = jump_take_maybe!(@unwrap tape, position, top_right_offset);
        let top_left = jump_take_maybe!(@unwrap tape, position, top_left_offset);
        let bottom_right = jump_take_maybe!(@unwrap tape, position, bottom_right_offset);
        let bottom_left = jump_take_maybe!(@unwrap tape, position, bottom_left_offset);

        try!(tape.seek(SeekFrom::Start(position + 8*(count + 1))));

        Ok(Kerning {
            top_right_offset: top_right_offset,
            top_left_offset: top_left_offset,
            bottom_right_offset: bottom_right_offset,
            bottom_left_offset: bottom_left_offset,
            top_right: top_right,
            top_left: top_left,
            bottom_right: bottom_right,
            bottom_left: bottom_left,
        })
    }
}