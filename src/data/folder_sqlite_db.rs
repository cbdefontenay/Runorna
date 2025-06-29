use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
thread_local! {
    static DELETE_COUNTER: std::cell::RefCell<u32> = std::cell::RefCell::new(0);
}

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("folder.db").expect("Failed to open database");
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
            CREATE TABLE IF NOT EXISTS folder (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                date_created DATETIME NOT NULL,
                parent_id INTEGER,
                FOREIGN KEY(parent_id) REFERENCES folder(id) ON DELETE CASCADE
            );
            CREATE TABLE IF NOT EXISTS note (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                date_created DATETIME NOT NULL,
                folder_id INTEGER NOT NULL,
                FOREIGN KEY(folder_id) REFERENCES folder(id) ON DELETE CASCADE
            );
            CREATE TABLE IF NOT EXISTS theme_preference (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                theme_name TEXT NOT NULL,
                date_created DATETIME NOT NULL
            );"

        ).unwrap();
        conn
    };
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: i32,
    pub content: String,
    pub date_created: String,
    pub folder_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub date_created: String,
    pub parent_id: Option<i32>,
    pub children: Vec<Folder>,
}

#[server]
pub async fn save_folder(name: String, date_created: String, parent_id: Option<i32>) -> Result<(), ServerFnError> {
    DB.with(|f| {
        f.execute(
            "INSERT INTO folder (name, date_created, parent_id) VALUES (?1, ?2, ?3)",
            (&name, &date_created, &parent_id),
        )
    })?;
    Ok(())
}

#[server]
pub async fn update_folder_name(id: i32, new_name: String) -> Result<(), ServerFnError> {
    DB.with(|f| {
        f.execute(
            "UPDATE folder SET name = ?1 WHERE id = ?2",
            (&new_name, &id),
        )
    })?;
    Ok(())
}

#[server]
pub async fn delete_folder_recursive(id: i32) -> Result<(), ServerFnError> {
    let deleted_count = DB.with(|conn| {
        conn.execute("DELETE FROM folder WHERE id = ?1 OR parent_id = ?1", [&id])
    })?;

    DELETE_COUNTER.with(|counter| {
        let mut count = counter.borrow_mut();
        *count += deleted_count as u32;

        if *count >= 2 {
            DB.with(|conn| {
                conn.execute("VACUUM", []).unwrap();
            });
            *count = 0;
        }
    });

    Ok(())
}


#[server]
pub async fn get_folders() -> Result<Vec<Folder>, ServerFnError> {
    DB.with(|conn| {
        let mut stmt = conn.prepare("SELECT id, name, date_created, parent_id FROM folder ORDER BY name ASC")?;
        let folder_rows = stmt
            .query_map([], |row| {
                Ok(Folder {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    date_created: row.get(2)?,
                    parent_id: row.get(3)?,
                    children: Vec::new(),
                })
            })?
            .collect::<Result<Vec<Folder>, _>>()?;

        let mut folders_map: std::collections::HashMap<i32, Folder> = folder_rows
            .into_iter()
            .map(|f| (f.id, f))
            .collect();

        let mut root_folders: Vec<Folder> = Vec::new();
        let mut temp_children_map: std::collections::HashMap<i32, Vec<Folder>> = std::collections::HashMap::new();

        for (_, folder) in folders_map.drain() {
            if let Some(parent_id) = folder.parent_id {
                temp_children_map.entry(parent_id).or_insert_with(Vec::new).push(folder);
            } else {
                root_folders.push(folder);
            }
        }

        fn assign_children_recursively(folder: &mut Folder, children_map: &mut std::collections::HashMap<i32, Vec<Folder>>) {
            if let Some(mut children) = children_map.remove(&folder.id) {
                children.sort_by(|a, b| a.name.cmp(&b.name));
                for mut child in children {
                    assign_children_recursively(&mut child, children_map);
                    folder.children.push(child);
                }
            }
        }

        root_folders.sort_by(|a, b| a.name.cmp(&b.name));
        for folder in root_folders.iter_mut() {
            assign_children_recursively(folder, &mut temp_children_map);
        }

        for (_, mut orphaned_children) in temp_children_map {
            root_folders.extend(orphaned_children.into_iter());
        }

        Ok(root_folders)
    })
}

// Helper to sort children recursively
fn sort_folders_recursively(folder: &mut Folder) {
    folder.children.sort_by(|a, b| a.name.cmp(&b.name));
    for child in folder.children.iter_mut() {
        sort_folders_recursively(child);
    }
}

#[server]
pub async fn save_note(content: String, date_created: String, folder_id: i32) -> Result<(), ServerFnError> {
    DB.with(|conn| {
        conn.execute(
            "INSERT INTO note (content, date_created, folder_id) VALUES (?1, ?2, ?3)",
            (&content, &date_created, &folder_id),
        )
    })?;
    Ok(())
}

#[server]
pub async fn get_notes(folder_id: i32) -> Result<Vec<Note>, ServerFnError> {
    DB.with(|conn| {
        let mut stmt = conn.prepare(
            "SELECT id, content, date_created, folder_id FROM note WHERE folder_id = ?1 ORDER BY date_created DESC"
        )?;

        let notes = stmt
            .query_map([folder_id], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    date_created: row.get(2)?,
                    folder_id: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<Note>, _>>()?;

        Ok(notes)
    })
}

#[server]
pub async fn update_note(id: i32, content: String, date_updated: String) -> Result<(), ServerFnError> {
    DB.with(|conn| {
        conn.execute(
            "UPDATE note SET content = ?1, date_created = ?2 WHERE id = ?3",
            (&content, &date_updated, &id),
        )
    })?;
    Ok(())
}

#[server]
pub async fn get_folder_name(folder_id: i32) -> Result<String, ServerFnError> {
    DB.with(|conn| {
        conn.query_row(
            "SELECT name FROM folder WHERE id = ?1",
            [folder_id],
            |row| row.get(0)
        )
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    })
}

#[server]
pub async fn save_theme_preference(theme_name: String) -> Result<(), ServerFnError> {
    let now = chrono::Local::now().to_rfc3339();
    DB.with(|conn| {
        // First delete any existing preferences (we'll just keep the latest)
        conn.execute("DELETE FROM theme_preference", [])?;
        // Insert new preference
        conn.execute(
            "INSERT INTO theme_preference (theme_name, date_created) VALUES (?1, ?2)",
            (&theme_name, &now),
        )
    })?;
    Ok(())
}

#[server]
pub async fn load_theme_preference() -> Result<String, ServerFnError> {
    DB.with(|conn| {
        match conn.query_row(
            "SELECT theme_name FROM theme_preference ORDER BY date_created DESC LIMIT 1",
            [],
            |row| row.get(0),
        ) {
            Ok(theme) => Ok(theme),
            Err(e) => {
                if e.to_string().contains("no rows") {
                    // Return default theme if no preference exists
                    Ok("base16-eighties.dark".to_string())
                } else {
                    Err(ServerFnError::ServerError(e.to_string()))
                }
            }

        }
    })
}

#[server]
pub async fn save_language_preference(language_code: String) -> Result<(), ServerFnError> {
    let now = chrono::Local::now().to_rfc3339();
    DB.with(|conn| {
        conn.execute("DELETE FROM language_preference", [])?; // Only keep the latest
        conn.execute(
            "INSERT INTO language_preference (language_code, date_created) VALUES (?1, ?2)",
            (&language_code, &now),
        )
    })?;
    Ok(())
}