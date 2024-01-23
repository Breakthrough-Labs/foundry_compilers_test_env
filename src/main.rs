use foundry_compilers::{remappings::Remapping, Project, ProjectPathsConfig};
use std::{path::PathBuf, vec};

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let _remappings = Remapping::find_many("hedge");
    let project = Project::builder()
        .set_auto_detect(true)
        .set_build_info(false)
        .set_no_artifacts(true)
        .set_cached(false)
        .solc_jobs(6)
        .paths(ProjectPathsConfig {
            root: manifest_dir.clone(),
            cache: manifest_dir.join(""),
            artifacts: manifest_dir.join(""),
            build_infos: manifest_dir.join("./build-infos"),
            sources: manifest_dir.join("hedge"),
            tests: manifest_dir.join("tests"),
            scripts: manifest_dir.join("scripts"),
            libraries: vec![manifest_dir.join("lib")],
            remappings: _remappings,
        })
        .build()
        .unwrap();
    let _output = project.compile().unwrap();
}
