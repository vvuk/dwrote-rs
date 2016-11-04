/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use winapi;

use super::*;

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
    let arial_font = arial_family.get_first_matching_font(winapi::DWRITE_FONT_WEIGHT_NORMAL,
                                                          winapi::DWRITE_FONT_STRETCH_NORMAL,
                                                          winapi::DWRITE_FONT_STYLE_NORMAL);

    let descriptor = arial_font.to_descriptor();
    assert!(descriptor.family_name == "Arial");

    let arial_font_2 = system_fc.get_font_from_descriptor(&descriptor).unwrap();
    let descriptor2 = arial_font_2.to_descriptor();
    assert_eq!(descriptor, descriptor2);
}
