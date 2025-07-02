use dioxus::prelude::*;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("theme.db").expect("Failed to open database");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS theme (
                id INTEGER PRIMARY KEY,
                mode TEXT NOT NULL,
                is_dark_mode BOOLEAN DEFAULT FALSE
            );",
        ).unwrap();
        conn
    };
}

#[server(endpoint = "save_theme")]
pub async fn save_theme(mode: String, is_dark_mode: bool) -> Result<(), ServerFnError> {
    DB.with(|f| {
        f.execute(
            "INSERT INTO theme (mode, is_dark_mode) VALUES (?1, ?2)",
            (&mode, &is_dark_mode),
        )
    })?;
    Ok(())
}

#[server(endpoint = "update_theme")]
pub async fn update_theme(id: usize, mode: String, is_dark_mode: bool) -> Result<(), ServerFnError> {
    DB.with(|f| {
        f.execute(
            "UPDATE theme SET mode = ?1, is_dark_mode = ?2 WHERE id = ?3",
            (&mode, &is_dark_mode, &id),
        )
    })?;
    Ok(())
}

#[server(endpoint = "load_latest_theme")]
pub async fn load_latest_theme() -> Result<(String, bool), ServerFnError> {
    let row = DB.with(|f| {
        f.query_row(
            "SELECT mode, is_dark_mode FROM theme ORDER BY id DESC LIMIT 1",
            [],
            |row| {
                let mode: String = row.get(0)?;
                let is_dark_mode: bool = row.get(1)?;
                Ok((mode, is_dark_mode))
            },
        )
    })?;
    Ok(row)
}