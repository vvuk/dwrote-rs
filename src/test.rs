/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;
use truetype;
use std::io::Cursor;
use truetype::Value;
use std::cmp::{min, max};

#[test]
fn test_system_family_iter() {
    let system_fc = FontCollection::system();
    let count = system_fc.families_iter().count();
    assert!(count > 0);
    assert!(system_fc.families_iter().find(|f| f.name() == "Arial").is_some());
}

#[test]
fn test_descriptor_round_trip() {
    let system_fc = FontCollection::system();

    let arial_family = system_fc.get_font_family_by_name("Arial").unwrap();
    let arial_font = arial_family.get_first_matching_font(FontWeight::Regular,
                                                          FontStretch::Normal,
                                                          FontStyle::Normal);

    let descriptor = arial_font.to_descriptor();
    assert!(descriptor.family_name == "Arial");

    let arial_font_2 = system_fc.get_font_from_descriptor(&descriptor).unwrap();
    let descriptor2 = arial_font_2.to_descriptor();
    assert_eq!(descriptor, descriptor2);
}

#[test]
fn test_get_font_file_bytes() {
    let system_fc = FontCollection::system();

    let arial_family = system_fc.get_font_family_by_name("Arial").unwrap();
    let arial_font = arial_family.get_first_matching_font(FontWeight::Regular,
                                                          FontStretch::Normal,
                                                          FontStyle::Normal);
    let face = arial_font.create_font_face();
    let files = face.get_files();
    assert!(files.len() > 0);

    let bytes = files[0].get_font_file_bytes();
    assert!(bytes.len() > 0);
}

#[test]
fn test_glyph_image() {
    let system_fc = FontCollection::system();
    let arial_family = system_fc.get_font_family_by_name("Arial").unwrap();
    let arial_font = arial_family.get_first_matching_font(FontWeight::Regular,
                                                          FontStretch::Normal,
                                                          FontStyle::Normal);

    let face = arial_font.create_font_face();
    let a_index = face.get_glyph_indices(&['A' as u32])[0];

    let metrics = face.get_metrics();
    println!("Metrics:\n======\n{:?}\n======", metrics);

    let gm = face.get_design_glyph_metrics(&[a_index], false)[0];
    println!("Glyph metrics:\n======\n{:?}\n======", gm);

    let device_pixel_ratio = 1.0f32;
    let em_size = 10.0f32;

    let design_units_per_pixel = face.metrics().designUnitsPerEm as f32 / 16. as f32;
    let scaled_design_units_to_pixels = (em_size * device_pixel_ratio) / design_units_per_pixel;

    let width = (gm.advanceWidth as i32 - (gm.leftSideBearing + gm.rightSideBearing)) as f32 * scaled_design_units_to_pixels;
    let height = (gm.advanceHeight as i32 - (gm.topSideBearing + gm.bottomSideBearing)) as f32 * scaled_design_units_to_pixels;
    let x = (-gm.leftSideBearing) as f32 * scaled_design_units_to_pixels;
    let y = (gm.verticalOriginY - gm.topSideBearing) as f32 * scaled_design_units_to_pixels;

    // FIXME I'm pretty sure we need to do a proper RoundOut type
    // operation on this rect to properly handle any aliasing
    let left_i = x.floor() as i32;
    let top_i = (height - y).floor() as i32;
    let width_u = width.ceil() as u32;
    let height_u = height.ceil() as u32;

    println!("GlyphDimensions: {} {} {} {}", left_i, top_i, width_u, height_u);

    let gdi_interop = GdiInterop::create();
    let rt = gdi_interop.create_bitmap_render_target(width_u, height_u);
    let rp = RenderingParams::create_for_primary_monitor();
    rt.set_pixels_per_dip(device_pixel_ratio);
    rt.draw_glyph_run(x as f32, y as f32,
                      DWRITE_MEASURING_MODE_NATURAL,
                      &face,
                      em_size,
                      &[a_index],
                      &[0f32],
                      &[GlyphOffset { advanceOffset: 0., ascenderOffset: 0. }],
                      &rp,
                      &(255.0f32, 255.0f32, 255.0f32));
    let bytes = rt.get_opaque_values_as_mask();
    println!("bytes length: {}", bytes.len());
}

type FontTableTag = u32;

fn make_tag(tag_bytes: &[u8]) -> FontTableTag {
    assert!(tag_bytes.len() == 4);
    unsafe { *(tag_bytes.as_ptr() as *const FontTableTag) }
}

//macro_rules! try_lossy(($result:expr) => (try!($result.map_err(|_| (())))));
macro_rules! try_lossy(($result:expr) => (try!($result.map_err(|_| panic!("Failed")))));

#[derive(Debug)]
struct FontInfo {
    family_name: String,
    face_name: String,
    weight: u32,
    stretch: u32,
    style: FontStyle,
}

impl FontInfo {
    fn new_from_face(face: &FontFace) -> Result<FontInfo, ()> {
        let mut info = FontInfo {
            family_name: "".to_owned(),
            face_name: "".to_owned(),
            weight: 0,
            stretch: 0,
            style: FontStyle::Normal,
        };

        if let Some(name_table_bytes) = face.get_font_table(make_tag(b"name")) {
            use truetype::NamingTable;
            println!("name table len {}", name_table_bytes.len());
            let mut table = Cursor::new(&name_table_bytes);
            let names = try_lossy!(NamingTable::read(&mut table));
            let (family, face) = match names {
                NamingTable::Format0(ref table) => {
                    if table.count < 3 { return Err(()); }
                    let strings = try_lossy!(table.strings());
                    let family = strings[1].clone();
                    let face = strings[2].clone();
                    (family, face)
                },
                NamingTable::Format1(ref table) => {
                    if table.count < 3 { return Err(()); }
                    let strings = try_lossy!(table.strings());
                    let family = strings[1].clone();
                    let face = strings[2].clone();
                    (family, face)
                }
            };
            info.family_name = family;
            info.face_name = face;
        } else {
            return Err(());
        }

        if let Some(os2_table_bytes) = face.get_font_table(make_tag(b"OS/2")) {
            use truetype::WindowsMetrics;
            let mut table = Cursor::new(&os2_table_bytes);
            let metrics = try_lossy!(WindowsMetrics::read(&mut table));
            let (weight_val, width_val, italic_bool) = match metrics {
                WindowsMetrics::Version3(ref m) => {
                    (m.weight_class, m.width_class, m.selection_flags.0 & 1 == 1)
                },
                WindowsMetrics::Version5(ref m) => {
                    (m.weight_class, m.width_class, m.selection_flags.0 & 1 == 1)
                },
            };

            info.weight = min(9, max(1, weight_val / 100)) as u32;
            info.stretch = min(9, max(1, width_val)) as u32;
            info.style = if italic_bool {
                FontStyle::Italic
            } else {
                FontStyle::Normal
            };
        } else {
            return Err(());
        }

        Ok(info)
    }
}

#[test]
fn test_create_font_file_from_bytes() {
    let system_fc = FontCollection::system();

    let arial_family = system_fc.get_font_family_by_name("Arial").unwrap();
    let arial_font = arial_family.get_first_matching_font(FontWeight::Regular,
                                                          FontStretch::Normal,
                                                          FontStyle::Normal);
    let face = arial_font.create_font_face();
    let files = face.get_files();
    assert!(files.len() > 0);

    let bytes = files[0].get_font_file_bytes();
    assert!(bytes.len() > 0);

    // now go back
    let new_font = FontFile::new_from_data(&bytes);
    assert!(new_font.is_some());

    let new_font = new_font.unwrap();

    let info = FontInfo::new_from_face(&new_font.create_face(0, super::DWRITE_FONT_SIMULATIONS_NONE));
    println!("{:?}", info);
}
