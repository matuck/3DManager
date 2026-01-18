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
use rusqlite::Connection;
use crate::models;
use models::project::Project;
use models::project_source::ProjectSource;

pub struct ProjectSourceRepository<'a> {
    connection: &'a Connection,
}

impl ProjectSourceRepository<'_> {
    pub fn get_source(&self, project_source_id: i32) -> ProjectSource {
        let sources_stmt = self.connection.prepare(
            "SELECT id, url, project_id, name FROM project_sources WHERE id = ?1"
        );
        let sources = sources_stmt.unwrap().query_one([project_source_id], |row| self.row_to_source(row)).unwrap();
        sources
    }
    pub fn get_sources_by_project(&self, project: Project) -> Vec<ProjectSource> {
        let sources_stmt = self.connection.prepare(
            "SELECT id, url, project_id, name FROM project_sources WHERE project_id = ?1"
        );
        let sources :Vec<ProjectSource> = sources_stmt.unwrap().query_map([project.id], |row| self.row_to_source(row)).unwrap().into_iter().map(|r| r.unwrap()).collect();
        sources
    }

    pub fn create(&self, project_source: ProjectSource) -> ProjectSource {
        let mut stmt = self.connection.prepare(
            "INSERT INTO project_sources (name, url, project_id) VALUES (?1, ?2, ?3)",
        ).unwrap();
        let _ = stmt.execute([project_source.name, project_source.url, project_source.project_id.to_string()]);
        let last_id = i32::try_from(self.connection.last_insert_rowid()).unwrap();
        self.get_source(last_id)
    }

    fn row_to_source(&self, row: &rusqlite::Row) -> Result<ProjectSource, rusqlite::Error> {
        Ok(ProjectSource{
            id: row.get(0)?,
            url: row.get(1)?,
            project_id: row.get(2)?,
            name: row.get(3)?,
        })
    }

    pub fn new(connection: &Connection) -> ProjectSourceRepository<'_> {
        ProjectSourceRepository {
            connection
        }
    }
}