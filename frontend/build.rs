use std::path;

use slint_build;

fn main() {
    let config = slint_build::CompilerConfiguration::new().with_library_paths(
	std::collections::HashMap::from([(
	    "material".to_string(),
	    path::Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("data/widgets/material-1.0/material.slint")
	)])
    );

    slint_build::compile_with_config("./data/ui/main_window.slint", config).unwrap();
}
