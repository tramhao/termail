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
use super::{
    ExitReason, TermailActivity, COMPONENT_PARAGRACH_MAIL, COMPONENT_TABLE_MAILLIST,
    COMPONENT_TEXT_ERROR, COMPONENT_TEXT_HELP, COMPONENT_TREEVIEW_MAILBOXES,
};
use crate::ui::keymap::{
    MSG_KEY_CHAR_CAPITAL_Q, MSG_KEY_CHAR_H, MSG_KEY_CHAR_J, MSG_KEY_CHAR_K, MSG_KEY_CHAR_L,
    MSG_KEY_CTRL_H, MSG_KEY_ENTER, MSG_KEY_ESC, MSG_KEY_TAB,
};
use tuirealm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    Msg, Payload, Value,
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

            (COMPONENT_TREEVIEW_MAILBOXES, key) if (key == &MSG_KEY_TAB) => {
                self.view.active(COMPONENT_TABLE_MAILLIST);
                None
            }

            (COMPONENT_TREEVIEW_MAILBOXES, key) if (key == &MSG_KEY_CHAR_H) => {
                let event: Event = Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                });
                self.view.on(event);
                None
            }

            (COMPONENT_TREEVIEW_MAILBOXES, key) if (key == &MSG_KEY_CHAR_L) => {
                if let Some(Payload::One(Value::Str(node_id))) =
                    self.view.get_state(COMPONENT_TREEVIEW_MAILBOXES)
                {
                    if let Some(node) = self.tree.query(&node_id) {
                        if node.is_leaf() {
                            self.load_mailbox(&node_id);
                        } else {
                            let event: Event = Event::Key(KeyEvent {
                                code: KeyCode::Right,
                                modifiers: KeyModifiers::NONE,
                            });
                            self.view.on(event);
                        }
                    }
                }
                None
            }

            (COMPONENT_TREEVIEW_MAILBOXES, Msg::OnSubmit(Payload::One(Value::Str(node_id)))) => {
                self.load_mailbox(node_id);
                None
            }

            (COMPONENT_TABLE_MAILLIST, key) if (key == &MSG_KEY_TAB) => {
                self.view.active(COMPONENT_TREEVIEW_MAILBOXES);
                None
            }

            (COMPONENT_TABLE_MAILLIST, key) if (key == &MSG_KEY_CHAR_L) => {
                if let Some(Payload::One(Value::Usize(index))) =
                    self.view.get_state(COMPONENT_TABLE_MAILLIST)
                {
                    match self.load_mail(index) {
                        Ok(_) => {
                            self.view.active(COMPONENT_PARAGRACH_MAIL);
                        }
                        Err(e) => self.mount_error(&e.to_string()),
                    }
                }
                None
            }

            (COMPONENT_TEXT_ERROR, key)
                if (key == &MSG_KEY_ESC)
                    | (key == &MSG_KEY_ENTER)
                    | (key == &MSG_KEY_CHAR_CAPITAL_Q) =>
            {
                self.umount_error();
                None
            }

            (COMPONENT_PARAGRACH_MAIL, key) if (key == &MSG_KEY_TAB) => {
                self.view.active(COMPONENT_TREEVIEW_MAILBOXES);
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
