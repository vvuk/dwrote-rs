/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::UnsafeCell;

use comptr::ComPtr;
use winapi;
use winapi::dwrite;
use std::mem;

use super::*;
use helpers::*;

#[derive(Debug)]
pub struct Font {
    native: UnsafeCell<ComPtr<dwrite::IDWriteFont>>,
}

impl Font {
    pub fn take(native: ComPtr<dwrite::IDWriteFont>) -> Font {
        Font {
            native: UnsafeCell::new(native)
        }
    }

    pub fn to_descriptor(&self) -> FontDescriptor {
        FontDescriptor {
            family_name: self.family_name(),
            stretch: self.stretch(),
            style: self.style(),
            weight: self.weight(),
        }
    }

    pub fn stretch(&self) -> FontStretch {
        unsafe {
            mem::transmute((*self.native.get()).GetStretch().0)
        }
    }

    pub fn style(&self) -> FontStyle {
        unsafe {
            mem::transmute((*self.native.get()).GetStyle().0)
        }
    }

    pub fn weight(&self) -> FontWeight {
        unsafe {
            mem::transmute((*self.native.get()).GetWeight().0)
        }
    }

    pub unsafe fn as_ptr(&self) -> *mut dwrite::IDWriteFont {
        (*self.native.get()).as_ptr()
    }

    pub fn family_name(&self) -> String {
        unsafe {
            let mut family: ComPtr<dwrite::IDWriteFontFamily> = ComPtr::new();
            let hr = (*self.native.get()).GetFontFamily(family.getter_addrefs());
            assert!(hr == 0);

            FontFamily::take(family).name()
        }
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
