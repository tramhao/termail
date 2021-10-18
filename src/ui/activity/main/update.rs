/*
 * MIT License
 *
 * termail - Copyright (c) 2021 Larry Hao
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
use super::{ExitReason, TermailActivity, COMPONENT_TEXT_ERROR, COMPONENT_TEXT_HELP};
use crate::ui::keymap::{
    MSG_KEY_CHAR_CAPITAL_Q, MSG_KEY_CHAR_J, MSG_KEY_CHAR_K, MSG_KEY_CTRL_H, MSG_KEY_ENTER,
    MSG_KEY_ESC,
};
use tuirealm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    Msg,
};

impl TermailActivity {
    /// ### update
    ///
    /// Update auth activity model based on msg
    /// The function exits when returns None
    pub(super) fn update(&mut self, msg: &Option<(String, Msg)>) -> Option<(String, Msg)> {
        let ref_msg: Option<(&str, &Msg)> = msg.as_ref().map(|(s, msg)| (s.as_str(), msg));
        ref_msg.and_then(|msg| match msg {
            // -- help
            (COMPONENT_TEXT_HELP, key)
                if (key == &MSG_KEY_ENTER)
                    | (key == &MSG_KEY_ESC)
                    | (key == &MSG_KEY_CHAR_CAPITAL_Q) =>
            {
                self.umount_help();
                None
            }
            // -- error
            (COMPONENT_TEXT_ERROR, key)
                if (key == &MSG_KEY_ESC)
                    | (key == &MSG_KEY_ENTER)
                    | (key == &MSG_KEY_CHAR_CAPITAL_Q) =>
            {
                self.umount_error();
                None
            }

            (_, key) => {
                self.update_on_global_key(key);
                None
            }
        })
    }

    fn update_on_global_key(&mut self, key: &Msg) {
        match key {
            key if key == &MSG_KEY_CHAR_J => {
                let event: Event = Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                });
                self.view.on(event);
            }
            key if key == &MSG_KEY_CHAR_K => {
                let event: Event = Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                });
                self.view.on(event);
            }

            key if key == &MSG_KEY_CTRL_H => self.mount_help(),

            key if (key == &MSG_KEY_ESC) | (key == &MSG_KEY_CHAR_CAPITAL_Q) => {
                self.exit_reason = Some(ExitReason::Quit);
            }

            &_ => {}
        }
    }
}
