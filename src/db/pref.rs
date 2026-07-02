use rusqlite::Connection;

pub fn get_theme(conn: &Connection, fingerprint: &str) -> Option<String> {
    conn.query_row(
        "SELECT theme FROM preferences WHERE fingerprint = ?1",
        [fingerprint],
        |row| row.get(0),
    )
    .ok()
}

pub fn set_theme(conn: &Connection, fingerprint: &str, theme: &str) {
    conn.execute(
        "INSERT INTO preferences (fingerprint, theme) VALUES (?1, ?2)
         ON CONFLICT(fingerprint) DO UPDATE SET theme = ?2",
        [fingerprint, theme],
    )
    .ok();
}
