// Copyright 2021 Thomas Ballasi
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::event::*;
use crate::utils::{Number, Vec2};
use crate::Layer;
use crate::Origin;

struct EventCollection {
    move_: Vec<Move>,
    fade_: Vec<Fade>,
    rotate_: Vec<Rotate>,
    scale_: Vec<Scale>,
}

impl EventCollection {
    pub fn new() -> Self {
        Self {
            move_: Vec::<Move>::new(),
            fade_: Vec::<Fade>::new(),
            rotate_: Vec::<Rotate>::new(),
            scale_: Vec::<Scale>::new(),
        }
    }

    pub fn to_str(&self) -> String {
        format!(
            "{}{}{}{}",
            self.move_
                .iter()
                .map(|event| event.to_line() + "\n")
                .collect::<Vec<String>>()
                .join(""),
            self.fade_
                .iter()
                .map(|event| event.to_line() + "\n")
                .collect::<Vec<String>>()
                .join(""),
            self.rotate_
                .iter()
                .map(|event| event.to_line() + "\n")
                .collect::<Vec<String>>()
                .join(""),
            self.scale_
                .iter()
                .map(|event| event.to_line() + "\n")
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

/// The struct corresponding to sprites
pub struct Sprite {
    events: EventCollection,
    current_depth: usize,
    path: String,
    pos: Vec2,
    layer: Layer,
    origin: Origin,
    start_time: Option<i32>,
    end_time: Option<i32>,
}

impl Sprite {
    /// Initializes a new `Sprite`
    ///
    /// See [trait implementations](#trait-implementations) to see how you can create a sprite
    pub fn new<T>(args: T) -> Self
    where
        T: Into<Sprite>,
    {
        args.into()
    }

    /// Performs the event [`Move`] to a `Sprite`
    ///
    /// ```
    /// use osb::{ Sprite, Easing, utils::Vec2 };
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    ///
    /// // There's a `Vec2` type you can use if you wish
    /// sprite.move_((Easing::Out, 0, 1000, Vec2::from(0, 0), Vec2::from(320, 240)));
    /// // But you're not forced to! Giving pairs of integers automatically translates to a `Vec2`
    /// sprite.move_((Easing::QuadInOut, 1000, 2000, 320, 240, 100, 100));
    /// // And of course you can use a static move too
    /// sprite.move_((3000, Vec2::from(320, 240)));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn move_<T>(&mut self, args: T)
    where
        T: Into<Move>,
    {
        let mut event = args.into();
        self.process_event(event.get_start_time(), event.get_end_time());
        event.set_depth(self.current_depth);
        self.events.move_.push(event);
    }

    /// Performs the event [`Fade`] to a `Sprite`
    ///
    /// ```
    /// use osb::{ Sprite, Easing, utils::Vec2 };
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.fade_((0, 1));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn fade_<T>(&mut self, args: T)
    where
        T: Into<Fade>,
    {
        let mut event = args.into();
        self.process_event(event.get_start_time(), event.get_end_time());
        event.set_depth(self.current_depth);
        self.events.fade_.push(event);
    }

    /// Performs the event [`Rotate`] to a `Sprite`
    ///
    /// ```
    /// use osb::{ Sprite, Easing, utils::Vec2 };
    /// use std::f32::consts::PI;
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.rotate_((0, PI));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn rotate_<T>(&mut self, args: T)
    where
        T: Into<Rotate>,
    {
        let mut event = args.into();
        self.process_event(event.get_start_time(), event.get_end_time());
        event.set_depth(self.current_depth);
        self.events.rotate_.push(event);
    }

    /// Performs the event [`Scale`] to a `Sprite`
    ///
    /// ```
    /// use osb::{ Sprite, Easing, utils::Vec2 };
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.scale_((0, 1));
    /// // Please refer to the trait implementations of the event to see everything you can do
    /// ```
    pub fn scale_<T>(&mut self, args: T)
    where
        T: Into<Scale>,
    {
        let mut event = args.into();
        self.process_event(event.get_start_time(), event.get_end_time());
        event.set_depth(self.current_depth);
        self.events.scale_.push(event);
    }

    fn process_event(&mut self, event_start: i32, event_end: i32) {
        match self.start_time {
            Some(sprite_start) => {
                if event_start < sprite_start {
                    self.start_time = Some(event_start)
                }
            }
            None => self.start_time = Some(event_start),
        }

        match self.end_time {
            Some(sprite_end) => {
                if sprite_end < event_end {
                    self.end_time = Some(event_end)
                }
            }
            None => self.end_time = Some(event_end),
        }
    }

    /// Returns the initial X position of a `Sprite`
    ///
    /// **Warning**: This does **not** return the X position in a certain time.
    ///
    /// Example:
    /// ```
    /// use osb::{utils::Number, Sprite};
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.move_((0, 100, 100));
    /// assert_eq!(sprite.get_x(), Number::Int(320));
    /// ```
    pub fn get_x(&self) -> Number {
        self.pos.x
    }

    /// Returns the initial Y position of a `Sprite`
    ///
    /// **Warning**: This does **not** return the Y position in a certain time.
    ///
    /// Example:
    /// ```
    /// use osb::{utils::Number, Sprite};
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// sprite.move_((0, 100, 100));
    /// assert_eq!(sprite.get_y(), Number::Int(240));
    /// ```
    pub fn get_y(&self) -> Number {
        self.pos.y
    }

    /// Returns the start time of the first event of a `Sprite`
    ///
    /// Example:
    /// ```
    /// use osb::Sprite;
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// assert_eq!(sprite.start_time(), None);
    ///
    /// sprite.move_((100, 200, 0, 0, 320, 240));
    /// assert_eq!(sprite.start_time(), Some(100));
    ///
    /// sprite.fade_((0, 100, 0, 1));
    /// assert_eq!(sprite.start_time(), Some(0));
    /// ```
    pub fn start_time(&self) -> Option<i32> {
        self.start_time
    }

    /// Returns the end time of the first event of a `Sprite`
    ///
    /// Example:
    /// ```
    /// use osb::Sprite;
    ///
    /// let mut sprite = Sprite::new("res/sprite.png");
    /// assert_eq!(sprite.end_time(), None);
    ///
    /// sprite.move_((0, 100, 0, 0, 320, 240));
    /// assert_eq!(sprite.end_time(), Some(100));
    ///
    /// sprite.fade_((100, 200, 1, 0));
    /// assert_eq!(sprite.end_time(), Some(200));
    /// ```
    pub fn end_time(&self) -> Option<i32> {
        self.end_time
    }

    /// Returns the contents of the `Sprite`
    ///
    /// **Warning**: this method is not meant to be used
    pub fn to_str(&self) -> String {
        format!(
            "Sprite,{},{},\"{}\",{},{}\n{}",
            self.layer,
            self.origin,
            self.path,
            self.pos.x,
            self.pos.y,
            self.events.to_str()
        )
    }

    /// Sets the [`Layer`] of the `Sprite`
    ///
    /// **Warning**: this method is not meant to be used
    pub fn set_layer(&mut self, layer: Layer) {
        self.layer = layer;
    }
}

/// Creates a `Sprite` with the path of the file
///
/// Example:
/// ```
/// use osb::Sprite;
/// let path = String::from("res/sprite.png");
/// let mut sprite = Sprite::new(path);
/// ```
impl Into<Sprite> for String {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self,
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
        }
    }
}

/// Creates a `Sprite` with the path of the file
///
/// Example:
/// ```
/// use osb::Sprite;
/// let path = "res/sprite.png";
/// let mut sprite = Sprite::new(path);
/// ```
impl Into<Sprite> for &str {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self),
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
        }
    }
}

/// Creates a `Sprite` with the origin and path of the file
///
/// Example:
/// ```
/// use osb::{Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = String::from("res/sprite.png");
/// let mut sprite = Sprite::new((origin, path));
/// ```
impl Into<Sprite> for (Origin, String) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.1,
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
        }
    }
}

/// Creates a `Sprite` with the origin and the path of the file
///
/// Example:
/// ```
/// use osb::{Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = "res/sprite.png";
/// let mut sprite = Sprite::new((origin, path));
/// ```
impl Into<Sprite> for (Origin, &str) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.1),
            pos: Vec2::from(320, 240),
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
        }
    }
}

/// Creates a `Sprite` with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
/// let path = String::from("res/sprite.png");
/// let pos = Vec2::from(320, 240);
/// let mut sprite = Sprite::new((path, pos));
/// ```
impl Into<Sprite> for (String, Vec2) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.0,
            pos: self.1,
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
        }
    }
}

/// Creates a `Sprite` with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
/// let path = String::from("res/sprite.png");
/// let x = 320;
/// let y = 240;
/// let mut sprite = Sprite::new((path, x, y));
/// ```
impl<T, U> Into<Sprite> for (String, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.0,
            pos: Vec2::from(self.1, self.2),
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
        }
    }
}

/// Creates a `Sprite` with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
/// let path = "res/sprite.png";
/// let pos = Vec2::from(320, 240);
/// let mut sprite = Sprite::new((path, pos));
/// ```
impl Into<Sprite> for (&str, Vec2) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.0),
            pos: self.1,
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
        }
    }
}

/// Creates a `Sprite` with the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Sprite};
/// let path = "res/sprite.png";
/// let x = 320;
/// let y = 240;
/// let mut sprite = Sprite::new((path, x, y));
/// ```
impl<T, U> Into<Sprite> for (&str, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.0),
            pos: Vec2::from(self.1, self.2),
            layer: Layer::Background,
            origin: Origin::Centre,
            start_time: None,
            end_time: None,
        }
    }
}

/// Creates a `Sprite` with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = String::from("res/sprite.png");
/// let pos = Vec2::from(320, 240);
/// let mut sprite = Sprite::new((origin, path, pos));
/// ```
impl Into<Sprite> for (Origin, String, Vec2) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.1,
            pos: self.2,
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
        }
    }
}

/// Creates a `Sprite` with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = String::from("res/sprite.png");
/// let x = 320;
/// let y = 240;
/// let mut sprite = Sprite::new((origin, path, x, y));
/// ```
impl<T, U> Into<Sprite> for (Origin, String, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: self.1,
            pos: Vec2::from(self.2, self.3),
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
        }
    }
}

/// Creates a `Sprite` with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = "res/sprite.png";
/// let pos = Vec2::from(320, 240);
/// let mut sprite = Sprite::new((origin, path, pos));
/// ```
impl Into<Sprite> for (Origin, &str, Vec2) {
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.1),
            pos: self.2,
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
        }
    }
}

/// Creates a `Sprite` with the origin, the path of the file and the original coordinates
///
/// Example:
/// ```
/// use osb::{utils::Vec2, Origin, Sprite};
/// let origin = Origin::Centre;
/// let path = "res/sprite.png";
/// let x = 320;
/// let y = 240;
/// let mut sprite = Sprite::new((origin, path, x, y));
/// ```
impl<T, U> Into<Sprite> for (Origin, &str, T, U)
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn into(self) -> Sprite {
        Sprite {
            events: EventCollection::new(),
            current_depth: 0,
            path: String::from(self.1),
            pos: Vec2::from(self.2, self.3),
            layer: Layer::Background,
            origin: self.0,
            start_time: None,
            end_time: None,
        }
    }
}
