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
use log::debug;
use rusqlite::{params, Connection};
use crate::repository;
use repository::project_file_repository::ProjectFileRepository;
use crate::models;
use models::project::Project;
use crate::models::project_tag::ProjectTag;
use crate::repository::project_source_repository::ProjectSourceRepository;
use crate::repository::project_tag_repository::ProjectTagRepository;

pub struct ProjectRepository<'a> {
    connection: &'a Connection,
}

impl<'repo> ProjectRepository<'repo> {
    pub fn get_project(&self, id: i32) -> Project {
        let mut stmt = self.connection.prepare(
            "SELECT id, name, path, notes FROM projects where id = ?1",
        ).unwrap();

        let mut project = stmt.query_one([id], |row| self.row_to_project(row)).unwrap();

        project.files = ProjectFileRepository::new(self.connection).get_files_for_project(project.clone());
        project.tags = ProjectTagRepository::new(self.connection).get_tags_by_project(project.clone());
        project.sources = ProjectSourceRepository::new(self.connection).get_sources_by_project(project.clone());
        project
    }

    pub fn get_filtered_projects(&self, name: Option<String>, path: Option<String>, tags: Option<Vec<ProjectTag>>) -> Vec<Project> {
        let mut sql = "select p.* from projects p".to_string();
        //add joins if needed
        if tags.is_some() {
            sql.push_str(" JOIN projects_tags pt ON pt.project_id = p.id");
        }
        if tags.is_some() || path.is_some() || name.is_some() {
            sql.push_str(" WHERE");
        }
        if name.is_some() {
            sql.push_str(format!(" name LIKE '%{}%'", name.clone().unwrap()).as_str());
        }
        if path.is_some() {
            if name.is_some() {
                sql.push_str(" AND");
            }
            sql.push_str(format!(" path = '{}'", path.clone().unwrap()).as_str());
        }

        if tags.is_some() {
            if path.is_some() || name.is_some() {
                sql.push_str(" AND");
            }
            let my_tags = tags.unwrap();
            let my_tags_id:Vec<String>= my_tags.iter().map(|tag| tag.id.to_string()).collect();
            sql.push_str(format!(" pt.tag_id IN ({}) GROUP BY p.id HAVING COUNT(DISTINCT pt.tag_id) = {}", my_tags_id.join(",").to_string(), my_tags.len()).as_str());
        }
        sql.push_str(" ORDER BY p.name");
        debug!("{}", sql);
        let mut stmt = self.connection.prepare(sql.as_str(),).unwrap();
        let projects :Vec<Project> = stmt.query_map([], |row| self.row_to_project(row)).unwrap().into_iter().map(|r| r.unwrap()).collect();


        let my_projects = projects.iter().map(|p| {
            let mut proj = p.clone();
            proj.files = ProjectFileRepository::new(self.connection).get_files_for_project(proj.clone());
            proj.tags = ProjectTagRepository::new(self.connection).get_tags_by_project(proj.clone());
            proj.sources = ProjectSourceRepository::new(self.connection).get_sources_by_project(proj.clone());
            proj
        }).collect();
        my_projects
    }
    pub fn create(&self, project: Project) -> rusqlite::Result<Project> {
        self.connection.execute(
            "INSERT INTO projects (name, path, notes) VALUES (?1, ?2, ?3)", params![project.name, project.path, project.notes],
        )?;
        let last_id = i32::try_from(self.connection.last_insert_rowid()).unwrap();

        Ok(self.get_project(last_id))
    }

    pub fn add_tag(&self, project: Project, tag: ProjectTag) -> Project {
        let mut stmt = self.connection.prepare(
            "INSERT INTO projects_tags (project_id, tag_id) VALUES (?1, ?2)",
        ).unwrap();
        let _ =stmt.execute(params![project.id, tag.id]);
        self.get_project(project.id)
    }

    pub fn remove_tag(&self, project: Project, tag: ProjectTag) -> Project {
        let mut stmt = self.connection.prepare(
            "DELETE FROM projects_tags WHERE project_id = ?1 AND tag_id = ?2",
        ).unwrap();

        stmt.execute(params![project.id, tag.id]).unwrap();
        self.get_project(project.id)
    }
    pub fn save(&self, project: Project) -> Project {
        let mut stmt = self.connection.prepare(
            "UPDATE projects SET name = ?1, notes = ?2, path = ?3 WHERE id = ?4",
        ).unwrap();
        let _ = stmt.execute([project.name, project.notes, project.path, project.id.to_string()]);
        self.get_project(project.id)
    }
    fn row_to_project(&self, row: &rusqlite::Row) -> Result<Project, rusqlite::Error> {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            notes: row.get(3)?,
            tags: vec![],
            files: vec![],
            sources: vec![],
        })
    }
    pub fn new(connection: &Connection) -> ProjectRepository<'_> {
        ProjectRepository {
            connection
        }
    }
}