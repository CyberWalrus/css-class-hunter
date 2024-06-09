#[cfg(test)]
mod tests {
    use crate::file::extract_exports_from_file::extract_exports_from_file;

    use std::{io::Write, path::Path};
    use tempfile::NamedTempFile;

    #[test]
    fn test_extract_basic_exports() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"
            export class MyClass {{}}
            export interface MyInterface {{}}
            export const myConst = 42;
            export function myFunction() {{}}
        "#
        )
        .unwrap();

        let mut result = extract_exports_from_file(file.path()).unwrap();
        let mut expected: Vec<String> = vec![
            "MyClass".to_string(),
            "MyInterface".to_string(),
            "myConst".to_string(),
            "myFunction".to_string(),
        ];
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_with_type_definitions() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"
            type MyType = {{
                field1: string;
                field2: number;
            }}
            export class AnotherClass {{}}
        "#
        )
        .unwrap();

        let mut result = extract_exports_from_file(file.path()).unwrap();
        let mut expected: Vec<String> = vec![
            "AnotherClass".to_string(),
            "field1".to_string(),
            "field2".to_string(),
        ];
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ignore_patterns() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"
            export type Styles = {{
                auto: string;
                full: string;
            }};
            export type ClassNames = keyof Styles;

            declare const styles: Styles;

            export default styles;
        "#
        )
        .unwrap();

        let mut result = extract_exports_from_file(file.path()).unwrap();
        let mut expected: Vec<String> = vec!["auto".to_string(), "full".to_string()];

        // Сортируем векторы перед сравнением
        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_exports() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "").unwrap();

        let result = extract_exports_from_file(file.path()).unwrap();
        let expected: Vec<String> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_invalid_file_path() {
        let invalid_path = Path::new("/non/existent/path");
        let result = extract_exports_from_file(&invalid_path);
        assert!(result.is_err());
    }
}
