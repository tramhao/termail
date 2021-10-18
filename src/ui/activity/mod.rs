//! ## Activities
//!
//! `activities` is the module which provides all the different activities
//! each activity identifies a layout with its own logic in the UI

/**
 * MIT License
 *
 * termail - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// Locals

// Activities
pub mod main;

use crate::ui::context::Context;
use serde::{Deserialize, Serialize};
// -- Exit reason

pub enum ExitReason {
    Quit,
    // NeedRefreshPlaylist(String),
    /* Disconnect,
     * EnterSetup, */
}

#[derive(Clone, Deserialize, Serialize)]
pub enum Loop {
    Single,
    Playlist,
    Queue,
}

impl std::fmt::Display for Loop {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let loop_state = match self {
            Self::Single => "single loop",
            Self::Playlist => "loop",
            Self::Queue => "queue",
        };
        write!(f, "{}", loop_state)
    }
}

// -- Activity trait

pub trait Activity {
    /// ### `on_create`
    ///
    /// `on_create` is the function which must be called to initialize the activity.
    /// `on_create` must initialize all the data structures used by the activity
    /// Context is taken from activity manager and will be released only when activity is destroyed
    fn on_create(&mut self, context: Context);

    /// ### `on_draw`
    ///
    /// `on_draw` is the function which draws the graphical interface.
    /// This function must be called at each tick to refresh the interface
    fn on_draw(&mut self);

    /// ### `will_umount`
    ///
    /// `will_umount` is the method which must be able to report to the activity manager, whether
    /// the activity should be terminated or not.
    /// If not, the call will return `None`, otherwise return`Some(ExitReason)`
    fn will_umount(&self) -> Option<&ExitReason>;

    /// ### `on_destroy`
    ///
    /// `on_destroy` is the function which cleans up runtime variables and data before terminating
    /// the activity. This function must be called once before terminating the activity.
    /// This function finally releases the context
    fn on_destroy(&mut self) -> Option<Context>;
}
