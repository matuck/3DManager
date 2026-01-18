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
use rusqlite::{params, Connection};
use crate::models;
use models::file::ProjectFile;
use models::project::Project;

pub struct ProjectFileRepository<'a> {
    connection: &'a Connection,
}

impl ProjectFileRepository<'_> {
    pub fn get_project_file(&self, id: i32) -> ProjectFile {
        let mut files_stmt = self.connection.prepare(
            "SELECT id, path, notes, project_id, isdefault FROM project_files WHERE id = ?1 LIMIT 1",
        ).unwrap();
        let file :ProjectFile = files_stmt.query_one([id], |row| self.row_to_file(row)).unwrap();
        file
    }
    pub fn get_files_for_project(&self, project: Project) -> Vec<ProjectFile> {
        let mut files_stmt = self.connection.prepare(
            "SELECT id, path, notes, project_id, isdefault FROM project_files WHERE project_id = ?1 ORDER BY path",
        ).unwrap();
        let files :Vec<ProjectFile> = files_stmt.query_map([project.id], |row| self.row_to_file(row)).unwrap().into_iter().map(|r| r.unwrap()).collect();
        files
    }
    pub fn delete(&self, project_file: ProjectFile) {
        let mut delete_files_stmt = self.connection.prepare(
            "DELETE FROM project_files WHERE id = ?1",
        ).unwrap();
        let _ = delete_files_stmt.execute([project_file.id]);
    }
    pub fn delete_by_project_and_path(&self, project: Project, path: String) {
        let mut delete_files_stmt = self.connection.prepare(
            "DELETE FROM project_files WHERE project_id = ?1 AND path = ?2;",
        ).unwrap();
        let _ = delete_files_stmt.execute((project.id, path));
    }

    pub fn create(&self, project_file: ProjectFile) -> ProjectFile {
        let mut add_files_stmt = self.connection.prepare(
            "INSERT INTO project_files (path, notes, isdefault, project_id) VALUES (?1, ?2, ?3, ?4);",
        ).unwrap();
        let _ = add_files_stmt.execute([
            project_file.path.clone(),
            project_file.notes.clone().unwrap_or("".to_string()),
            i32::from(project_file.default).to_string(),
            project_file.project_id.to_string(),
        ]);
        let last_id = i32::try_from(self.connection.last_insert_rowid()).unwrap();

        self.get_project_file(last_id)
    }
    pub fn save(&self, project_file:ProjectFile) -> ProjectFile {
        let mut update_stmt = self.connection.prepare(
            "UPDATE project_files SET path = ?1, notes = ?2, isdefault = ?3,  project_id=?4 WHERE id = ?5;",
        ).unwrap();

        //make all other files not default for project if this file is default.
        if project_file.default {
            let set_not_default_stmt = self.connection.prepare(
                "UPDATE project_files SET isdefault = 0 WHERE project_id = ?1"
            );
            let _ = set_not_default_stmt.unwrap().execute(params![project_file.project_id]);
        }

        let isdefault = match project_file.default {
            true => 1,
            false => 0,
        }.to_string();
        let _ = update_stmt.execute([project_file.path, project_file.notes.unwrap_or("".to_string()),isdefault, project_file.project_id.to_string(), project_file.id.to_string()]);
        self.get_project_file(project_file.id)
    }
    fn row_to_file(&self, row: &rusqlite::Row) -> Result<ProjectFile, rusqlite::Error> {
        Ok(ProjectFile {
            id: row.get(0).unwrap(),
            path: row.get(1).unwrap(),
            notes: row.get(2).unwrap(),
            project_id: row.get(3).unwrap(),
            default: row.get(4).unwrap(),
        })
    }
    pub fn new(connection: &Connection) -> ProjectFileRepository<'_> {
        ProjectFileRepository {
            connection
        }
    }
}