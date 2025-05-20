use crate::utils::models::teacher::{Teacher, PartialTeacher};
use rusqlite::{Connection, Result as SqlResult};
#[cfg(test)]
mod tests {
    use rusqlite::fallible_iterator::FallibleIterator;
    use crate::utils::models::teacher::Teacher;
    use super::*;

    fn create_memory_database() -> SqlResult<Connection> {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
    r#"CREATE TABLE IF NOT EXISTS teachers(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL UNIQUE,
    grades VARCHAR,
    total_lessions INTEGER NOT NULL DEFAULT 0,
    present INTEGER NOT NULL DEFAULT 0,
    absent INTEGER NOT NULL DEFAULT 0,
    late INTEGER NOT NULL DEFAULT 0
)"#,
            []
        ).unwrap_or_else(|e| panic!("Failed to create table: {}", e));

        Ok(conn)
    }

    #[test]
    fn insert_teacher(){
        let conn = create_memory_database().unwrap();
        let teacher = Teacher {
            id: 0,
            name: "Józef Piłsudzki".to_string(),
            grades: "6,3,6".to_string(),
            total_lessions: 7,
            present: 2,
            absent: 4,
            late: 1
        };

        let inserted_teacher = teacher.create(&conn).unwrap();
        let fetched_teacher = Teacher::get_by_id(&conn, inserted_teacher.id).unwrap();

        let fetched_teacher = match fetched_teacher {
            Some(teacher) => teacher,
            None => return
        };

        assert_eq!(inserted_teacher.name, fetched_teacher.name);
        assert_eq!(inserted_teacher.grades, fetched_teacher.grades);
        assert_eq!(inserted_teacher.total_lessions, fetched_teacher.total_lessions);
        assert_eq!(inserted_teacher.present, fetched_teacher.present);
        assert_eq!(inserted_teacher.absent, fetched_teacher.absent);
        assert_eq!(inserted_teacher.late, fetched_teacher.late);
    }

    #[test]
    fn update_teacher(){
        let conn = create_memory_database().unwrap();
        let teacher = Teacher {
            id: 1,
            name: "Paweł Zawadzki".to_string(),
            grades: "1 ,2 ,3 , 1".to_string(),
            total_lessions: 1,
            present: 2,
            absent: 3,
            late: 9

        };

        let pteach: PartialTeacher = PartialTeacher {
            name: Option::from("Marcin Kubacki".to_string()),
            grades: Option::from("5,6,5,5,6".to_string()),
            total_lessions: 2,
            present: 1,
            absent: 1,
            late: 1
        };

        let inserted_teacher = teacher.clone().create(&conn).unwrap();
        let update_result  = teacher.update_self(&conn, pteach.clone());

        assert!(update_result);

        // Pobieramy nauczyciela po aktualizacji
        let updated_teacher = Teacher::get_by_id(&conn, inserted_teacher.id).unwrap();

        let updated_teacher = match updated_teacher {
            Some(teacher) => teacher,
            None => return
        };

        assert_eq!(updated_teacher.name, "Marcin Kubacki");
        assert_eq!(updated_teacher.grades, "5,6,5,5,6");
        assert_eq!(updated_teacher.total_lessions, 2);
        assert_eq!(updated_teacher.present, 1);
        assert_eq!(updated_teacher.absent, 1);
        assert_eq!(updated_teacher.late, 1);
    }

    #[test]
    fn find_teacher_by_id() {
        // Utworzenie bazy w pamięci
        let conn = create_memory_database().unwrap();

        // Utworzenie nowego nauczyciela
        let teacher = Teacher {
            id: 0,  // Bazowy ID, który zostanie nadane przez bazę
            name: "Jan Kowalski".to_string(),
            grades: "4,5,6".to_string(),
            total_lessions: 10,
            present: 7,
            absent: 3,
            late: 0,
        };

        // Zapisanie nauczyciela w bazie
        let inserted_teacher = teacher.create(&conn).unwrap();

        // Próba pobrania nauczyciela po ID
        let teacher_from_db = Teacher::get_by_id(&conn, inserted_teacher.id).unwrap();

        let teacher_from_db = match teacher_from_db {
            Some(teacher) => teacher,
            None => return
        };

        assert_eq!(teacher_from_db.id, inserted_teacher.id, "ID nauczyciela się nie zgadza");
        assert_eq!(teacher_from_db.name, inserted_teacher.name, "Imię nauczyciela się nie zgadza");
        assert_eq!(teacher_from_db.grades, inserted_teacher.grades, "Oceny nauczyciela się nie zgadzają");
        assert_eq!(teacher_from_db.total_lessions, inserted_teacher.total_lessions, "Liczba lekcji nauczyciela się nie zgadza");
        assert_eq!(teacher_from_db.present, inserted_teacher.present, "Liczba obecności nauczyciela się nie zgadza");
        assert_eq!(teacher_from_db.absent, inserted_teacher.absent, "Liczba nieobecności nauczyciela się nie zgadza");
        assert_eq!(teacher_from_db.late, inserted_teacher.late, "Liczba spóźnień nauczyciela się nie zgadza");
    }


    #[test]
    fn remove_teacher() {
        let conn = create_memory_database().unwrap();

        // Utworzenie nauczyciela
        let teacher = Teacher {
            id: 0,
            name: "Jan Kowalski".to_string(),
            grades: "4,5,6".to_string(),
            total_lessions: 10,
            present: 7,
            absent: 3,
            late: 0,
        };

        let inserted_teacher = teacher.create(&conn).unwrap();
        assert!(inserted_teacher.id > 0, "Nauczyciel nie został zapisany poprawnie");

        let removal_result = inserted_teacher.remove(&conn);
        assert!(removal_result, "Nie udało się usunąć nauczyciela");

        let teacher_after_removal = Teacher::get_by_id(&conn, inserted_teacher.id).unwrap();

        assert!(teacher_after_removal.is_none(), "Nauczyciel nie został usunięty z bazy");
    }


    #[test]
    fn teacher_not_found() {
        let conn = create_memory_database().unwrap();

        let result = Teacher::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }
}