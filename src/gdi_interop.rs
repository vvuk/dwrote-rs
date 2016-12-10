/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::ptr;
use std::cell::UnsafeCell;

use comptr::ComPtr;
use winapi;
use super::{BitmapRenderTarget, DWriteFactory, Font, FontFace};

#[derive(Debug)]
pub struct GdiInterop {
    native: UnsafeCell<ComPtr<winapi::IDWriteGdiInterop>>,
}

impl GdiInterop {
    pub fn create() -> GdiInterop {
        unsafe {
            let mut native: ComPtr<winapi::IDWriteGdiInterop> = ComPtr::new();
            let hr = (*DWriteFactory()).GetGdiInterop(native.getter_addrefs());
            assert!(hr == 0);
            GdiInterop::take(native)
        }
    }

    pub fn take(native: ComPtr<winapi::IDWriteGdiInterop>) -> GdiInterop {
        GdiInterop {
            native: UnsafeCell::new(native),
        }
    }

    pub fn create_font_from_logfont(&self, logfont: &winapi::LOGFONTW) -> Font {
        unsafe {
            let mut native: ComPtr<winapi::IDWriteFont> = ComPtr::new();
            let logfont = logfont as *const _;
            let hr = (*self.native.get()).CreateFontFromLOGFONT(logfont, native.getter_addrefs());
            assert!(hr == 0);
            Font::take(native)
        }
    }

    pub fn convert_font_to_logfont(&self, font: Font) -> winapi::LOGFONTW {
        unsafe {
            let pointer: *mut winapi::LOGFONTW = ptr::null_mut();
            let is_system_font: *mut winapi::BOOL = ptr::null_mut();
            let hr = (*self.native.get()).ConvertFontToLOGFONT(font.as_ptr(),
                                                               pointer,
                                                               is_system_font);
            assert!(hr == 0);
            *pointer

        }
    }

    pub fn convert_font_face_to_logfont(&self, face: FontFace) -> winapi::LOGFONTW {
        unsafe {
            let pointer: *mut winapi::LOGFONTW = ptr::null_mut();
            let hr = (*self.native.get()).ConvertFontFaceToLOGFONT(face.as_ptr(), pointer);
            assert!(hr == 0);
            *pointer
        }
    }

    pub fn create_font_face_from_hdc(&self, hdc: winapi::HDC) -> FontFace {
        unsafe {
            let mut native: ComPtr<winapi::IDWriteFontFace> = ComPtr::new();
            let hr = (*self.native.get()).CreateFontFaceFromHdc(hdc, native.getter_addrefs());
            assert!(hr == 0);
            FontFace::take(native)
        }
    }

    pub fn create_bitmap_render_target(&self, width: u32, height: u32) -> BitmapRenderTarget {
        unsafe {
            let mut native: ComPtr<winapi::IDWriteBitmapRenderTarget> = ComPtr::new();
            let hr = (*self.native.get()).CreateBitmapRenderTarget(ptr::null_mut(),
                                                                   width, height,
                                                                   native.getter_addrefs());
            assert!(hr == 0);
            BitmapRenderTarget::take(native)
        }
    }
}
