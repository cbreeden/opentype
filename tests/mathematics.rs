use opentype::mathematics::Mathematics;
use opentype::layout::Coverage;
use truetype::Value;

use std::io::{Seek, SeekFrom};
use std::fs::File;

macro_rules! read_math_table {
    (XITS) => {
        {
            let mut file = ok!(File::open("tests/fixtures/XITS-Math.otf"));
            ok!(file.seek(SeekFrom::Start(496284)));
            ok!(Value::read(&mut file))
        }
    };

    (LatinModern) => {
        {
            let mut file = ok!(File::open("tests/fixtures/latinmodern-math.otf"));
            ok!(file.seek(SeekFrom::Start(689248)));
            ok!(Value::read(&mut file))
        }
    };
}

#[test]
fn table() {
    let math: Mathematics = read_math_table!(XITS);

    assert_eq!(math.header.constants_offset, 10);
    assert_eq!(math.header.glyph_info_offset, 224);
    assert_eq!(math.header.variants_offset, 10152);
}

#[test]
fn constants() {
    let math: Mathematics = read_math_table!(XITS);
    let constants = math.constants;

    assert_eq!(constants.script_percent_scale_down, 75);
    assert_eq!(constants.delimited_sub_formula_min_height, 1500);
    assert_eq!(constants.math_leading.value, 150);
    assert_eq!(constants.radical_kern_after_degree.value, -555);
    assert_eq!(constants.radical_degree_bottom_raise_percent, 70);    
}

#[test]
fn glyphs() {
    let math: Mathematics = read_math_table!(XITS);
    let glyphs = math.glyphs;

    match glyphs.corrections.coverage {
        Coverage::Format2(_) => (),
        _ => panic!("Parsed incorrect coverage table format for italics corrections."),
    }
    assert_eq!(glyphs.corrections.count, 643);
    assert_eq!(glyphs.corrections.values[0].value, 100);
    assert_eq!(glyphs.corrections.values[642].value, 80);

    match glyphs.attachments.coverage {
        Coverage::Format2(_) => (),
        _ => panic!("Parsed incorrect coverage table format for accent attachments."),
    }
    assert_eq!(glyphs.attachments.count, 1328);
    assert_eq!(glyphs.attachments.values[0].value, 361);
    assert_eq!(glyphs.attachments.values[1327].value, 201);

    if let Some(extended_shapes) = glyphs.extended_shape_coverage {
        match extended_shapes {
            Coverage::Format2(_) => (),
            _ => panic!("Parsed incorrect coverage table format for extended shapes."),
        }
    }

    let kernings = match glyphs.kernings {
        Some(kernings) => kernings,
        None => panic!("No kerning table found."),
    };
    assert_eq!(kernings.count, 29);

    let top_right = kernings.records[0].clone().top_right.unwrap();
    assert_eq!(top_right.correction_heights[0].value, 275);
    assert_eq!(top_right.values[0].value, -100);
    assert_eq!(top_right.values[1].value, -150);
    assert_eq!(kernings.records[0].clone().bottom_right.unwrap().values[0].value, 50);
}

#[test]
fn variants() {
    let math: Mathematics = read_math_table!(XITS);
    let variants = math.variants;

    assert_eq!(variants.min_connector_overlap, 50);
    assert_eq!(variants.horizontal_count, 41);
    assert_eq!(variants.vertical_count, 166);
}