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
use super::TermailActivity;
use maildir::Maildir;
use std::path::Path;
use tui_realm_treeview::{Node, Tree};
// use tuirealm::{Payload, PropPayload, PropValue, PropsBuilder, Value};

impl TermailActivity {
    pub fn scan_dir(&mut self, p: &Path) {
        self.path = p.to_path_buf();
        self.tree = Tree::new(Self::dir_tree(p, 3));
    }

    // pub fn upper_dir(&self) -> Option<&Path> {
    //     self.path.parent()
    // }

    pub fn dir_tree(p: &Path, depth: usize) -> Node {
        let mut name: String = match p.file_name() {
            None => "/".to_string(),
            Some(n) => n.to_string_lossy().into_owned(),
        };

        let mail_dir = Maildir::from(p.to_string_lossy().to_string());
        let new_items = mail_dir.list_new().count();
        if new_items > 0 {
            name.push('(');
            name.push_str(&new_items.to_string());
            name.push(')');
        }

        let mut node: Node = Node::new(p.to_string_lossy().into_owned(), name);
        if depth > 0 && p.is_dir() {
            if let Ok(paths) = std::fs::read_dir(p) {
                let paths: Vec<_> = paths.filter_map(std::result::Result::ok).collect();
                // let mut paths: Vec<_> = paths.filter_map(std::result::Result::ok).collect();

                // paths.sort_by_cached_key(|k| {
                //     get_pin_yin(&k.file_name().to_string_lossy().to_string())
                // });
                for p in paths {
                    node.add_child(Self::dir_tree(p.path().as_path(), depth - 1));
                }
            }
        }
        node
    }

    // pub fn dir_children(p: &Path) -> Vec<String> {
    //     let mut children: Vec<String> = vec![];
    //     if p.is_dir() {
    //         if let Ok(paths) = std::fs::read_dir(p) {
    //             let mut paths: Vec<_> = paths.filter_map(std::result::Result::ok).collect();

    //             // paths.sort_by_cached_key(|k| {
    //             //     get_pin_yin(&k.file_name().to_string_lossy().to_string())
    //             // });
    //             for p in paths {
    //                 if !p.path().is_dir() {
    //                     children.push(String::from(p.path().to_string_lossy()));
    //                 }
    //             }
    //         }
    //     }
    //     children
    // }
}
