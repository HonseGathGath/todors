use rusqlite::{Connection, Result as SqlResult};
use std::path::PathBuf;

use crate::hierarchy::{Priority, Project, Task};
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> SqlResult<Self> {
        let db_path = Self::get_db_path();
        let conn = Connection::open(db_path)?;
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        let db = Database { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn get_db_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".todo.db");
        path
    }

    fn init_schema(&self) -> SqlResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                parent_id INTEGER NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                project_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                priority INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                due_time TEXT,
                completed_at TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS app_state (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                next_task_id INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn save_projects(&self, projects: &[Project]) -> SqlResult<()> {
        let tx = self.conn.unchecked_transaction()?;
        
        tx.execute("DELETE FROM tasks", [])?;
        tx.execute("DELETE FROM projects", [])?;

        for project in projects {
            tx.execute(
                "INSERT INTO projects (id, name, parent_id) VALUES (?1, ?2, ?3)",
                [&project.id.to_string(), &project.name, &project.parent_id.to_string()],
            )?;

            for task in &project.tasks {
                let priority = task.priority() as i32;
                let created_at = task.created_at().to_rfc3339();
                let due_time = task.due_time().map(|d| d.to_string());
                let completed_at = task.completed_at().map(|d| d.to_rfc3339());

                tx.execute(
                    "INSERT INTO tasks (id, project_id, name, description, priority, created_at, due_time, completed_at) 
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    rusqlite::params![
                        task.id(),
                        task.project_id(),
                        task.name(),
                        task.description(),
                        priority,
                        created_at,
                        due_time,
                        completed_at,
                    ],
                )?;
            }
        }

        tx.commit()?;
        Ok(())
    }

    pub fn save_task(&self, task: &Task) -> SqlResult<()> {
        let priority = task.priority() as i32;
        let created_at = task.created_at().to_rfc3339();
        let due_time = task.due_time().map(|d| d.to_string());
        let completed_at = task.completed_at().map(|d| d.to_rfc3339());

        self.conn.execute(
            "INSERT INTO tasks (id, project_id, name, description, priority, created_at, due_time, completed_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                task.id(),
                task.project_id(),
                task.name(),
                task.description(),
                priority,
                created_at,
                due_time,
                completed_at,
            ],
        )?;

        Ok(())
    }

    pub fn load_projects(&self) -> SqlResult<Vec<Project>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, parent_id FROM projects ORDER BY id")?;
        let project_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, usize>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, usize>(2)?,
            ))
        })?;

        let mut projects = Vec::new();
        for project_result in project_iter {
            let (id, name, parent_id) = project_result?;
            let tasks = self.load_tasks_for_project(id)?;
            projects.push(Project {
                id,
                name,
                parent_id,
                tasks,
            });
        }

        Ok(projects)
    }

    fn load_tasks_for_project(&self, project_id: usize) -> SqlResult<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, name, description, priority, created_at, due_time, completed_at 
             FROM tasks WHERE project_id = ?1 ORDER BY id",
        )?;

        let task_iter = stmt.query_map([project_id], |row| {
            let id = row.get::<_, usize>(0)?;
            let project_id = row.get::<_, usize>(1)?;
            let name = row.get::<_, String>(2)?;
            let description = row.get::<_, String>(3)?;
            let priority_int = row.get::<_, i32>(4)?;
            let created_at_str = row.get::<_, String>(5)?;
            let due_time_str = row.get::<_, Option<String>>(6)?;
            let completed_at_str = row.get::<_, Option<String>>(7)?;

            let priority = match priority_int {
                1 => Priority::Low,
                2 => Priority::Medium,
                3 => Priority::High,
                _ => Priority::None,
            };

            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            let due_time = due_time_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

            let completed_at = completed_at_str.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .map(|dt| dt.with_timezone(&Utc))
                    .ok()
            });

            Ok(Task::new(
                id,
                project_id,
                name,
                description,
                priority,
                created_at,
                due_time,
                completed_at,
            ))
        })?;

        let mut tasks = Vec::new();
        for task_result in task_iter {
            tasks.push(task_result?);
        }

        Ok(tasks)
    }

    pub fn save_next_task_id(&self, next_task_id: usize) -> SqlResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO app_state (id, next_task_id) VALUES (1, ?1)",
            [next_task_id],
        )?;
        Ok(())
    }

    pub fn load_next_task_id(&self) -> SqlResult<usize> {
        let result: Result<usize, _> = self.conn.query_row(
            "SELECT next_task_id FROM app_state WHERE id = 1",
            [],
            |row| row.get(0),
        );

        match result {
            Ok(id) => Ok(id),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
            Err(e) => Err(e),
        }
    }
}
