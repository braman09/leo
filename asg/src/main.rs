// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_asg::{Asg, AsgConvertError, NullImportResolver};
use leo_ast::Ast;
use leo_grammar::Grammar;
use std::{env, path::Path, time::Instant};

fn to_leo_graph(filepath: &Path) -> Result<(), AsgConvertError> {
    // Loads the Leo code as a string from the given file path.
    let program_filepath = filepath.to_path_buf();
    let program_string = Grammar::load_file(&program_filepath)?;

    // Parses the Leo file and constructs a pest ast.
    let ast = Grammar::new(&program_filepath, &program_string)?;

    // Parse the pest ast and constructs a ast.
    let mut leo_ast = Ast::new("leo_tree", &ast)?;

    let ctx = leo_asg::new_context();

    // Set import resolver to null
    let mut import_resolver = NullImportResolver;

    // Create a new symbol table from the program, imported_programs, and program_input.

    // calculate the execution time to generate the asg from the ast
    let timer = Instant::now();

    let _asg = Asg::new(&ctx, &mut leo_ast, &mut import_resolver)?;

    println!("Generated the ASG in {} microseconds \n", timer.elapsed().as_micros());

    // TODO (raychu86): Return the asg.
    Ok(())
}

fn main() -> Result<(), AsgConvertError> {
    // Parse the command-line arguments as strings.
    let cli_arguments = env::args().collect::<Vec<String>>();

    // Check that the correct number of command-line arguments were passed in.
    if cli_arguments.len() < 2 || cli_arguments.len() > 3 {
        eprintln!("Warning - an invalid number of command-line arguments were provided.");
        println!(
            "\nCommand-line usage:\n\n\tleo_asg {{PATH/TO/INPUT_FILENAME}}.leo {{PATH/TO/OUTPUT_DIRECTORY (optional)}}\n"
        );
        return Ok(()); // Exit innocently
    }

    // Construct the input filepath.
    let input_filepath = Path::new(&cli_arguments[1]);

    // Construct the serialized syntax graph.
    let _asg = to_leo_graph(&input_filepath)?;

    // Determine the output directory.
    let _output_directory = match cli_arguments.len() == 3 {
        true => format!(
            "{}/{}.json",
            cli_arguments[2],
            input_filepath.file_stem().unwrap().to_str().unwrap()
        ),
        false => format!("./{}.json", input_filepath.file_stem().unwrap().to_str().unwrap()),
    };

    Ok(())
}
