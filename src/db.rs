use rusqlite::{Connection, Result as SqlResult, params};

use crate::confession::Confession;

pub fn init(path: &str) -> SqlResult<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS confessions (
            id INTEGER PRIMARY KEY,
            text TEXT NOT NULL,
            x INTEGER NOT NULL,
            y INTEGER NOT NULL,
            votes INTEGER DEFAULT 0,
            author_fingerprint TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS votes (
            confession_id INTEGER,
            voter_fingerprint TEXT,
            PRIMARY KEY (confession_id, voter_fingerprint)
        );",
    )?;
    Ok(conn)
}

pub fn get_all(conn: &Connection) -> Vec<Confession> {
    let mut stmt = conn
        .prepare(
            "SELECT id, text, x, y, votes, author_fingerprint, created_at
             FROM confessions ORDER BY id",
        )
        .unwrap();

    stmt.query_map([], |row| {
        Ok(Confession {
            id: row.get(0)?,
            text: row.get(1)?,
            x: row.get(2)?,
            y: row.get(3)?,
            votes: row.get(4)?,
            author_fingerprint: row.get(5)?,
            created_at: row.get(6)?,
        })
    })
    .unwrap()
    .filter_map(|r| r.ok())
    .collect()
}

pub fn insert(
    conn: &Connection,
    text: &str,
    x: i64,
    y: i64,
    fingerprint: &str,
) -> SqlResult<Confession> {
    conn.execute(
        "INSERT INTO confessions (text, x, y, author_fingerprint) VALUES (?1, ?2, ?3, ?4)",
        params![text, x, y, fingerprint],
    )?;
    let id = conn.last_insert_rowid();
    Ok(Confession {
        id,
        text: text.to_string(),
        x,
        y,
        votes: 0,
        author_fingerprint: fingerprint.to_string(),
        created_at: String::new(),
    })
}

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

pub fn posts_today(conn: &Connection, fingerprint: &str) -> i64 {
    conn.query_row(
        "SELECT COUNT(*) FROM confessions
         WHERE author_fingerprint = ?1
         AND created_at > datetime('now', '-1 day')",
        params![fingerprint],
        |row| row.get(0),
    )
    .unwrap_or(0)
}

pub const DAILY_POST_LIMIT: i64 = 3;
