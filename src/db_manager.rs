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

use rusqlite::{params, Connection, Result};
use rust_embed::{Embed};
#[allow(unused)]
use log::{error, warn, info, debug, trace};
use crate::models;
use models::project::Project;
use crate::models::project_tag::ProjectTag;

pub struct DbManager {
    connection: Connection,
}
#[derive(Embed)]
#[folder = "migrations/"]
struct Migrations;

impl DbManager {
    pub fn new(connection_string: String) -> DbManager {
        let conn = Connection::open(connection_string).unwrap();
        let _ = conn.execute(
            "create table if not exists _migrations (version VARCHAR(50) NOT NULL, run_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL)",
            params![]
        );
        DbManager { connection: conn }

    }
    pub fn run_migration(&self) {
        let currversion = self.connection.query_one("SELECT * FROM _migrations ORDER BY version DESC LIMIT 1", params![], |row| {
            Ok(row.get::<usize, String>(0)?)
        }).unwrap_or("0".to_string()).parse::<i64>().unwrap();
        debug!("currversion: {}", currversion);
        for file in Migrations::iter() {
            let file_parts = file.split("/").collect::<Vec<&str>>();
            if currversion < file_parts[0].parse::<i64>().unwrap() && file_parts[1] == "up.sql" {
                info!("Running migration {}", file_parts[0]);
                let currfile = Migrations::get(&file.to_string()).unwrap();
                let sql_to_run = std::str::from_utf8(&currfile.data).unwrap();
                let _ = self.connection.execute_batch(sql_to_run);
                let _ = self.connection.execute("INSERT INTO _migrations (version) VALUES (?)", &[&file_parts[0]]);
            }
        }
    }

    pub fn get_filtered_projects(&self, name: Option<String>, path: Option<String>, tags: Option<Vec<ProjectTag>>) -> Result<Vec<Project>> {
        let mut sql = "select p.* from projects p".to_string();
        //add joins if needed
        if tags.is_some() {
            sql.push_str(" JOIN project_tag pt ON pt.project_id = o.id JOIN tags t ON t.id = pt.tag_id");
        }
        if tags.is_some() || path.is_some() || name.is_some() {
            sql.push_str(" WHERE");
        }
        if name.is_some() {
            sql.push_str(format!(" name LIKE '%{}%'", name.unwrap()).as_str());
        }
        if path.is_some() {
            sql.push_str(format!(" AND path = '{}'", path.unwrap()).as_str());
        }

        /*if tags.is_some() {
            let mut taglist = "".to_string();
            println!("{}", &tags.unwrap().iter().map(|t| { format!("{}",t.).to_string() }).collect::<Vec<String>>().join(", "));
        }*/
        debug!("{}", sql);
        let mut stmt = self.connection.prepare(sql.as_str(),)?;
        let project_rows = stmt.query_map([], |row| {
            Ok(Project {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                path: row.get(2)?,
                notes: row.get(3)?,
                tags: None,
                files: None,
            })
        })?;

        let result: Vec<Project> = project_rows.into_iter().map(|r| r.unwrap()).collect();
        Ok(result)
    }

    pub fn create_project(&self, project: Project) -> Result<Project> {
        self.connection.execute(
            "INSERT INTO projects (name, path, notes) VALUES (?1, ?2, ?3)", params![project.name, project.path, project.notes.unwrap_or("".to_string())],
        )?;
        let last_id = self.connection.last_insert_rowid();

        let mut stmt = self.connection.prepare(
            "SELECT id, name, path, notes FROM projects where id = ?1",
        )?;

        let inserted_project = stmt.query_one([last_id], |row| {
            Ok(Project {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                path: row.get(2)?,
                notes: row.get(3)?,
                tags: None,
                files: None,
            })
        });
        inserted_project
    }

}