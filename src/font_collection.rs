/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use comptr::ComPtr;
use winapi::dwrite;
use winapi::FALSE;
use std::cell::UnsafeCell;

use super::{DWriteFactory, FontFamily, Font, FontFace};

pub struct FontCollectionFamilyIterator {
    collection: ComPtr<dwrite::IDWriteFontCollection>,
    curr: u32,
    count: u32,
}

impl Iterator for FontCollectionFamilyIterator {
    type Item = FontFamily;
    fn next(&mut self) -> Option<FontFamily> {
        if self.curr == self.count {
            return None;
        }

        unsafe {
            let mut family: ComPtr<dwrite::IDWriteFontFamily> = ComPtr::new();
            let hr = self.collection.GetFontFamily(self.curr, family.getter_addrefs());
            assert!(hr == 0);
            self.curr += 1;
            Some(FontFamily::take(family))
        }
    }
}

pub struct FontCollection {
    native: UnsafeCell<ComPtr<dwrite::IDWriteFontCollection>>,
}

impl FontCollection {
    pub fn system() -> FontCollection {
        unsafe {
            let mut native: ComPtr<dwrite::IDWriteFontCollection> = ComPtr::new();
            let hr = (*DWriteFactory()).GetSystemFontCollection(native.getter_addrefs(), FALSE);
            assert!(hr == 0);

            FontCollection {
                native: UnsafeCell::new(native)
            }
        }
    }

    pub unsafe fn as_ptr(&self) -> *mut dwrite::IDWriteFontCollection {
        (*self.native.get()).as_ptr()
    }

    pub fn families_iter(&self) -> FontCollectionFamilyIterator {
        unsafe {
            FontCollectionFamilyIterator {
                collection: (*self.native.get()).clone(),
                curr: 0,
                count: (*self.native.get()).GetFontFamilyCount(),
            }
        }
    }

    pub fn get_font_from_face(&self, face: &FontFace) -> Option<Font> {
        unsafe {
            let mut font: ComPtr<dwrite::IDWriteFont> = ComPtr::new();
            let hr = (*self.native.get()).GetFontFromFontFace(face.as_ptr(), font.getter_addrefs());
            if hr != 0 {
                return None;
            }
            Some(Font::take(font))
        }
    }
}
