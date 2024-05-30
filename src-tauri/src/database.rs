use tauri::AppHandle;
use rusqlite::Connection;
use rusqlite::ffi::sqlite3_auto_extension;
use sqlite_vec::sqlite3_vec_init;
use zerocopy::AsBytes;


pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
    }

    // 打开数据库
    let db = Connection::open_in_memory()?;

    let version = get_version(&db)?;

    let items: Vec<(usize, Vec<f32>)> = vec![
        (1, vec![0.1, 0.1, 0.1, 0.1]),
        (2, vec![0.2, 0.2, 0.2, 0.2]),
        (3, vec![0.3, 0.3, 0.3, 0.3]),
        (4, vec![0.4, 0.4, 0.4, 0.4]),
        (5, vec![0.5, 0.5, 0.5, 0.5]),
    ];

    // 创建向量数据表
    db.execute(
        "CREATE VIRTUAL TABLE vec_items USING vec0(embedding float[4])",
        [],
    )?;

    // 插入向量数据
    let mut stmt = db.prepare("INSERT INTO vec_items(rowid, embedding) VALUES (?, ?)")?;
    for item in items.clone() {
        stmt.execute(rusqlite::params![item.0, item.1.as_bytes()])?;
    }
    drop(stmt);

    // 更新向量数据
    let mut stmt = db.prepare("UPDATE vec_items SET embedding=? WHERE rowid=?")?;
    let item: Vec<f32> = vec![0.14, 0.14, 0.14, 0.14];
    stmt.execute(rusqlite::params![item.as_bytes(), 4])?;
    drop(stmt);

    // 删除向量数据
    let mut stmt = db.prepare("DELETE FROM vec_items WHERE rowid=?")?;
    stmt.execute(rusqlite::params![5])?;
    drop(stmt);

    let query: Vec<f32> = vec![0.3, 0.3, 0.3, 0.3];
    println!("length {}", query.as_bytes().len());
    let result: Vec<(i64, f64)> = db
        .prepare(
            r"
          SELECT rowid, distance
          FROM vec_items
          WHERE embedding MATCH ?1
            AND k = 10
          ORDER BY distance
        ",
        )?
        .query_map([query.as_bytes()], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;
    println!("result: {:?}", result);

    Ok(db)
}

pub fn get_version(db: &Connection) -> Result<String, rusqlite::Error> {
    let v: Vec<f32> = vec![0.1, 0.2, 0.3];

    let (sqlite_version, vec_version, x): (String, String, String) = db.query_row(
        "select sqlite_version(), vec_version(), vec_to_json(?)",
        &[v.as_bytes()],
        |x| Ok((x.get(0)?, x.get(1)?, x.get(2)?)),
    )?;
    println!("sqlite_version={sqlite_version}, vec_version={vec_version}");
    Ok(vec_version)
}

pub fn add_vector(db: &Connection) -> Result<String, rusqlite::Error> {
    let query: Vec<f32> = vec![0.3, 0.3, 0.3, 0.3];
    let result: Vec<(i64, f64)> = db
        .prepare(
            r"
          SELECT
            rowid,
            distance
          FROM vec_items
          WHERE embedding MATCH ?1
          ORDER BY distance
          LIMIT 3
        ",
        )?
        .query_map([query.as_bytes()], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    let mut vec_str = String::new();
    vec_str.push_str("[");
    for (rowid, distance) in result {
        println!("rowid={rowid}, distance={distance}");
        vec_str.push_str(format!("rowid={rowid}, distance={distance} ").as_str());
    }
    vec_str.push_str("]");
    Ok(vec_str)
}
