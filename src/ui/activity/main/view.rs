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
// Locals
use super::{
    TermailActivity, COMPONENT_LABEL_HELP, COMPONENT_TABLE_MAILLIST, COMPONENT_TEXTAREA_MAIL,
    COMPONENT_TEXT_ERROR, COMPONENT_TEXT_HELP, COMPONENT_TEXT_MESSAGE,
    COMPONENT_TREEVIEW_MAILBOXES,
};
use crate::ui::{draw_area_in, draw_area_top_right};
// Ext
use tui_realm_stdlib::{
    Label, LabelPropsBuilder, Paragraph, ParagraphPropsBuilder, Table, TablePropsBuilder, Textarea,
    TextareaPropsBuilder,
};
use tuirealm::{
    props::{
        borders::{BorderType, Borders},
        TableBuilder, TextSpan,
    },
    tui::{
        layout::{Alignment, Constraint, Direction, Layout},
        style::Color,
        widgets::Clear,
    },
    PropPayload, PropsBuilder, View,
};
// tui
use tui_realm_treeview::{TreeView, TreeViewPropsBuilder};

#[allow(unused)]
impl TermailActivity {
    // -- view

    /// ### `init_setup`
    ///
    /// Initialize setup view
    pub(super) fn init_setup(&mut self) {
        // Init view
        self.view = View::init();
        // Let's mount the component we need
        self.view.mount(
            COMPONENT_LABEL_HELP,
            Box::new(Label::new(
                LabelPropsBuilder::default()
                    .with_foreground(Color::Cyan)
                    .with_text(format!(
                        "Press <CTRL+H> for help. Version: {}",
                        crate::VERSION,
                    ))
                    .build(),
            )),
        );
        self.view.mount(
            COMPONENT_TEXTAREA_MAIL,
            Box::new(Textarea::new(
                TextareaPropsBuilder::default()
                    // .with_foreground(Color::Cyan)
                    .with_background(Color::Black)
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::Green)
                    .with_highlighted_str(Some("\u{1f680}"))
                    .with_max_scroll_step(4)
                    .with_title("Mail", Alignment::Left)
                    .with_texts(vec![TextSpan::new("No mail available.")
                        .underlined()
                        .fg(Color::Green)])
                    .build(),
            )),
        );

        // Scrolltable
        self.view.mount(
            COMPONENT_TABLE_MAILLIST,
            Box::new(Table::new(
                TablePropsBuilder::default()
                    .with_background(Color::Black)
                    .with_highlighted_str(Some("\u{1f680}"))
                    .with_highlighted_color(Color::LightBlue)
                    .with_max_scroll_step(4)
                    .with_borders(Borders::ALL, BorderType::Thick, Color::Blue)
                    .scrollable(true)
                    .with_title("Mail List", Alignment::Left)
                    .with_header(&["Idx", "Time", "From", "Title"])
                    .with_widths(&[5, 18, 22, 55])
                    .with_table(
                        TableBuilder::default()
                            .add_col(TextSpan::from("Loading.."))
                            .add_col(TextSpan::from(""))
                            .add_col(TextSpan::from(""))
                            .add_col(TextSpan::from(""))
                            .build(),
                    )
                    .build(),
            )),
        );

        self.view.mount(
            COMPONENT_TREEVIEW_MAILBOXES,
            Box::new(TreeView::new(
                TreeViewPropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::LightYellow)
                    .with_foreground(Color::LightYellow)
                    .with_background(Color::Black)
                    .with_title("Mailboxes", Alignment::Left)
                    .with_tree_and_depth(self.tree.root(), 2)
                    .with_highlighted_str("\u{1f680}")
                    .keep_state(true)
                    .build(),
            )),
        );

        // We need to initialize the focus
        self.view.active(COMPONENT_TREEVIEW_MAILBOXES);
    }

    /// View gui
    #[allow(clippy::too_many_lines)]
    pub(super) fn view(&mut self) {
        if let Some(mut ctx) = self.context.take() {
            let _drop = ctx.context.draw(|f| {
                // Prepare chunks
                let chunks_main = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([Constraint::Min(2), Constraint::Length(1)].as_ref())
                    .split(f.size());
                let chunks_left = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints([Constraint::Ratio(2, 7), Constraint::Ratio(5, 7)].as_ref())
                    .split(chunks_main[0]);
                let chunks_right = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([Constraint::Min(2), Constraint::Length(18)].as_ref())
                    .split(chunks_left[1]);

                self.view
                    .render(COMPONENT_TREEVIEW_MAILBOXES, f, chunks_left[0]);
                self.view.render(COMPONENT_LABEL_HELP, f, chunks_main[1]);
                self.view
                    .render(COMPONENT_TABLE_MAILLIST, f, chunks_right[0]);
                self.view
                    .render(COMPONENT_TEXTAREA_MAIL, f, chunks_right[1]);

                if let Some(props) = self.view.get_props(COMPONENT_TEXT_HELP) {
                    if props.visible {
                        // make popup
                        let popup = draw_area_in(f.size(), 50, 90);
                        f.render_widget(Clear, popup);
                        self.view.render(COMPONENT_TEXT_HELP, f, popup);
                    }
                }

                if let Some(props) = self.view.get_props(COMPONENT_TEXT_ERROR) {
                    if props.visible {
                        let popup = draw_area_in(f.size(), 50, 10);
                        f.render_widget(Clear, popup);
                        // make popup
                        self.view.render(COMPONENT_TEXT_ERROR, f, popup);
                    }
                }

                if let Some(props) = self.view.get_props(COMPONENT_TEXT_MESSAGE) {
                    if props.visible {
                        let popup = draw_area_top_right(f.size(), 32, 15);
                        f.render_widget(Clear, popup);
                        // make popup
                        self.view.render(COMPONENT_TEXT_MESSAGE, f, popup);
                    }
                }
            });
            self.context = Some(ctx);
        }
    }

    // -- mount

    // ### mount_error
    //
    // Mount error box
    pub(super) fn mount_error(&mut self, text: &str) {
        // Mount
        self.view.mount(
            COMPONENT_TEXT_ERROR,
            Box::new(Paragraph::new(
                ParagraphPropsBuilder::default()
                    .with_foreground(Color::Red)
                    .bold()
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::Red)
                    .with_title("Error", Alignment::Center)
                    .with_texts(vec![TextSpan::from(text)])
                    .build(),
            )),
        );
        // Give focus to error
        self.view.active(COMPONENT_TEXT_ERROR);
    }

    /// ### `umount_error`
    ///
    /// Umount error message
    pub(super) fn umount_error(&mut self) {
        self.view.umount(COMPONENT_TEXT_ERROR);
    }
    // ### mount_message
    //
    // Mount message box
    pub(super) fn mount_message(&mut self, title: &str, text: &str) {
        // Mount
        self.view.mount(
            COMPONENT_TEXT_MESSAGE,
            Box::new(Paragraph::new(
                ParagraphPropsBuilder::default()
                    .with_foreground(Color::Green)
                    .bold()
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::Cyan)
                    .with_title(title, Alignment::Center)
                    .with_text_alignment(Alignment::Center)
                    .with_texts(vec![TextSpan::from(text)])
                    .build(),
            )),
        );
        // Give focus to error
        // self.view.active(COMPONENT_TEXT_MESSAGE);
    }

    /// ### `umount_message`
    ///
    /// Umount error message
    pub(super) fn umount_message(&mut self, _title: &str, text: &str) {
        if let Some(props) = self.view.get_props(COMPONENT_TEXT_MESSAGE) {
            if let Some(PropPayload::Vec(spans)) = props.own.get("spans") {
                if let Some(display_text) = spans.get(0) {
                    if text == display_text.unwrap_text_span().content {
                        self.view.umount(COMPONENT_TEXT_MESSAGE);
                    }
                }
            }
        }
    }

    // /// ### mount_help
    // ///
    // /// Mount help
    pub(super) fn mount_help(&mut self) {
        self.view.mount(
            COMPONENT_TEXT_HELP,
            Box::new(Table::new(
                TablePropsBuilder::default()
                    .with_borders(Borders::ALL, BorderType::Rounded, Color::Green)
                    .with_title("Help", Alignment::Center)
                    .with_header(&["Key", "Function"])
                    .with_widths(&[30, 70])
                    .with_table(
                        TableBuilder::default()
                            .add_col(TextSpan::new("<ESC> or <Q>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Exit"))
                            .add_row()
                            .add_col(TextSpan::new("<TAB>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Switch focus"))
                            .add_row()
                            .add_col(TextSpan::new("<h,j,k,l,g,G>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Move cursor(vim style)"))
                            .add_row()
                            .add_col(TextSpan::new("<f/b>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Seek forward/backward 5 seconds"))
                            .add_row()
                            .add_col(TextSpan::new("<F/B>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Seek forward/backward 1 second for lyrics"))
                            .add_row()
                            .add_col(TextSpan::new("<F/B>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Before 10 seconds,adjust offset of lyrics"))
                            .add_row()
                            .add_col(TextSpan::new("<T>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Switch lyrics if more than 1 available"))
                            .add_row()
                            .add_col(TextSpan::new("<n/N/space>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Next/Previous/Pause current song"))
                            .add_row()
                            .add_col(TextSpan::new("<+,=/-,_>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Increase/Decrease volume"))
                            .add_row()
                            .add_col(TextSpan::new("Library").bold().fg(Color::LightYellow))
                            .add_row()
                            .add_col(TextSpan::new("<l/L>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Add one/all songs to playlist"))
                            .add_row()
                            .add_col(TextSpan::new("<d>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Delete song or folder"))
                            .add_row()
                            .add_col(TextSpan::new("<s>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Download or search song from youtube"))
                            .add_row()
                            .add_col(TextSpan::new("<t>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Open tag editor for tag and lyric download"))
                            .add_row()
                            .add_col(TextSpan::new("<y/p>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Yank and Paste files"))
                            .add_row()
                            .add_col(TextSpan::new("<Enter>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Open sub directory as root"))
                            .add_row()
                            .add_col(TextSpan::new("<Backspace>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Go back to parent directory"))
                            .add_row()
                            .add_col(TextSpan::new("</>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Search in library"))
                            .add_row()
                            .add_col(TextSpan::new("Playlist").bold().fg(Color::LightYellow))
                            .add_row()
                            .add_col(TextSpan::new("<d/D>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Delete one/all songs from playlist"))
                            .add_row()
                            .add_col(TextSpan::new("<l>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Play selected"))
                            .add_row()
                            .add_col(TextSpan::new("<s>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Shuffle playlist"))
                            .add_row()
                            .add_col(TextSpan::new("<m>").bold().fg(Color::Cyan))
                            .add_col(TextSpan::from("Loop mode toggle"))
                            .build(),
                    )
                    .build(),
            )),
        );
        // Active help
        self.view.active(COMPONENT_TEXT_HELP);
    }

    /// ### `umount_help`
    ///
    /// Umount help
    pub(super) fn umount_help(&mut self) {
        self.view.umount(COMPONENT_TEXT_HELP);
    }
}
