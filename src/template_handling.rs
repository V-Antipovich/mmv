use regex::Regex;
use std::fmt;
use std::{result::Result, str, usize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub struct TemplateError {
    /// Custom error handling mistakes in code of this file
    pub asterisks: usize,
    pub hashes: usize,
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "mmv: In target template there are {} variables,
                                but in the choice template only {}",
            self.hashes, self.asterisks
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct MoveBuilder {
    /// Struct that holds pattern of initial choice filenames and allows to create new names with given target templates
    asterisk_number: usize,
    filenames: Vec<String>,
    asterisk_sequences: Vec<Vec<String>>,
}

impl MoveBuilder {
    /// Creates new MoveBuilder by template and files corresponding to it
    ///
    /// ## Example
    /// ```
    /// let move_builder = MoveBuilder::new("/some*file.txt", vec!["/someAfile.txt".to_string(), "/someBfile.txt".to_string()]);
    /// ```
    pub fn new(template: &str, files_to_move: &Vec<String>) -> Self {
        let mut asterisk_sequences: Vec<Vec<String>> = vec![];
        let mut filenames: Vec<String> = vec![];
        let splitted_template: Vec<String> = template
            .split('*')
            .map(|substring| substring.to_string())
            .collect();
        for filename_string in files_to_move {
            let mut asterisk_sequence: Vec<String> = vec![];
            let mut shift: usize = splitted_template[0].len();
            for i in 1..splitted_template.len() {
                let Some(current_part_index) = filename_string[shift..].find(&splitted_template[i])
                else {
                    break;
                };
                if splitted_template[i].is_empty() {
                    if i == splitted_template.len() - 1 {
                        asterisk_sequence.push(filename_string[shift..].to_string());
                    } else {
                        asterisk_sequence.push("".to_string());
                    }
                } else {
                    let current_asterisk: String =
                        filename_string[shift..shift + current_part_index].to_string();
                    shift += current_part_index + splitted_template[i].len();
                    asterisk_sequence.push(current_asterisk);
                }
            }
            filenames.push(filename_string.clone());
            asterisk_sequences.push(asterisk_sequence);
        }
        let asterisk_number = match asterisk_sequences.len() {
            0 => 0,
            _ => asterisk_sequences[0].len(),
        };
        MoveBuilder {
            asterisk_number,
            filenames,
            asterisk_sequences,
        }
    }

    /// Method for building target names by given template
    ///
    /// Puts the substrings that were decoded as those under the '*' in choice template
    /// in the places that ParsedTarget.template_index_sequence tells
    /// Get the tuple of old names and new names string vectors
    ///
    /// ## Example
    /// ```
    /// // Continue example for 'new'
    /// let parsed_target = ParsedTarget{
    ///     stable_filename_parts: vec!["changed".to_string(), "file.jpg".to_string()],
    ///     template_index_sequence: vec![1]
    /// };
    /// let target_names = move_builder.build_target_names(&parsed_target);
    /// // Second vector of unwrapped target_names is ["changedAfile.jpg", "changedBfile.jpg"]
    /// ```
    pub fn build_target_names(
        &self,
        parsed_target_template: &ParsedTarget,
    ) -> Result<(Vec<String>, Vec<String>), TemplateError> {
        let max_target_template_some = parsed_target_template.template_index_sequence.iter().max();
        let max_target_template_number: usize = match max_target_template_some.is_some() {
            true => max_target_template_some.unwrap().clone(),
            false => 0,
        };
        if self.asterisk_number < max_target_template_number {
            Err(TemplateError {
                asterisks: self.asterisk_number,
                hashes: max_target_template_number,
            })
        } else {
            let mut final_target_filenames: Vec<String> = vec![];
            for i in 0..self.filenames.len() {
                let mut splitted_target_filename: Vec<String> = vec![];
                splitted_target_filename.resize_with(
                    2 * parsed_target_template.stable_filename_parts.len() - 1,
                    || "".to_string(),
                );
                for j in 0..parsed_target_template.stable_filename_parts.len() {
                    splitted_target_filename[j * 2] =
                        parsed_target_template.stable_filename_parts[j].clone();
                }
                for j in 0..parsed_target_template.template_index_sequence.len() {
                    splitted_target_filename[j * 2 + 1] = self.asterisk_sequences[i]
                        [parsed_target_template.template_index_sequence[j] - 1]
                        .clone();
                }
                final_target_filenames.push(splitted_target_filename.join(""));
            }

            Ok((self.filenames.clone(), final_target_filenames))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParsedTarget {
    pub stable_filename_parts: Vec<String>,
    pub template_index_sequence: Vec<usize>,
}

impl ParsedTarget {
    /// Takes string and finds all "#n", n is digit 1-9. Splits the string by them - these are stable parts.
    ///
    /// # Example
    /// ```
    /// parsed_target = ParsedTarget::new("changed#1file.txt")
    /// // stable_filename_parts == ["changed", "file.txt"]
    /// // template_index_sequence == [1]
    /// ```
    pub fn new(target_template: &str) -> Self {
        let hash_regex = Regex::new("#[1-9]").unwrap();
        let template_index_sequence: Vec<usize> = hash_regex
            .find_iter(target_template)
            .filter_map(|pattern| pattern.as_str().parse().ok())
            .map(|substring: String| {
                (substring.as_bytes()[1] as char).to_digit(10).unwrap() as usize
            })
            .collect();
        let stable_filename_parts: Vec<String> = hash_regex
            .replace_all(target_template, "\n")
            .split('\n')
            .map(|substring| substring.to_string())
            .collect();
        ParsedTarget {
            stable_filename_parts,
            template_index_sequence,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::template_handling::{MoveBuilder, ParsedTarget, TemplateError};
    #[test]
    fn test_parsing_template1() {
        let template = "/Desktop/path/to/changed_*_filename.*";
        let filenames = vec![
            "/Desktop/path/to/changed_A_filename.txt".to_string(),
            "/Desktop/path/to/changed_B_filename.jpg".to_string(),
            "/Desktop/path/to/changed__filename.gif".to_string(),
            "/Desktop/path/to/changed_jnskfjnes_filename.c".to_string(),
        ];
        assert_eq!(
            MoveBuilder::new(template, &filenames),
            MoveBuilder {
                asterisk_number: 2,
                filenames: filenames,
                asterisk_sequences: vec![
                    vec!["A".to_string(), "txt".to_string()],
                    vec!["B".to_string(), "jpg".to_string()],
                    vec!["".to_string(), "gif".to_string()],
                    vec!["jnskfjnes".to_string(), "c".to_string()]
                ]
            }
        );
        assert_eq!(
            MoveBuilder::new(template, &vec![]),
            MoveBuilder {
                asterisk_number: 0,
                filenames: vec![],
                asterisk_sequences: vec![]
            }
        );
    }

    #[test]
    fn test_parsing_template2() {
        let template = "/Documents/music/*/* - *.mp3";
        let filenames = vec![
            "/Documents/music/pop/ - Maroon5.mp3".to_string(),
            "/Documents/music/pop/Neizvesten  - Bez nazvania.mp3".to_string(),
            "/Documents/music/rock/A - B.mp3".to_string(),
            "/Documents/music/rock/B - D.mp3".to_string(),
            "/Documents/music/vk/Neizvesten - Bez nazvania.mp3".to_string(),
            "/Documents/music/vk/Izvesten - S nazvaniem.mp3".to_string(),
        ];
        assert_eq!(
            MoveBuilder::new(template, &filenames),
            MoveBuilder {
                asterisk_number: 3,
                filenames: filenames,
                asterisk_sequences: vec![
                    vec!["pop".to_string(), "".to_string(), "Maroon5".to_string()],
                    vec![
                        "pop".to_string(),
                        "Neizvesten ".to_string(),
                        "Bez nazvania".to_string()
                    ],
                    vec!["rock".to_string(), "A".to_string(), "B".to_string()],
                    vec!["rock".to_string(), "B".to_string(), "D".to_string()],
                    vec![
                        "vk".to_string(),
                        "Neizvesten".to_string(),
                        "Bez nazvania".to_string()
                    ],
                    vec![
                        "vk".to_string(),
                        "Izvesten".to_string(),
                        "S nazvaniem".to_string()
                    ]
                ]
            }
        );
    }

    #[test]
    fn test_parsing_template3() {
        let template = "/Documents/music/* - *.mp3";
        let filenames = vec![
            "/Documents/music/pop/ - Maroon5.mp3".to_string(),
            "/Documents/music/pop/Neizvesten  - Bez nazvania.mp3".to_string(),
            "/Documents/music/rock/A - B.mp3".to_string(),
            "/Documents/music/rock/B - D.mp3".to_string(),
            "/Documents/music/vk/Neizvesten - Bez nazvania.mp3".to_string(),
            "/Documents/music/vk/Izvesten - S nazvaniem.mp3".to_string(),
            "/Documents/music/vk/to/path/Neizvesten - Bez nazvania.mp3".to_string(),
            "/Documents/music/Neizvesten - Bez nazvania.mp3".to_string(),
            "/Documents/music/ - Bez nazvania.mp3".to_string(),
            "/Documents/music/vk/vk/vk/vk/vk/ -  .mp3".to_string(),
        ];
        assert_eq!(
            MoveBuilder::new(template, &filenames),
            MoveBuilder {
                asterisk_number: 2,
                filenames: filenames,
                asterisk_sequences: vec![
                    vec!["pop/".to_string(), "Maroon5".to_string()],
                    vec!["pop/Neizvesten ".to_string(), "Bez nazvania".to_string()],
                    vec!["rock/A".to_string(), "B".to_string()],
                    vec!["rock/B".to_string(), "D".to_string()],
                    vec!["vk/Neizvesten".to_string(), "Bez nazvania".to_string()],
                    vec!["vk/Izvesten".to_string(), "S nazvaniem".to_string()],
                    vec![
                        "vk/to/path/Neizvesten".to_string(),
                        "Bez nazvania".to_string()
                    ],
                    vec!["Neizvesten".to_string(), "Bez nazvania".to_string()],
                    vec!["".to_string(), "Bez nazvania".to_string()],
                    vec!["vk/vk/vk/vk/vk/".to_string(), " ".to_string()]
                ]
            }
        );
    }

    #[test]
    fn test_target_template() {
        let template1 = "/home/Desktop/path/to/changed_#1_filename.#2";
        assert_eq!(
            ParsedTarget::new(template1),
            ParsedTarget {
                stable_filename_parts: vec![
                    "/home/Desktop/path/to/changed_".to_string(),
                    "_filename.".to_string(),
                    "".to_string()
                ],
                template_index_sequence: vec![1, 2]
            }
        );
        let template2 = "/home/Desktop/path/to/changed_#1_fil_#2_e_#1_#2_nam_#1_e.#2";
        assert_eq!(
            ParsedTarget::new(template2),
            ParsedTarget {
                stable_filename_parts: vec![
                    "/home/Desktop/path/to/changed_".to_string(),
                    "_fil_".to_string(),
                    "_e_".to_string(),
                    "_".to_string(),
                    "_nam_".to_string(),
                    "_e.".to_string(),
                    "".to_string()
                ],
                template_index_sequence: vec![1, 2, 1, 2, 1, 2]
            }
        );
        let template3 = "/home/Desktop/path/to/changed_filename.txt";
        assert_eq!(
            ParsedTarget::new(template3),
            ParsedTarget {
                stable_filename_parts: vec![
                    "/home/Desktop/path/to/changed_filename.txt".to_string()
                ],
                template_index_sequence: vec![]
            }
        );
    }

    #[test]
    fn test_building_target_files1() {
        let template_from = "/Desktop/path/to/some_*_filename.*";
        let template_to1 = "/home/Desktop/path/to/changed_#1_filename.#2";
        let filenames = vec![
            "/Desktop/path/to/some_A_filename.txt".to_string(),
            "/Desktop/path/to/some_B_filename.jpg".to_string(),
            "/Desktop/path/to/some__filename.gif".to_string(),
            "/Desktop/path/to/some_jnskfjnes_filename.c".to_string(),
        ];
        let move_builder = MoveBuilder::new(&template_from, &filenames);
        let parsed_target1 = ParsedTarget::new(&template_to1);
        // let result = Result<(Vec<String>, Vec<String>), (usize, usize)>
        let result1: Result<(Vec<String>, Vec<String>), TemplateError> = Ok((
            filenames.clone(),
            vec![
                "/home/Desktop/path/to/changed_A_filename.txt".to_string(),
                "/home/Desktop/path/to/changed_B_filename.jpg".to_string(),
                "/home/Desktop/path/to/changed__filename.gif".to_string(),
                "/home/Desktop/path/to/changed_jnskfjnes_filename.c".to_string(),
            ],
        ));
        assert_eq!(move_builder.build_target_names(&parsed_target1), result1);

        let result2: Result<(Vec<String>, Vec<String>), TemplateError> = Err(TemplateError {
            asterisks: 2,
            hashes: 3,
        });
        let template_to2 = "/home/Desktop/path/to/changed_#1_fil#3ename.#2";
        let parsed_target2 = ParsedTarget::new(&template_to2);
        assert_eq!(move_builder.build_target_names(&parsed_target2), result2)
    }
    #[test]
    fn test_building_target_files2() {
        let template_from = "/Desktop/path/to/some_*_filename.*";
        let template_to = "/home/Desktop/path#1/to/changed_#1_#2_#1_#2_#1_filename.#2";
        let filenames = vec![
            "/Desktop/path/to/some_A_filename.txt".to_string(),
            "/Desktop/path/to/some_B_filename.jpg".to_string(),
            "/Desktop/path/to/some__filename.gif".to_string(),
            "/Desktop/path/to/some_jnskfjnes_filename.c".to_string(),
        ];
        let move_builder = MoveBuilder::new(&template_from, &filenames);
        let parsed_target = ParsedTarget::new(&template_to);
        let result: Result<(Vec<String>, Vec<String>), TemplateError> = Ok((
            filenames.clone(),
            vec![
            "/home/Desktop/pathA/to/changed_A_txt_A_txt_A_filename.txt".to_string(),
            "/home/Desktop/pathB/to/changed_B_jpg_B_jpg_B_filename.jpg".to_string(),
            "/home/Desktop/path/to/changed__gif__gif__filename.gif".to_string(),
            "/home/Desktop/pathjnskfjnes/to/changed_jnskfjnes_c_jnskfjnes_c_jnskfjnes_filename.c".to_string(),
        ]));
        assert_eq!(move_builder.build_target_names(&parsed_target), result)
    }
}
