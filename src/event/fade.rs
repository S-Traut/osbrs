// Copyright 2021 Thomas Ballasi
// This file has been written by Stéphane Traut
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::easing::Easing;
use crate::Event;

#[cfg(test)]
mod tests {
    use crate::{event::*, Easing};

    #[test]
    fn to_line_static() {
        let fade_event: Fade = (0, 1).into();
        assert_eq!(fade_event.to_line(), " F,0,0,,1");

        let mut fade_event_depth: Fade = (0, 1).into();
        fade_event_depth.set_depth(2);
        assert_eq!(fade_event_depth.to_line(), "   F,0,0,,1");
    }

    #[test]
    fn to_line_dynamic() {
        let fade_event: Fade = (0, 1000, 0, 1).into();
        assert_eq!(fade_event.to_line(), " F,0,0,1000,0,1");

        let fade_event_easing: Fade = (Easing::QuadOut, 0, 1000, 0, 1).into();
        assert_eq!(fade_event_easing.to_line(), " F,4,0,1000,0,1");
    }
}

/// `Fade` event
pub enum Fade {
    Static(usize, i32, i32),
    Dynamic(usize, Easing, i32, i32, i32, i32),
}

impl Event for Fade {
    fn to_line(&self) -> String {
        match self {
            Fade::Static(depth, time, value) => {
                format!(
                    "{} F,{},{},,{}",
                    " ".repeat(*depth),
                    Easing::Linear.id(),
                    time,
                    value
                )
            }
            Fade::Dynamic(depth, easing, start_time, end_time, start_value, end_value) => format!(
                "{} F,{},{},{},{},{}",
                " ".repeat(*depth),
                easing.id(),
                start_time,
                end_time,
                start_value,
                end_value
            ),
        }
    }

    fn set_depth(&mut self, depth: usize) {
        match self {
            Fade::Static(ref mut current_depth, ..) => *current_depth = depth,
            Fade::Dynamic(ref mut current_depth, ..) => *current_depth = depth,
        }
    }
}

/// Creates a static `Fade` event with the timestamp and the value of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::{event::Fade, Sprite};
///
/// let time = 0;
/// let value = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.fade_((time, value));
/// ```
impl Into<Fade> for (i32, i32) {
    fn into(self) -> Fade {
        Fade::Static(0, self.0, self.1)
    }
}

/// Creates a dynamic `Fade` event with the timestamps and the values of the element
///
/// Uses a `Linear` easing
///
/// Example:
/// ```
/// use osb::{event::Fade, Sprite};
///
/// let start_time = 0;
/// let end_time = 1000;
/// let start_value = 0;
/// let end_value = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.fade_((start_time, end_time, start_value, end_value));
/// ```
impl Into<Fade> for (i32, i32, i32, i32) {
    fn into(self) -> Fade {
        Fade::Dynamic(0, Easing::Linear, self.0, self.1, self.2, self.3)
    }
}

/// Creates a dynamic `Fade` event with the easing, the timestamps and the values of the element
///
/// Example:
/// ```
/// use osb::{event::Fade, Easing, Sprite};
///
/// let easing = Easing::Out;
/// let start_time = 0;
/// let end_time = 1000;
/// let start_value = 0;
/// let end_value = 1;
///
/// let mut sprite = Sprite::new("res/sprite.png");
/// sprite.fade_((easing, start_time, end_time, start_value, end_value));
/// ```
impl Into<Fade> for (Easing, i32, i32, i32, i32) {
    fn into(self) -> Fade {
        Fade::Dynamic(0, self.0, self.1, self.2, self.3, self.4)
    }
}
