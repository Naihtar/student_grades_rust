use csv::StringRecord;
use std::{error::Error, fmt, fs::File, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    // CSV file path
    let filename = "alumnos.csv";

    // Read raw rows from CSV
    let data = extract_data_csv(filename)?;

    // Map rows into Student structs
    let mut students = assign_data_to_student(data)?;

    // Compute metrics for each student
    for student in &mut students {
        student.calculate_grades();
    }

    // Print each student (uses Display impl)
    for student in &students {
        println!("{}\n", student);
    }

    Ok(())
}

/// Reads CSV file and returns its records as raw data.
fn extract_data_csv<P: AsRef<Path>>(filename: P) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut data: Vec<StringRecord> = Vec::new();

    for row in reader.records() {
        let row_data: StringRecord = row?;
        data.push(row_data);
    }

    Ok(data)
}

/// Converts raw CSV data into a vector of Student structs.
fn assign_data_to_student(data: Vec<StringRecord>) -> Result<Vec<Student>, Box<dyn Error>> {
    let mut students: Vec<Student> = Vec::new();

    for student_data in data {
        let mut name: String = String::new();
        let mut grades: Vec<f32> = Vec::new();

        for (i, field) in student_data.iter().enumerate() {
            if i == 0 {
                name = field.to_string()
            } else {
                grades.push(field.parse::<f32>().unwrap_or(0.0));
            }
        }

        let student: Student = Student::new(name, grades);
        students.push(student);
    }
    Ok(students)
}

/// Data structure representing a student.
#[derive(Debug)]
struct Student {
    name: String,
    grades: Vec<f32>,
    average: f32,
    highest_grade: f32,
    lowest_grade: f32,
    result: bool,
}

impl Student {
    /// Creates a new Student with default metrics.
    fn new(student_name: String, student_grades: Vec<f32>) -> Self {
        Self {
            name: student_name,
            grades: student_grades,
            average: 0.0,
            highest_grade: 0.0,
            lowest_grade: 0.0,
            result: false,
        }
    }

    /// Calculates average, min, max and pass/fail result.
    fn calculate_grades(&mut self) {
        if self.grades.is_empty() {
            return;
        }
        self.average = self.calculate_average();
        self.highest_grade = self.calculate_highest_grade();
        self.lowest_grade = self.calculate_lowest_grade();
        self.result = self.passed();
    }

    /// Calculates average grade.
    fn calculate_average(&self) -> f32 {
        if self.grades.is_empty() {
            return 0.0;
        }
        let mut average: f32 = 0.0;
        for grade in &self.grades {
            average += grade;
        }
        average / self.grades.len() as f32
    }

    /// Finds highest grade.
    fn calculate_highest_grade(&self) -> f32 {
        if self.grades.is_empty() {
            return 0.0;
        }
        let mut max_grade: f32 = self.grades[0];
        for grade in &self.grades {
            if grade > &max_grade {
                max_grade = *grade;
            }
        }
        max_grade
    }

    /// Finds lowest grade.
    fn calculate_lowest_grade(&self) -> f32 {
        if self.grades.is_empty() {
            return 0.0;
        }
        let mut min_grade: f32 = self.grades[0];
        for grade in &self.grades {
            if grade < &min_grade {
                min_grade = *grade;
            }
        }
        min_grade
    }

    /// Determines if the student passed (average >= 5.0).
    fn passed(&self) -> bool {
        self.average >= 5.0
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Student: {}\nGrades: {:?}\nAverage: {:.2}\nHighest: {:.2}\nLowest: {:.2}\nResult: {}",
            self.name,
            self.grades,
            self.average,
            self.highest_grade,
            self.lowest_grade,
            if self.result { "Passed" } else { "Failed" }
        )
    }
}
