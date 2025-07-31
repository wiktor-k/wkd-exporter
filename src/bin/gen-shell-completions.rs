fn main() -> Result<(), clap_allgen::Error> {
    clap_allgen::render_shell_completions::<wkd_exporter::cli::Cli>(
        &std::env::args().collect::<Vec<_>>()[1],
    )
}
