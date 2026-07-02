use rusqlite::{Connection, params};

pub fn upvote(conn: &Connection, confession_id: i64, fingerprint: &str) -> Result<i64, String> {
    let result = conn.execute(
        "INSERT OR IGNORE INTO votes (confession_id, voter_fingerprint) VALUES (?1, ?2)",
        params![confession_id, fingerprint],
    );

    match result {
        Ok(0) => Err("Already voted".to_string()),
        Ok(_) => {
            conn.execute(
                "UPDATE confessions SET votes = votes + 1 WHERE id = ?1",
                params![confession_id],
            )
            .map_err(|e| e.to_string())?;
            conn.execute(
                "INSERT OR IGNORE INTO reactions (confession_id, reactor_fingerprint, reaction)
                 VALUES (?1, ?2, '♥')",
                params![confession_id, fingerprint],
            )
            .map_err(|e| e.to_string())?;

            let votes: i64 = conn
                .query_row(
                    "SELECT votes FROM confessions WHERE id = ?1",
                    params![confession_id],
                    |row| row.get(0),
                )
                .map_err(|e| e.to_string())?;
            Ok(votes)
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn voted_confession_ids(conn: &Connection, fingerprint: &str) -> Vec<i64> {
    let mut stmt = conn
        .prepare("SELECT confession_id FROM votes WHERE voter_fingerprint = ?1")
        .unwrap();
    stmt.query_map(params![fingerprint], |row| row.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
}
