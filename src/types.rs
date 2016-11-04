/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use winapi;
use winapi::dwrite;

#[derive(PartialEq, Debug)]
pub struct FontDescriptor {
    pub family_name: String,
    pub weight: winapi::DWRITE_FONT_WEIGHT,
    pub stretch: winapi::DWRITE_FONT_STRETCH,
    pub style: winapi::DWRITE_FONT_STYLE,
}
