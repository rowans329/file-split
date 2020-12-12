// Lib imports
use file_split::cli::Cli;
use file_split::error::UnwrapOrExit;
use file_split::utils;

fn main() {
    let cli = Cli::from_args().unwrap_or_exit();

    let iter = utils::split(
        &utils::open_file(&cli.input_path).unwrap_or_exit(),
        cli.lines,
        cli.include_headers,
    );

    utils::create_split_dir(&cli.working_directory, &cli.new_name_base).unwrap_or_exit();

    for (n, chunk) in iter.enumerate() {
        utils::create_split_file(
            &cli.working_directory,
            &cli.new_name_base,
            n,
            &cli.extension,
            &chunk,
        )
        .unwrap_or_exit();
    }
}
