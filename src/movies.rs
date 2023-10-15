/*
  Author: Mihajlo Zekovic
  https://www.linkedin.com/in/mihajlo-zekovic/
  2023-10-06
  
  7 GUIs - [5] CRUD
  cargo run --bin gui75
  
  SQLite database examples found at:
  https://github.com/bbrumm/databasestar/tree/main/sample_databases
  
  Collected in simple database movies_table.db with:
  
  SELECT m.movie_id, m.title, m.overview, m.release_date, group_concat(g.genre_name, ', ') AS genres
  FROM movie m
  INNER JOIN movie_genres mg ON mg.movie_id = m.movie_id
  INNER JOIN genre g ON g.genre_id = mg.genre_id
  GROUP BY m.movie_id
  
*/

#![allow(unused)]

use rusqlite::{Connection, Result, Error, MappedRows, Rows, params};

pub struct MoviesDB {
	conn: Connection,
}

impl MoviesDB {
	pub fn new() -> Self {
		let conn = match std::fs::metadata("movies_table.db") {
			Ok(_) => Connection::open("movies_table.db"),
			Err(_) => Connection::open("../../movies_table.db"),
		};
		match conn {
			Ok(c) => { println!("Loaded..."); MoviesDB { conn: c }},
			Err(err) => {println!("NOT Loaded... {:?}", err); panic!("Error while opening database");}
		}
	}
	
	pub fn load(&self) -> Result<Vec<DataItem>> {
		
		let mut data_arr: Vec<DataItem> = Vec::new();
		
		let mut stmt = self.conn.prepare("SELECT * FROM movies LIMIT 10000")?;
		let mut rows = stmt.query([])?;
		
		while let Some(row) = rows.next()? {
			data_arr.push(DataItem {
				id: row.get(0).unwrap_or(0),
				title: row.get(1).unwrap_or(String::from("")),
				description: row.get(2).unwrap_or(String::from("")),
				date: row.get(3).unwrap_or(String::from("")),
				genre: row.get(4).unwrap_or(String::from("")),
			});
		}
		
		Ok(data_arr)
	}
	
	pub fn add(&self, row: &DataItem) -> Result<i64, Error> {
		let result = self.conn.execute("INSERT INTO movies (title, description, date, genre) VALUES (?1, ?2, ?3, ?4)",
			params![row.title, row.description, row.date, row.genre]);
		
		//u32::try_from(self.conn.last_insert_rowid()).unwrap()
		match result {
			Ok(_) => Ok(self.conn.last_insert_rowid()),
			Err(err) => { Err(err) },
		}
	}
	
	pub fn update(&self, row: &DataItem) -> Result<bool, Error> {
		let result = self.conn.execute("UPDATE movies SET title=?1, description=?2, date=?3, genre=?4 WHERE id=?5", 
			params![row.title, row.description, row.date, row.genre, row.id]);
		
		match result {
			Ok(_) => Ok(true),
			Err(err) => { Err(err) },
		}
	}
	
	pub fn delete(&self, row_id: i64) -> Result<bool, Error> {
		let result_deleted = self.conn.execute("DELETE FROM movies WHERE id = ?1", params![row_id]);
		
		match result_deleted {
			Ok(_) => Ok(true),
			Err(err) => { Err(err) },
		}
	}
}

#[derive(Debug)]
pub struct DataItem {
	pub id: i64,
	pub title: String,
	pub description: String,
	pub date: String,
	pub genre: String,
}
