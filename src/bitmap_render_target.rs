/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::slice;
use std::ptr;
use std::cell::UnsafeCell;
use std::mem::zeroed;

use comptr::ComPtr;
use winapi;
use super::{FontFace, RenderingParams};

#[derive(Debug)]
pub struct BitmapRenderTarget {
    native: UnsafeCell<ComPtr<winapi::IDWriteBitmapRenderTarget>>,
}

impl BitmapRenderTarget {
    pub fn take(native: ComPtr<winapi::IDWriteBitmapRenderTarget>) -> BitmapRenderTarget {
        BitmapRenderTarget {
            native: UnsafeCell::new(native),
        }
    }

    pub unsafe fn as_ptr(&self) -> *mut winapi::IDWriteBitmapRenderTarget {
        (*self.native.get()).as_ptr()
    }

    // A dip is 1/96th of an inch, so this value is the number of pixels per inch divided by 96.
    pub fn set_pixels_per_dip(&self, ppd: f32) {
        unsafe {
            (*self.native.get()).SetPixelsPerDip(ppd);
        }
    }
    
    pub fn get_memory_dc(&self) -> winapi::HDC {
        unsafe {
            (*self.native.get()).GetMemoryDC()
        }
    }

    pub fn draw_glyph_run(&self,
                          baseline_origin_x: f32,
                          baseline_origin_y: f32,
                          measuring_mode: winapi::DWRITE_MEASURING_MODE,
                          font_face: &FontFace,
                          em_size: f32,
                          glyph_indices: &[u16],
                          glyph_advances: &[f32],
                          glyph_offsets: &[winapi::DWRITE_GLYPH_OFFSET],
                          rendering_params: &RenderingParams,
                          color: &(f32, f32, f32))
    {
        unsafe {
            assert!(glyph_indices.len() == glyph_advances.len());
            assert!(glyph_indices.len() == glyph_offsets.len());
            
            let r = (color.0 * 255.0) as u8;
            let g = (color.1 * 255.0) as u8;
            let b = (color.2 * 255.0) as u8;

            let mut glyph_run: winapi::DWRITE_GLYPH_RUN = zeroed();
            glyph_run.fontFace = font_face.as_ptr();
            glyph_run.fontEmSize = em_size;
            glyph_run.glyphCount = glyph_indices.len() as u32;
            glyph_run.glyphIndices = glyph_indices.as_ptr();
            glyph_run.glyphAdvances = glyph_advances.as_ptr();
            glyph_run.glyphOffsets = glyph_offsets.as_ptr();
            glyph_run.isSideways = 0;
            glyph_run.bidiLevel = 0;

            (*self.native.get()).DrawGlyphRun(baseline_origin_x,
                                              baseline_origin_y,
                                              measuring_mode,
                                              &glyph_run,
                                              rendering_params.as_ptr(),
                                              winapi::RGB(r,g,b),
                                              ptr::null_mut());
        }
    }
}
