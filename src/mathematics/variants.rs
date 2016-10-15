use truetype::GlyphID;
use layout::Coverage;
use super::Quantity;
use std::io::SeekFrom;

table! {
    @position
    #[doc = "A table of varaint glyphs to satisfy alternate measurements requirements."]
    pub Variants { // MathVariants
        min_connector_overlap      (u16),
        vertical_coverage_offset   (u16), // VertGlyphCoverage
        horizontal_coverage_offset (u16), // HorizGlyphCoverage
        vertical_count             (u16), // VertGlyphCount
        horizontal_count           (u16), // HorizGlyphCount

        vertical_constructions (Vec<Construction>) |this, tape, position| {
            let mut values: Vec<Construction> = Vec::with_capacity(this.vertical_count as usize);
            let offsets: Vec<u16> = try!(tape.take_given(this.vertical_count as usize));
            for n in offsets {
                values.push(jump_take!(@unwrap tape, position, n as usize));
            }
            try!(tape.seek(SeekFrom::Start(position + 10 + 2*this.vertical_count as u64)));
            Ok(values)
        },

        horizontal_constructions (Vec<Construction>) |this, tape, position| {
            let mut values: Vec<Construction> = Vec::with_capacity(this.horizontal_count as usize);
            let offsets: Vec<u16> = try!(tape.take_given(this.horizontal_count as usize));
            for n in offsets {
                values.push(jump_take!(@unwrap tape, position, n as usize));
            }
            Ok(values)
        },

        vertical_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.vertical_coverage_offset)
        },

        horizontal_coverage (Coverage) |this, tape, position| {
            jump_take!(tape, position, this.horizontal_coverage_offset)
        },
    }
}

table! {
    @position
    #[doc = "A table for assembling extended variants of a particular glyph."]
    pub Construction { // MathGlyphConstruction
        assembly_offset (u16), // GlyphAssembly
        count           (u16), // VariantCount

        variants (Vec<Variant>) |this, tape, _| {
            tape.take_given(this.count as usize)
        },

        assembly (Option<Assembly>) |this, tape, position| {
            jump_take_maybe!(tape, position, this.assembly_offset)
        },
    }
}

table! {
    #[doc = "A glyph’s measurement in the direction of extension (vertical or horizontal)."]
    #[derive(Copy)]
    pub Variant { // MathGlyphVariantRecord
        glyph_id    (GlyphID),
        measurement (u16    ), // AdvanceMeasurement
    }
}

table! {
    #[doc = "A table which specifies how the shape for a particular glyph can be \
             constructed from parts found in the glyph set"]
    pub Assembly { // GlyphAssembly
        italics_correction (Quantity),
        count              (u16     ), // PartCount

        parts (Vec<Part>) |this, tape| {
            tape.take_given(this.count as usize)
        },
    }
}

table! {
    #[doc = "A table used to align glyphs while extending."]
    #[derive(Copy)]
    pub Part { // GlyphPartRecord
        glyph_id               (GlyphID),
        start_connector_length (u16    ),
        end_connector_length   (u16    ),
        advance                (u16    ), // FullAdvance
        flags                  (u16    ), // PartFlags
    }
}