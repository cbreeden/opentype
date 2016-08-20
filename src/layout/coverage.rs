use truetype::GlyphID;

use {Result, Tape, Value};

/// A coverage table.
#[derive(Clone, Debug)]
pub enum Coverage {
    /// Format 1.
    Format1(Coverage1),
    /// Format 2.
    Format2(Coverage2),
}

table! {
    #[doc = "A coverage table in format 1."]
    pub Coverage1 {
        format (u16) = { 1 }, // CoverageFormat
        count  (u16), // GlyphCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // GlyphArray
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A coverage table in format 2."]
    pub Coverage2 {
        format (u16) = { 2 }, // CoverageFormat
        count  (u16), // RangeCount

        ranges (Vec<CoverageRange>) |this, tape| { // RangeRecord
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A coverage range."]
    #[derive(Copy)]
    pub CoverageRange { // RangeRecord
        start (GlyphID), // Start
        end   (GlyphID), // End
        index (u16    ), // StartCoverageIndex
    }
}

impl Value for Coverage {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            1 => Coverage::Format1(try!(tape.take())),
            2 => Coverage::Format2(try!(tape.take())),
            _ => raise!("found an unknown format of the coverage table"),
        })
    }
}
