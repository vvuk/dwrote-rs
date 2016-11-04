/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[test]
fn test_system_family_iter() {
    let system_fc = FontCollection::system();
    let count = system_fc.families_iter().count();
    assert!(count > 0);
    assert!(system_fc.families_iter().find(|f| f.name() == "Arial").is_some());
}
