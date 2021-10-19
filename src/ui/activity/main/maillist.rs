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
use super::{TermailActivity, COMPONENT_PARAGRACH_MAIL, COMPONENT_TABLE_MAILLIST};
// use std::path::Path;
// use tui_realm_treeview::{Node, Tree};
// use tuirealm::{Payload, PropPayload, PropValue, PropsBuilder, Value};
use anyhow::{anyhow, Result};
use chrono::prelude::DateTime;
use chrono::Local;
use maildir::Maildir;
use mailparse::MailHeaderMap;
use std::time::{Duration, UNIX_EPOCH};
use tui_realm_stdlib::ParagraphPropsBuilder;
use tui_realm_stdlib::TablePropsBuilder;
use tuirealm::props::{TableBuilder, TextSpan};
use tuirealm::tui::style::Color;
use tuirealm::PropsBuilder;

impl TermailActivity {
    pub fn load_mailbox(&mut self, node_id: &str) {
        let mail_dir = Maildir::from(node_id);
        // let mut mail_new_entries: Vec<_> = mail_dir.list_new().map(|m| m.unwrap()).collect();
        // mail_new_entries.sort_by_key(|mail| mail.date().unwrap_or(0));

        let mail_new_entries = mail_dir.list_new();
        let mail_cur_entries = mail_dir.list_cur();

        let mut table: TableBuilder = TableBuilder::default();
        // Add new items
        let mut mail_index = 0;
        for (idx, record) in mail_new_entries.enumerate() {
            if record.is_err() {
                continue;
            }
            let mut record = record.unwrap();

            if idx > 0 {
                table.add_row();
            }

            // let id = record.id();
            let date = record.date().unwrap_or(0);
            // let received = record.received().unwrap_or(0);
            let header = record.headers().unwrap();
            let sender = header
                .get_first_value("From")
                .unwrap_or_else(|| "No Sender".to_string());
            let subject = header
                .get_first_value("Subject")
                .unwrap_or_else(|| "No Subject".to_string());
            // Creates a new SystemTime from the specified number of whole seconds
            #[allow(clippy::cast_sign_loss)]
            let d = UNIX_EPOCH + Duration::from_secs(date as u64);
            // Create DateTime from SystemTime
            let datetime = DateTime::<Local>::from(d);
            // Formats the combined date and time with the specified format string.
            let timestamp_str = datetime.format("%y-%m-%d %H:%M").to_string();
            mail_index += 1;
            self.mail_items.push_back(record);
            table
                .add_col(TextSpan::new(mail_index.to_string()))
                .add_col(TextSpan::new(timestamp_str).fg(Color::LightYellow))
                .add_col(TextSpan::new(sender).bold().fg(Color::Green))
                .add_col(TextSpan::new(subject).bold().fg(Color::Green));
        }

        // Add read items
        for record in mail_cur_entries {
            if record.is_err() {
                continue;
            }
            let mut record = record.unwrap();

            if mail_index > 0 {
                table.add_row();
            }

            // let id = record.id();
            let date = record.date().unwrap_or(0);
            // let received = record.received().unwrap_or(0);
            let header = record.headers().unwrap();
            let sender = header
                .get_first_value("From")
                .unwrap_or_else(|| "No Sender".to_string());
            let subject = header
                .get_first_value("Subject")
                .unwrap_or_else(|| "No Subject".to_string());

            // Creates a new SystemTime from the specified number of whole seconds
            #[allow(clippy::cast_sign_loss)]
            let d = UNIX_EPOCH + Duration::from_secs(date as u64);
            // Create DateTime from SystemTime
            let datetime = DateTime::<Local>::from(d);
            // Formats the combined date and time with the specified format string.
            let timestamp_str = datetime.format("%y-%m-%d %H:%M").to_string();
            mail_index += 1;
            self.mail_items.push_back(record);
            table
                .add_col(TextSpan::new(mail_index.to_string()))
                .add_col(TextSpan::new(timestamp_str).fg(tuirealm::tui::style::Color::LightYellow))
                .add_col(TextSpan::new(sender))
                .add_col(TextSpan::new(subject));
        }

        if mail_index == 0 {
            return;
            // table.add_col(TextSpan::from(""));
            // table.add_col(TextSpan::from(""));
            // table.add_col(TextSpan::from("empty maillist"));
            // table.add_col(TextSpan::from(""));
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
        let parsed_mail = mail_item.parsed()?;
        let body = parsed_mail.get_body()?;
        // println!("{}", body);
        // if let Ok(content) = String::from_utf8(body) {
        let props = self
            .view
            .get_props(COMPONENT_PARAGRACH_MAIL)
            .ok_or_else(|| anyhow!("error get props"))?;
        let props = ParagraphPropsBuilder::from(props)
            .with_texts(vec![TextSpan::new(body)])
            // .with_texts(vec![TextSpan::new(content)])
            .build();
        self.view.update(COMPONENT_PARAGRACH_MAIL, props);

        Ok(())
        // }
    }
}
