// DCR — Cargo-like C/C++ project manager.
//
// Copyright (C) 2026 Dexoron (Bezotechestvo Vladimir) <main@dexoron.su>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use toml::Value;

#[derive(Debug, Clone)]
pub struct DepLock {
    pub name: String,
    pub version: String,
    pub checksum: String,
    pub source: String,
}

pub fn write_lock(
    project_root: &Path,
    project_name: &str,
    project_version: &str,
    packages: &[DepLock],
) -> Result<(), String> {
    let mut out = String::new();
    out.push_str("[[package]]\n");
    out.push_str(&format!("name = \"{}\"\n", escape_value(project_name)));
    out.push_str(&format!(
        "version = \"{}\"\n",
        escape_value(project_version)
    ));
    if !packages.is_empty() {
        out.push_str(&format!(
            "dependencies = [{}]\n",
            quote_list(&packages.iter().map(|p| p.name.clone()).collect::<Vec<_>>())
        ));
    }
    out.push('\n');

    for pkg in packages {
        out.push_str("[[package]]\n");
        out.push_str(&format!("name = \"{}\"\n", escape_value(&pkg.name)));
        out.push_str(&format!("version = \"{}\"\n", escape_value(&pkg.version)));
        out.push_str(&format!("source = \"{}\"\n", escape_value(&pkg.source)));
        out.push_str(&format!("checksum = \"{}\"\n", escape_value(&pkg.checksum)));
        out.push('\n');
    }
    fs::write(project_root.join("dcr.lock"), out)
        .map_err(|err| format!("Failed to write dcr.lock: {err}"))?;
    Ok(())
}

fn quote_list(items: &[String]) -> String {
    items
        .iter()
        .map(|s| format!("\"{}\"", escape_value(s)))
        .collect::<Vec<_>>()
        .join(", ")
}

fn escape_value(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

#[allow(dead_code)]
pub fn read_dep_version(dep_path: &Path) -> Option<String> {
    let path = dep_path.join("dcr.toml");
    let content = fs::read_to_string(path).ok()?;
    let value: Value = content.parse().ok()?;
    value
        .get("package")
        .and_then(|v| v.as_table())
        .and_then(|t| t.get("version"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

#[allow(dead_code)]
pub fn compute_checksum(root: &Path) -> Result<String, String> {
    let mut files = Vec::new();
    collect_files(root, root, &mut files)?;
    files.sort();
    let mut hasher = Sha256::new();
    for rel in files {
        hasher.update(rel.as_bytes());
        let data =
            fs::read(root.join(&rel)).map_err(|err| format!("failed to read {}: {err}", rel))?;
        hasher.update(&data);
    }
    let hash = hasher.finalize();
    Ok(to_hex(&hash))
}

#[allow(dead_code)]
fn collect_files(root: &Path, dir: &Path, out: &mut Vec<String>) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|err| format!("read_dir failed: {err}"))? {
        let entry = entry.map_err(|err| format!("read_dir failed: {err}"))?;
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str == "target" {
            continue;
        }
        if path.is_dir() {
            collect_files(root, &path, out)?;
        } else if path.is_file() {
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            out.push(rel);
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}
