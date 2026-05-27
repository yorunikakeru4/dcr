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

pub fn setup(_args: &[String]) -> i32 {
    println!("Setting up DCR registries...");
    match crate::core::registry::RegistryManager::load() {
        Ok(manager) => {
            println!("Loaded {} registries.", manager.config.registry.len());
            for (name, reg) in manager.config.registry {
                println!("- {}: {} (priority {})", name, reg.url, reg.priority);
            }
            0
        }
        Err(e) => {
            println!("Error: {}", e);
            1
        }
    }
}
