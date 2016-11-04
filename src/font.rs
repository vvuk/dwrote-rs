/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use comptr::ComPtr;
use winapi::dwrite;
use std::cell::UnsafeCell;

use super::{FontFace};
use helpers::*;

pub struct Font {
    native: UnsafeCell<ComPtr<dwrite::IDWriteFont>>,
}

impl Font {
    pub fn take(native: ComPtr<dwrite::IDWriteFont>) -> Font {
        Font {
            native: UnsafeCell::new(native)
        }
    }

    pub unsafe fn as_ptr(&self) -> *mut dwrite::IDWriteFont {
        (*self.native.get()).as_ptr()
    }

    pub fn face_name(&self) -> String {
        unsafe {
            let mut names: ComPtr<dwrite::IDWriteLocalizedStrings> = ComPtr::new();
            let hr = (*self.native.get()).GetFaceNames(names.getter_addrefs());
            assert!(hr == 0);

            get_locale_string(&mut names)
        }
    }

    pub fn create_font_face(&self) -> FontFace {
        unsafe {
            let mut face: ComPtr<dwrite::IDWriteFontFace> = ComPtr::new();
            let hr = (*self.native.get()).CreateFontFace(face.getter_addrefs());
            assert!(hr == 0);
            FontFace::take(face)
        }
    }
}
