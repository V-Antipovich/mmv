pub mod files_operations;
pub mod template_handling;
use crate::template_handling::{MoveBuilder, ParsedTarget};
use clap::Parser;
use files_operations::{get_files_by_template, mass_move};
use std::process::exit;

#[derive(Parser, Debug)]
struct Arguments {
    /// Choice files template. Asterisk '*' stands for any sequence of symbols in file name (not in directories)
    files_template: String,
    /// Target files template. Inserting '#n', where n is a number 1-9 means you want the sequence under n-th asteriks be placed here
    target_template: String,
    /// Will overwrite the target files if they are present in the directory
    #[clap(long, short)]
    force: bool,
}

fn main() {
    let arguments = Arguments::parse();
    let files_by_template = get_files_by_template(&arguments.files_template);
    match files_by_template {
        Err(error_template) => {
            eprintln!("{}", error_template);
            exit(1);
        },
        Ok(filenames) => {
            let move_builder = MoveBuilder::new(&arguments.files_template, &filenames);
            let parsed_target = ParsedTarget::new(&arguments.target_template);
            let files_pairs = move_builder.build_target_names(&parsed_target);
            match files_pairs {
                Err(template_error)  => {
                    eprintln!("{}", template_error);
                    exit(1);
                }
                Ok((initial_filenames, target_filenames)) => {
                    let resilt_mass_move =
                        mass_move(&initial_filenames, &target_filenames, arguments.force);
                    match resilt_mass_move {
                        Ok(_) => {
                            for i in 0..initial_filenames.len() {
                                println!("{} -> {}", &initial_filenames[i], &target_filenames[i]);
                            }
                            println!("mmv: Succeded!");
                            exit(0);
                        },
                        Err(replace_error) => {
                            eprintln!("{}", replace_error);
                            exit(1);
                        },
                    }
                }
            }
        }
    }
}
