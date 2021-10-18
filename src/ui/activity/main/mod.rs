//! ## `MainActivity`
//!
//! `main_activity` is the module which implements the Main activity, which is the activity to
//! work on termusic app
/**
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
mod mailboxes;
mod maillist;
mod update;
mod view;
use super::{Activity, Context, ExitReason};
use crate::config::{TermailConfig, MAIL_DIR};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use log::error;
use std::path::{Path, PathBuf};
use tui_realm_treeview::Tree;
use tuirealm::View;

// -- components
const COMPONENT_LABEL_HELP: &str = "LABEL_HELP";
const COMPONENT_PARAGRACH_MAIL: &str = "PARAGRAPH_MAIL";
const COMPONENT_TABLE_MAILLIST: &str = "TABLE_MAILS";
const COMPONENT_TREEVIEW_MAILBOXES: &str = "TREEVIEW_MAILBOXES";
const COMPONENT_TEXT_HELP: &str = "TEXT_HELP";
const COMPONENT_TEXT_ERROR: &str = "TEXT_ERROR";
const COMPONENT_TEXT_MESSAGE: &str = "TEXT_MESSAGE";

/// ### `ViewLayout`

/// ## `MainActivity`
///
/// Main activity states holder
pub struct TermailActivity {
    exit_reason: Option<ExitReason>,
    context: Option<Context>, // Context holder
    view: View,               // View
    redraw: bool,
    path: PathBuf,
    tree: Tree,
    config: TermailConfig,
}
impl Default for TermailActivity {
    fn default() -> Self {
        let full_path = shellexpand::tilde(MAIL_DIR);
        let p: &Path = Path::new(full_path.as_ref());
        let config = TermailConfig::default();
        Self {
            exit_reason: None,
            context: None,
            view: View::init(),
            redraw: true, // Draw at first `on_draw`
            path: p.to_path_buf(),
            tree: Tree::new(Self::dir_tree(p, 3)),
            config,
        }
    }
}

impl TermailActivity {
    pub fn init_config(&mut self, config: &TermailConfig) {
        self.config = config.clone();
        if let Some(mail_dir) = self.config.mail_dir_from_cli.clone() {
            let full_path = shellexpand::tilde(&mail_dir);
            let p: &Path = Path::new(full_path.as_ref());
            self.scan_dir(p);
        } else {
            let mail_dir = self.config.mail_dir.clone();
            let full_path = shellexpand::tilde(&mail_dir);
            let p: &Path = Path::new(full_path.as_ref());
            self.scan_dir(p);
        }
    }
}

impl Activity for TermailActivity {
    /// ### `on_create`
    ///
    /// `on_create` is the function which must be called to initialize the activity.
    /// `on_create` must initialize all the data structures used by the activity
    /// Context is taken from activity manager and will be released only when activity is destroyed
    fn on_create(&mut self, context: Context) {
        // Set context
        self.context = Some(context);
        // // Clear terminal
        if let Some(context) = self.context.as_mut() {
            context.clear_screen();
        }
        // // Put raw mode on enabled
        if let Err(err) = enable_raw_mode() {
            error!("Failed to enter raw mode: {}", err);
        }
        // // Init view
        self.init_setup();
    }

    /// ### `on_draw`
    ///
    /// `on_draw` is the function which draws the graphical interface.
    /// This function must be called at each tick to refresh the interface
    fn on_draw(&mut self) {
        // Context must be something
        if self.context.is_none() {
            return;
        }
        // Read one event
        // if let Some(context) = self.context.as_ref() {
        if let Ok(Some(event)) = crate::ui::inputhandler::InputHandler::read_event() {
            // Set redraw to true
            self.redraw = true;
            // Handle event
            let msg = self.view.on(event);
            self.update(&msg);
        }
        // }
        // Redraw if necessary
        if self.redraw {
            // View
            self.view();
            // Redraw back to false
            self.redraw = false;
        }
    }

    /// ### `will_umount`
    ///
    /// `will_umount` is the method which must be able to report to the activity manager, whether
    /// the activity should be terminated or not.
    /// If not, the call will return `None`, otherwise return`Some(ExitReason)`
    fn will_umount(&self) -> Option<&ExitReason> {
        self.exit_reason.as_ref()
    }

    /// ### `on_destroy`
    ///
    /// `on_destroy` is the function which cleans up runtime variables and data before terminating
    /// the activity. This function must be called once before terminating the activity.
    /// This function finally releases the context
    fn on_destroy(&mut self) -> Option<Context> {
        if let Err(err) = self.config.save() {
            error!("Failed to save config: {}", err);
        }
        // Disable raw mode
        if let Err(err) = disable_raw_mode() {
            error!("Failed to disable raw mode: {}", err);
        }
        self.context.as_ref()?;
        // Clear terminal and return
        if let Some(mut ctx) = self.context.take() {
            ctx.clear_screen();
            return Some(ctx);
        }
        None
    }
}
