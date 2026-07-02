use std::collections::HashMap;

use rusqlite::{Connection, OptionalExtension, Result as SqlResult, params};

use crate::model::reaction::ReactionCount;

pub fn set_reaction(
    conn: &Connection,
    confession_id: i64,
    reaction: &str,
    fingerprint: &str,
) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO reactions (confession_id, reactor_fingerprint, reaction)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(confession_id, reactor_fingerprint)
         DO UPDATE SET reaction = excluded.reaction",
        params![confession_id, fingerprint, reaction],
    )?;
    Ok(())
}

pub fn get_reaction(
    conn: &Connection,
    confession_id: i64,
    fingerprint: &str,
) -> SqlResult<Option<String>> {
    conn.query_row(
        "SELECT reaction FROM reactions
         WHERE confession_id = ?1 AND reactor_fingerprint = ?2",
        params![confession_id, fingerprint],
        |row| row.get(0),
    )
    .optional()
}

pub fn reaction_counts(conn: &Connection) -> HashMap<i64, Vec<ReactionCount>> {
    let mut stmt = conn
        .prepare(
            "SELECT confession_id, reaction, COUNT(*) as count
             FROM reactions
             GROUP BY confession_id, reaction
             ORDER BY confession_id, count DESC, reaction ASC",
        )
        .unwrap();

    let mut grouped: HashMap<i64, Vec<ReactionCount>> = HashMap::new();
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                ReactionCount {
                    token: row.get(1)?,
                    count: row.get(2)?,
                },
            ))
        })
        .unwrap();

    for row in rows.flatten() {
        grouped.entry(row.0).or_default().push(row.1);
    }

    grouped
}
