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
use models::project_tag::ProjectTag;
use models::file::ProjectFile;
use crate::models::project_source::ProjectSource;
use crate::repository;
use repository::project_repository::ProjectRepository;
use crate::repository::project_file_repository::ProjectFileRepository;
use crate::repository::project_source_repository::ProjectSourceRepository;
use crate::repository::project_tag_repository::ProjectTagRepository;

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
        let x = DbManager { connection: conn };
        ProjectRepository::new(&x.connection);
        x
    }
    pub fn run_migration(&self) {
        let current_version = self.connection.query_one("SELECT * FROM _migrations ORDER BY version DESC LIMIT 1", params![], |row| {
            Ok(row.get::<usize, String>(0)?)
        }).unwrap_or("0".to_string()).parse::<i64>().unwrap();
        debug!("current version: {}", current_version);
        for file in Migrations::iter() {
            let file_parts = file.split("/").collect::<Vec<&str>>();
            if current_version < file_parts[0].parse::<i64>().unwrap() && file_parts[1] == "up.sql" {
                info!("Running migration {}", file_parts[0]);
                let current_file = Migrations::get(&file.to_string()).unwrap();
                let sql_to_run = std::str::from_utf8(&current_file.data).unwrap();
                let _ = self.connection.execute_batch(sql_to_run);
                let _ = self.connection.execute("INSERT INTO _migrations (version) VALUES (?)", &[&file_parts[0]]);
            }
        }
    }

    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }
}