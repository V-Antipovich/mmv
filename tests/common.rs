use std::fs::{create_dir, create_dir_all, remove_dir_all, File};
pub static ROOT_DIRECTORY_NAME: &str = "dehftcbt4yu3h53r5435ergieruh";
use std::env::current_dir;
use std::path::Path;
/// Making up catalogue with some files we will test mmv on
///
/// Creating folder with the name that very unlikely to be present in the current directory
/// All the tests are supposed to move files inside it not to ruin anything on pc
pub fn setup_env() {
    let path = current_dir().unwrap().to_string_lossy().to_string() + "/" + ROOT_DIRECTORY_NAME;
    let _ = create_dir(&path);
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
        let mut full_path = path.clone(); //ROOT_DIRECTORY_NAME.to_string().clone();
                                          // full_path.push_str(&directory1);
        full_path.push_str(&filename);
        let pathbuf_path = Path::new(&full_path);
        let path_prefix = pathbuf_path.parent().unwrap();
        let _ = create_dir_all(path_prefix);
        let _ = File::create(&full_path);
    }
}

pub fn destroy_env() {
    let _ = remove_dir_all(
        current_dir().unwrap().to_string_lossy().to_string() + "/" + ROOT_DIRECTORY_NAME,
    );
}
