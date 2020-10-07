// Copyright 2020 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The type returned from widget layout.

use crate::{Insets, Size};

/// The result of a [`Widget`]'s [`layout`] method.
///
/// This always includes the widget's desired [`Size`], and may optionally
/// contain other information, such as any paint insets or baseline
/// positions.
///
/// [`Widget`]: trait.Widget.html
/// [`Size`]: struct.Size.html
/// [`layout`]: trait.Widget.html#tymethod.layout
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    size: Size,
    // not currently used, but here so we don't get clippy's trivially_copy_pass_by_ref lint
    _insets: Insets,
}

impl Layout {
    /// Create a new `Layout` from some `Size`.
    pub fn new(size: impl Into<Size>) -> Self {
        Layout {
            size: size.into(),
            _insets: Insets::ZERO,
        }
    }

    /// The widget's desired [`Size`](struct.Size.html).
    #[inline]
    pub fn size(&self) -> Size {
        self.size
    }

    /// The widget's desired height; equivalent to `this.size().height`.
    #[inline]
    pub fn height(&self) -> f64 {
        self.size.height
    }

    /// The widget's desired width; equivalent to `this.size().width`.
    #[inline]
    pub fn width(&self) -> f64 {
        self.size.width
    }
}

impl From<Size> for Layout {
    fn from(src: Size) -> Layout {
        Layout::new(src)
    }
}
