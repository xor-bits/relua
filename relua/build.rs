fn main() {
    lalrpop::Configuration::new()
        .emit_rerun_directives(true)
        .use_colors_if_tty()
        .process_current_dir()
        .unwrap();
}
