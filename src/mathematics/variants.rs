table! {
    #[doc = "A table of varaint glyphs to satisfy alternate measurements requirements."]
    #[derive(Copy)]
    pub Variants { // MathVariants
        min_connector_overlap      (i16),
        vertical_coverage_offset   (u16), // VertGlyphCoverage
        horizontal_coverage_offset (u16), // HorizGlyphCoverage
        vertical_count             (u16), // VertGlyphCount
        horizontal_count           (u16), // HorizGlyphCount
    }
}