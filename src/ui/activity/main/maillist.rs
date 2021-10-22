/**
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
    MailEntryNewOrRead, TermailActivity, COMPONENT_TABLE_MAILLIST, COMPONENT_TEXTAREA_MAIL,
    COMPONENT_TREEVIEW_MAILBOXES,
};
// use std::path::Path;
// use tui_realm_treeview::{Node, Tree};
// use tuirealm::{Payload, PropPayload, PropValue, PropsBuilder, Value};
use anyhow::{anyhow, Result};
use chrono::prelude::DateTime;
use chrono::Local;
use maildir::Maildir;
use mailparse::{MailHeaderMap, ParsedMail};
use std::io::Write;
use std::time::{Duration, UNIX_EPOCH};
use tui_realm_stdlib::TablePropsBuilder;
use tui_realm_stdlib::TextareaPropsBuilder;
use tui_realm_treeview::TreeViewPropsBuilder;
use tuirealm::props::{TableBuilder, TextSpan};
use tuirealm::tui::style::Color;
use tuirealm::PropsBuilder;

impl TermailActivity {
    pub fn load_mailbox(&mut self, node_id: &str) {
        self.mail_items = Vec::new();
        let mail_dir = Maildir::from(node_id);
        let mail_new_entries = mail_dir.list_new();
        let mail_cur_entries = mail_dir.list_cur();

        // Add new items
        for record in mail_new_entries {
            if record.is_err() {
                continue;
            }
            let mut record = record.unwrap();
            self.mail_items.push(MailEntryNewOrRead {
                date: record.date().unwrap_or(0),
                item: record,
                new: true,
            });
        }

        // Add read items
        for record in mail_cur_entries {
            if record.is_err() {
                continue;
            }
            let mut record = record.unwrap();
            self.mail_items.push(MailEntryNewOrRead {
                date: record.date().unwrap_or(0),
                item: record,
                new: false,
            });
        }
        self.mail_items.sort_by(|b, a| a.date.cmp(&b.date));
        self.mail_items.sort_by(|b, a| a.new.cmp(&b.new));

        self.current_maildir = mail_dir;
        self.sync_maillist();
    }

    pub fn sync_maillist(&mut self) {
        let mut table: TableBuilder = TableBuilder::default();
        // Add new items
        for (idx, record) in self.mail_items.iter_mut().enumerate() {
            if idx > 0 {
                table.add_row();
            }

            let date = record.date;
            let header = record.item.headers().unwrap();
            let sender = header
                .get_first_value("From")
                .unwrap_or_else(|| "No Sender".to_string());
            let subject = header
                .get_first_value("Subject")
                .unwrap_or_else(|| "No Subject".to_string());
            // Creates a new SystemTime from the specified number of whole seconds
            let date_u64 = if date.is_negative() {
                0
            } else {
                date.unsigned_abs()
            };
            // let result = if a > b { a } else { b };
            let d = UNIX_EPOCH + Duration::from_secs(date_u64);
            // Create DateTime from SystemTime
            let datetime = DateTime::<Local>::from(d);
            // Formats the combined date and time with the specified format string.
            let timestamp_str = datetime.format("%y-%m-%d %H:%M").to_string();
            table
                .add_col(TextSpan::new(idx.to_string()))
                .add_col(TextSpan::new(timestamp_str).fg(Color::LightYellow));

            if record.new {
                table
                    .add_col(TextSpan::new(sender).bold().fg(Color::Green))
                    .add_col(TextSpan::new(subject).bold().fg(Color::Green));
            } else {
                table
                    .add_col(TextSpan::new(sender))
                    .add_col(TextSpan::new(subject));
            }
        }

        let table = table.build();

        if let Some(props) = self.view.get_props(COMPONENT_TABLE_MAILLIST) {
            let props = TablePropsBuilder::from(props).with_table(table).build();
            let msg = self.view.update(COMPONENT_TABLE_MAILLIST, props);
            self.update(&msg);
            self.view.active(COMPONENT_TABLE_MAILLIST);
        }
    }

    pub fn load_mail(&mut self, index: usize) -> Result<()> {
        let mail_item = self
            .mail_items
            .get_mut(index)
            .ok_or_else(|| anyhow!("error get mail_item"))?;
        let parsed_mail = mail_item.item.parsed()?;
        let content = Self::get_body_recursive(&parsed_mail)?;
        let mut vec_lines: Vec<TextSpan> = vec![];
        for line in content.split('\n') {
            let trimed = line.trim();
            if !trimed.is_empty() {
                vec_lines.push(TextSpan::from(trimed));
            }
        }

        let mut file = std::fs::File::create("data.txt").expect("create failed");
        file.write_all(&parsed_mail.get_body_raw().unwrap())
            .expect("write failed");
        // update mail text area
        let props = self
            .view
            .get_props(COMPONENT_TEXTAREA_MAIL)
            .ok_or_else(|| anyhow!("error get props"))?;
        let props = TextareaPropsBuilder::from(props)
            // .with_texts(vec![TextSpan::new(body)])
            .with_texts(vec_lines)
            .build();
        self.view.update(COMPONENT_TEXTAREA_MAIL, props);

        if mail_item.new {
            // update mail list
            self.current_maildir.move_new_to_cur(mail_item.item.id())?;
            mail_item.new = false;
            self.sync_maillist();

            // update mail box tree view
            let path = self.path.clone();
            self.scan_dir(&path);
            if let Some(props) = self.view.get_props(COMPONENT_TREEVIEW_MAILBOXES) {
                let props = TreeViewPropsBuilder::from(props)
                    .with_tree_and_depth(self.tree.root(), 2)
                    .build();
                self.view.update(COMPONENT_TREEVIEW_MAILBOXES, props);
            }
        }

        Ok(())
    }

    fn get_body_recursive(mail: &ParsedMail) -> Result<String> {
        let mut content = String::new();
        let parts_quantity = mail.subparts.len();
        if parts_quantity == 0 {
            if mail.ctype.mimetype.starts_with("text/plain") {
                content = mail.get_body()?;
            } else if mail.ctype.mimetype.starts_with("text/html") {
                let frag = scraper::Html::parse_fragment(&mail.get_body()?);
                for node in frag.tree {
                    if let scraper::node::Node::Text(text) = node {
                        content.push_str(&text.text);
                    }
                }
            }
        } else {
            for i in 0..parts_quantity - 1 {
                content.push_str(&Self::get_body_recursive(&mail.subparts[i])?);
            }
        }

        Ok(content)
    }
}
