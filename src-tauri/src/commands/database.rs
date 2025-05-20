use csv::{ReaderBuilder, WriterBuilder};
use crate::utils::database;
use crate::utils::models::teacher::{PartialTeacher, Teacher};

#[tauri::command]
pub fn get_all() -> Vec<Teacher> {
    let conn = database::get_connection().unwrap();
    Teacher::get_all(&conn)
}

#[tauri::command]
pub fn add_new(teacher: Teacher) -> bool {
    let conn = database::get_connection().unwrap();
    match teacher.create(&conn) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri::command]
pub fn remove_teacher(id: u64) -> bool {
    let conn = database::get_connection().unwrap();
    Teacher::get_by_id(&conn, id).unwrap().unwrap().remove(&conn)
}

#[tauri::command]
pub fn update_teacher(id: u64, teacher: PartialTeacher) -> bool {
    let conn = database::get_connection().unwrap();

    Teacher::update(&conn, id, teacher)
}

#[tauri::command]
pub fn import_csv(data: String) -> bool {
    let conn = database::get_connection().unwrap();
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(data.as_bytes());

    for result in reader.records() {
        match result {
            Ok(record) => {
                // Oczekujemy: name, grades, total_lessions, present, absent, late
                if record.len() != 6 {
                    eprintln!("Nieprawidłowa liczba kolumn: {:?}", record);
                    continue;
                }

                let teacher = Teacher {
                    id: 0,
                    name: record[0].to_string(),
                    grades: record[1].to_string(),
                    total_lessions: record[2].parse().unwrap_or(0),
                    present: record[3].parse().unwrap_or(0),
                    absent: record[4].parse().unwrap_or(0),
                    late: record[5].parse().unwrap_or(0),
                };

                if teacher.create(&conn).is_err() {
                    eprintln!("Błąd tworzenia nauczyciela");
                }
            }
            Err(err) => {
                eprintln!("Błąd przy odczycie wiersza CSV: {}", err);
                continue;
            }
        }
    }

    true
}

#[tauri::command]
pub fn export_csv() -> String {
    let conn = database::get_connection().unwrap();
    let teachers = Teacher::get_all(&conn);
    let mut wtr = WriterBuilder::new().from_writer(vec![]);

    // Nagłówki
    wtr.write_record(&[
        "name",
        "grades",
        "total_lessions",
        "present",
        "absent",
        "late",
    ]).unwrap();

    for teacher in teachers {
        wtr.write_record(&[
            &teacher.name,
            &teacher.grades, // "6,3,1" zostanie zapisane jako "\"6,3,1\""
            &teacher.total_lessions.to_string(),
            &teacher.present.to_string(),
            &teacher.absent.to_string(),
            &teacher.late.to_string(),
        ]).unwrap();
    }

    // Zwróć string
    String::from_utf8(wtr.into_inner().unwrap()).unwrap()
}
