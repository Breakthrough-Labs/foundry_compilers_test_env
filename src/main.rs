use foundry_compilers::{remappings::Remapping, Project, ProjectPathsConfig};
use std::{path::PathBuf, vec};

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let _remappings = Remapping::find_many("./ajna-core");
    let project = Project::builder()
        .set_auto_detect(true)
        .set_build_info(false)
        .set_no_artifacts(true)
        .set_cached(false)
        .paths(ProjectPathsConfig {
            root: manifest_dir.clone(),
            cache: manifest_dir.join(""),
            artifacts: manifest_dir.join(""),
            build_infos: manifest_dir.join("./build-infos"),
            sources: manifest_dir.join("./ajna-core"),
            tests: manifest_dir.join("tests"),
            scripts: manifest_dir.join("scripts"),
            libraries: vec![manifest_dir.join("lib")],
            remappings: _remappings,
        })
        .build()
        .unwrap();
    let output = project.compile().unwrap();
}
