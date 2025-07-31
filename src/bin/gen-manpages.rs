fn main() -> Result<(), clap_allgen::Error> {
    clap_allgen::render_manpages::<wkd_exporter::cli::Cli>(&std::env::args().collect::<Vec<_>>()[1])
}
