/*
 * Copyright (c) 2025-2026 Mitch Tuck
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::models;
use serde::{Serialize, Deserialize};
use models::{file::File, project_tag::ProjectTag};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: Option<i32>,
    pub name: String,
    pub path: String,
    pub notes: Option<String>,
    pub files: Option<Vec<File>>,
    pub tags: Option<Vec<ProjectTag>>,
}