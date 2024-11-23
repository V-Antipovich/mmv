use glob::glob;
use std::env::current_dir;
use std::fs::rename;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum MassMoveError {
    /// Custom error handling errors of mass_move
    #[error("mmv: Not able to replace existing file: '{0}'")]
    ReplaceError(String),
    #[error("mmv: Not able to move file '{0}'. Attempt to reach restricted directory or another filesystem")]
    PermissionError(String),
}

#[derive(Error, Debug, PartialEq)]
#[error("mmv: Files for pattern '{template_name}' not found")]
pub struct NoFilesError {
    /// Custom error for catching no suitable files for template
    template_name: String,
}

pub fn get_files_by_template(template: &str) -> Result<Vec<String>, NoFilesError> {
    /*
    Gets files names that suit the given template ('*' stands for any chars sequence in filename)
    */
    let all_template_files: Vec<PathBuf> = glob(template).unwrap().filter_map(Result::ok).collect();
    if all_template_files.len() == 0 {
        Err(NoFilesError {
            template_name: template.to_string(),
        })
    } else {
        Ok(all_template_files
            .iter()
            .map(|filename| filename.to_string_lossy().to_string())
            .collect())
    }
}

pub fn mass_move(
    initial_filenames: &Vec<String>,
    target_filenames: &Vec<String>,
    force_rewrite: bool,
) -> Result<(), MassMoveError> {
    for i in 0..target_filenames.len() {
        if Path::new(&target_filenames[i]).exists() && !force_rewrite {
            return Err(MassMoveError::ReplaceError(target_filenames[i].to_string()));
        }
        let _ = match rename(&initial_filenames[i], &target_filenames[i]) {
            Ok(_) => {}
            Err(_) => {
                let current_directory = current_dir().unwrap().to_string_lossy().to_string();
                let _ = match rename(
                    current_directory.clone() + "/" + &initial_filenames[i],
                    current_directory.clone() + "/" + &target_filenames[i],
                ) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(MassMoveError::PermissionError(
                            current_directory.clone() + "/" + &target_filenames[i].to_string(),
                        ));
                    }
                };
            }
        };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::files_operations::{get_files_by_template, mass_move, MassMoveError, NoFilesError};
    use std::fs::{create_dir, create_dir_all, remove_dir_all, File};
    use std::path::Path;
    pub static ROOT_DIRECTORY_NAME: &str = "dehftcbt4yu3h53r5435ergieruh";
    #[test]
    fn test_get_files_by_template1() {
        let _ = remove_dir_all(ROOT_DIRECTORY_NAME);
        local_setup_environment();
        let root = ROOT_DIRECTORY_NAME.to_string();
        let mut path1 = root.clone();
        path1.push_str("/path/to/some_*_filename.*");
        let result1: Result<Vec<String>, NoFilesError> = Ok(vec![
            (root.clone() + "/path/to/some_A_filename.txt").to_string(),
            (root.clone() + "/path/to/some_B_filename.jpg").to_string(),
            (root.clone() + "/path/to/some__filename.gif").to_string(),
            (root.clone() + "/path/to/some_jnskfjnes_filename.c").to_string(),
        ]);
        assert_eq!(get_files_by_template(&path1), result1);
        local_destroy_environment();
    }
    #[test]
    fn test_get_files_by_template2() {
        local_destroy_environment();
        local_setup_environment();
        let root = ROOT_DIRECTORY_NAME.to_string();
        let mut path2 = root.clone();
        path2.push_str("/Documents/music/*/* - *.mp3");
        let result2: Result<Vec<String>, NoFilesError> = Ok(vec![
            (root.clone() + "/Documents/music/pop/ - Maroon5.mp3").to_string(),
            (root.clone() + "/Documents/music/pop/Neizvesten  - Bez nazvania.mp3").to_string(),
            (root.clone() + "/Documents/music/rock/A - B.mp3").to_string(),
            (root.clone() + "/Documents/music/rock/B - D.mp3").to_string(),
            (root.clone() + "/Documents/music/vk/Izvesten - S nazvaniem.mp3").to_string(),
            (root.clone() + "/Documents/music/vk/Neizvesten - Bez nazvania.mp3").to_string(),
        ]);
        assert_eq!(get_files_by_template(&path2), result2);
        local_destroy_environment();
    }

    #[test]
    fn test_get_files_by_template3() {
        local_destroy_environment();
        local_setup_environment();
        let root = ROOT_DIRECTORY_NAME.to_string();
        let mut path3 = root.clone();
        path3.push_str("/Documents/music/* - *.mp3");
        let result3: Result<Vec<String>, NoFilesError> = Ok(vec![
            (root.clone() + "/Documents/music/ - Bez nazvania.mp3").to_string(),
            (root.clone() + "/Documents/music/Neizvesten - Bez nazvania.mp3").to_string(),
        ]);
        assert_eq!(get_files_by_template(&path3), result3);
        local_destroy_environment();
    }
    #[test]
    fn test_mass_move1() {
        local_destroy_environment();
        local_setup_environment();
        let root = ROOT_DIRECTORY_NAME.to_string();
        let initial_filenames = vec![
            (root.clone() + "/path/to/some_A_filename.txt").to_string(),
            (root.clone() + "/path/to/some_B_filename.jpg").to_string(),
            (root.clone() + "/path/to/some__filename.gif").to_string(),
            (root.clone() + "/path/to/some_jnskfjnes_filename.c").to_string(),
        ];
        let target_filenames = vec![
            (root.clone() + "/path/to/changed_A_filename.txt").to_string(),
            (root.clone() + "/path/to/changed_B_filename.jpg").to_string(),
            (root.clone() + "/path/to/changed__filename.gif").to_string(),
            (root.clone() + "/path/to/changed_jnskfjnes_filename.c").to_string(),
        ];

        assert_eq!(
            mass_move(&initial_filenames, &target_filenames, false),
            Ok(())
        );
        for initial_filename in initial_filenames {
            assert!(!Path::new(&initial_filename).exists())
        }
        for target_filename in target_filenames {
            assert!(Path::new(&target_filename).exists())
        }
        local_destroy_environment();
    }

    #[test]
    fn test_mass_move2() {
        local_destroy_environment();
        local_setup_environment();
        let root = ROOT_DIRECTORY_NAME.to_string();
        let initial_filenames = vec![
            (root.clone() + "/path/to/changed_A_filename.txt").to_string(),
            (root.clone() + "/path/to/changed_B_filename.jpg").to_string(),
            (root.clone() + "/path/to/changed__filename.gif").to_string(),
            (root.clone() + "/path/to/changed_jnskfjnes_filename.c").to_string(),
        ];
        for initial_filename in &initial_filenames {
            let _ = File::create(&initial_filename);
        }
        assert_eq!(
            mass_move(&initial_filenames, &initial_filenames, false),
            Err(MassMoveError::ReplaceError(
                (root.clone() + "/path/to/changed_A_filename.txt").to_string()
            ))
        );

        assert_eq!(
            mass_move(&initial_filenames, &initial_filenames, true),
            Ok(())
        );

        local_destroy_environment();
    }

    fn local_setup_environment() {
        let _ = create_dir(ROOT_DIRECTORY_NAME);
        let files_directory1: Vec<String> = vec![
            "/path/to/some_A_filename.txt".to_string(),
            "/path/to/some_B_filename.jpg".to_string(),
            "/path/to/some__filename.gif".to_string(),
            "/path/to/some_jnskfjnes_filename.c".to_string(),
            "/Documents/music/pop/ - Maroon5.mp3".to_string(),
            "/Documents/music/pop/Neizvesten  - Bez nazvania.mp3".to_string(),
            "/Documents/music/rock/A - B.mp3".to_string(),
            "/Documents/music/rock/A-B.mp3".to_string(),
            "/Documents/music/rockkk/A- B.mp3".to_string(),
            "/Documents/music/rock/B - D.mp3".to_string(),
            "/Documents/music/vk/Neizvesten/Neizvesten - Bez nazvania.mp3".to_string(),
            "/Documents/music/vk/Neizvesten - Bez nazvania.mp3".to_string(),
            "/Documents/music/vk/Izvesten - S nazvaniem.mp3".to_string(),
            "/Documents/music/Neizvesten - Bez nazvania.mp3".to_string(),
            "/Documents/music/ - Bez nazvania.mp3".to_string(),
            "/Documents/music/vk/vk/vk/vk/vk/ -  .mp3".to_string(),
        ];
        for filename in &files_directory1 {
            let mut full_path_string = ROOT_DIRECTORY_NAME.to_string().clone();
            full_path_string.push_str(&filename);
            let full_path = Path::new(&full_path_string);
            let path_prefix = full_path.parent().unwrap();
            let _ = create_dir_all(path_prefix);
            let _ = File::create(&full_path);
        }
    }

    fn local_destroy_environment() {
        let _ = remove_dir_all(ROOT_DIRECTORY_NAME);
    }
}
