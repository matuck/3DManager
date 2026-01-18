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
use rusqlite::{Connection};
use crate::models;
use models::project::Project;
use crate::models::project_tag::ProjectTag;

pub struct ProjectTagRepository<'a> {
    connection: &'a Connection,
}

impl ProjectTagRepository<'_> {
    pub fn get_tag(&self, id: i32) -> ProjectTag {
        let mut stmt = self.connection.prepare(
            "Select id, tag FROM tags WHERE id = ?1 LIMIT 1",
        ).unwrap();
        let my_tag = stmt.query_one([id], |row| {
            Ok(ProjectTag {
                id: row.get(0)?,
                tag: row.get(1)?,
            })
        }).unwrap();
        my_tag
    }
    pub fn get_tag_by_tag(&self, tag: String) -> rusqlite::Result<ProjectTag> {
        let mut stmt = self.connection.prepare(
            "Select id, tag FROM tags WHERE tag = ?1 LIMIT 1",
        )?;
        let my_tag = stmt.query_one([tag.clone()], |row| self.row_to_tag(row));
        my_tag
    }
    pub fn get_tags(&self) -> Vec<ProjectTag> {
        let mut stmt = self.connection.prepare(
            "Select id, tag FROM tags ORDER BY tag",
        ).unwrap();
        stmt.query_map([], |row| self.row_to_tag(row)).unwrap().into_iter().map(|r| r.unwrap()).collect()
    }
    pub fn get_tags_by_project(&self, project: Project) -> Vec<ProjectTag> {
        let mut tags_stmt = self.connection.prepare(
            "SELECT t.id, t.tag FROM projects_tags pt LEFT JOIN tags t on pt.tag_id = t.id WHERE pt.project_id = ?1",
        ).unwrap();
        let tags :Vec<ProjectTag> = tags_stmt.query_map([project.id], |row| self.row_to_tag(row)).unwrap().into_iter().map(|r| r.unwrap()).collect();
        tags
    }
    pub fn create(&self, tag: String) -> ProjectTag {
        let mut add_stmt = self.connection.prepare(
            "INSERT INTO tags (tag) VALUES (?1)"
        ).unwrap();
        add_stmt.execute([tag.clone()]).unwrap();
        let last_id = i32::try_from(self.connection.last_insert_rowid()).unwrap();
        self.get_tag(last_id)
    }
    
    fn row_to_tag(&self, row: &rusqlite::Row) -> Result<ProjectTag, rusqlite::Error> {
        Ok(ProjectTag{
            id: row.get(0).unwrap(),
            tag: row.get(1).unwrap(),
        })
    }
    pub fn new(connection: &Connection) -> ProjectTagRepository<'_> {
        ProjectTagRepository {
            connection
        }
    }
}