/*
 * MIT License
 *
 * termusic - Copyright (c) 2021 Larry Hao
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
use super::{
    ExitReason, TermusicActivity, COMPONENT_CONFIRMATION_INPUT, COMPONENT_CONFIRMATION_RADIO,
    COMPONENT_INPUT_SEARCH_LIBRARY, COMPONENT_INPUT_URL, COMPONENT_LABEL_HELP,
    COMPONENT_PARAGRAPH_LYRIC, COMPONENT_PROGRESS, COMPONENT_TABLE_PLAYLIST,
    COMPONENT_TABLE_SEARCH_LIBRARY, COMPONENT_TABLE_YOUTUBE, COMPONENT_TEXT_ERROR,
    COMPONENT_TEXT_HELP, COMPONENT_TREEVIEW_LIBRARY,
};
use crate::ui::activity::Loop;
use crate::ui::keymap::MSG_KEY_CHAR_K;
use crate::ui::keymap::{
    MSG_KEY_BACKSPACE, MSG_KEY_CHAR_B, MSG_KEY_CHAR_CAPITAL_B, MSG_KEY_CHAR_CAPITAL_D,
    MSG_KEY_CHAR_CAPITAL_F, MSG_KEY_CHAR_CAPITAL_G, MSG_KEY_CHAR_CAPITAL_L, MSG_KEY_CHAR_CAPITAL_N,
    MSG_KEY_CHAR_CAPITAL_Q, MSG_KEY_CHAR_CAPITAL_T, MSG_KEY_CHAR_D, MSG_KEY_CHAR_DASH,
    MSG_KEY_CHAR_EQUAL, MSG_KEY_CHAR_F, MSG_KEY_CHAR_G, MSG_KEY_CHAR_H, MSG_KEY_CHAR_J,
    MSG_KEY_CHAR_L, MSG_KEY_CHAR_M, MSG_KEY_CHAR_MINUS, MSG_KEY_CHAR_N, MSG_KEY_CHAR_P,
    MSG_KEY_CHAR_PLUS, MSG_KEY_CHAR_R, MSG_KEY_CHAR_S, MSG_KEY_CHAR_T, MSG_KEY_CHAR_Y,
    MSG_KEY_CTRL_H, MSG_KEY_ENTER, MSG_KEY_ESC, MSG_KEY_SHIFT_TAB, MSG_KEY_SLASH, MSG_KEY_SPACE,
    MSG_KEY_TAB,
};
use humantime::format_duration;
use if_chain::if_chain;
use std::path::{Path, PathBuf};
use std::thread::{self, sleep};
use std::time::Duration;
use tui_realm_stdlib::{LabelPropsBuilder, ParagraphPropsBuilder, ProgressBarPropsBuilder};
use tui_realm_treeview::TreeViewPropsBuilder;
use tuirealm::props::Alignment;
use tuirealm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    props::TextSpan,
    Msg, Payload, PropsBuilder, Value,
};

impl TermusicActivity {
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
