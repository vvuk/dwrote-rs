/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use comptr::ComPtr;
use winapi::dwrite;
use std::cell::UnsafeCell;

use helpers::*;

pub struct FontFamily {
    native: UnsafeCell<ComPtr<dwrite::IDWriteFontFamily>>,
}

impl FontFamily {
    pub fn take(native: ComPtr<dwrite::IDWriteFontFamily>) -> FontFamily {
        FontFamily {
            native: UnsafeCell::new(native)
        }
    }

    pub fn name(&self) -> String {
        unsafe {
            let mut family_names: ComPtr<dwrite::IDWriteLocalizedStrings> = ComPtr::new();
            let hr = (*self.native.get()).GetFamilyNames(family_names.getter_addrefs());
            assert!(hr == 0);

            get_locale_string(&mut family_names)
        }
    }
}
