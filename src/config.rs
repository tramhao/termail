use anyhow::{anyhow, Result};
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
use serde::{Deserialize, Serialize};
use std::fs::{self, read_to_string};
use std::path::PathBuf;

pub const MAIL_DIR: &str = "~/.local/share/mail";

#[derive(Clone, Deserialize, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct TermailConfig {
    pub mail_dir: String,
    #[serde(skip_serializing)]
    pub mail_dir_from_cli: Option<String>,
}
impl Default for TermailConfig {
    fn default() -> Self {
        Self {
            mail_dir: MAIL_DIR.to_string(),
            mail_dir_from_cli: None,
        }
    }
}

impl TermailConfig {
    pub fn save(&self) -> Result<()> {
        let mut path = get_app_config_path()?;
        path.push("config.toml");

        let string = toml::to_string(self)?;

        fs::write(path.to_string_lossy().as_ref(), string)?;

        Ok(())
    }

    pub fn load(&mut self) -> Result<()> {
        let mut path = get_app_config_path()?;
        path.push("config.toml");
        if !path.exists() {
            let config = Self::default();
            config.save()?;
        }

        let string = read_to_string(path.to_string_lossy().as_ref())?;
        let config: Self = toml::from_str(&string)?;
        *self = config;
        Ok(())
    }
}

pub fn get_app_config_path() -> Result<PathBuf> {
    let mut path =
        dirs_next::config_dir().ok_or_else(|| anyhow!("failed to find os config dir."))?;
    path.push("termail");

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}
