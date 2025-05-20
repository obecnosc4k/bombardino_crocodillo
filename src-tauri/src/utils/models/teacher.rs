use rusqlite::{params, Row, Result as SqlResult, Connection, Error};
use serde::{Deserialize, Serialize};
use std::ffi::NulError;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Teacher {
    pub id: u64,
    pub name: String,
    pub grades: String, // "6,5,3..."
    pub total_lessions: i32,
    pub present: i32,
    pub absent: i32,
    pub late: i32
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PartialTeacher {
    pub name: Option<String>,
    pub grades: Option<String>,
    pub total_lessions: i32,
    pub present: i32,
    pub absent: i32,
    pub late: i32
}

impl Teacher {
    pub fn from_row(row: &Row) -> Teacher {
        Teacher {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            grades: row.get(2).unwrap(),
            total_lessions: row.get(3).unwrap(),
            present: row.get(4).unwrap(),
            absent: row.get(5).unwrap(),
            late: row.get(6).unwrap(),
        }
    }

    pub fn get_all(conn: &Connection) -> Vec<Teacher> {
        let mut stmt = conn.prepare("SELECT id, name, grades, total_lessions, present, absent, late FROM teachers").unwrap();
        let teacher_iter = stmt
            .query_map([], |row| Ok(Teacher::from_row(row)))
            .unwrap();

        let mut teachers = vec![];
        for teacher in teacher_iter {
            if let Ok(t) = teacher {
                teachers.push(t);
            }
        }

        teachers
    }

    pub fn get_by_id(conn: &Connection, id: u64) -> SqlResult<Option<Teacher>> {
        let mut stmt = conn.prepare("SELECT id, name, grades, total_lessions, present, absent, late FROM teachers WHERE id = ?1")?;
        let teacher = stmt.query_row(params![id], |row| Ok(Teacher::from_row(row)));

        match teacher {
            Ok(t) => Ok(Some(t)),  // Zwróć nauczyciela, jeśli znaleziono
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),  // Jeśli brak rekordu, zwróć None
            Err(e) => Err(e.into()),  // Obsłuż inne błędy
        }
    }

    pub fn create(self, conn: &Connection) -> Result<Teacher, Error> {
        let result = conn.execute(
            "INSERT INTO teachers (name, grades, total_lessions, present, absent, late)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                self.name,
                self.grades,
                self.total_lessions,
                self.present,
                self.absent,
                self.late
            ],
        );

        match result {
            Ok(_) => {
                let id = conn.last_insert_rowid();
                Ok(Teacher {
                    id: id as u64,
                    ..self
                })
            }
            Err(e) => panic!("Błąd tworzenia nauczyciela: {:?}", e),
        }
    }

    pub fn remove(&self, conn: &Connection) -> bool {
        let result = conn.execute("DELETE FROM teachers WHERE id = ?1", params![self.id]);

        match result {
            Ok(affected) => affected > 0,
            Err(_) => false,
        }
    }

    pub fn update(conn: &Connection, id: u64, updated: PartialTeacher) -> bool {
        let teacher_option = Teacher::get_by_id(&conn, id);

        // Sprawdzamy, czy nauczyciel został znaleziony
        let teacher = match teacher_option {
            Ok(Some(teacher)) => teacher,
            Ok(None) => return false,
            _ => { return false }
        };
        teacher.update_self(conn, updated)
    }

    pub fn update_self(&self, conn: &Connection, updated: PartialTeacher) -> bool {
        let id = self.id;

        let teacher = self.clone();

        let updated_teacher = Teacher {
            id,
            name: updated.name.unwrap_or(teacher.name),
            grades: updated.grades.unwrap_or(teacher.grades),
            total_lessions: updated.total_lessions,
            present: updated.present,
            absent: updated.absent,
            late: updated.late,
        };

        let result = conn.execute(
            "UPDATE teachers SET name = ?1, grades = ?2, total_lessions = ?3, present = ?4, absent = ?5, late = ?6 WHERE id = ?7",
            params![
                updated_teacher.name,
                updated_teacher.grades,
                updated_teacher.total_lessions,
                updated_teacher.present,
                updated_teacher.absent,
                updated_teacher.late,
                updated_teacher.id,
            ],
        );

        match result {
            Ok(affected) => affected > 0,
            Err(_) => false,
        }
    }
}