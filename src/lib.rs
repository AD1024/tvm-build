use std::path::{Path, PathBuf};

use git2::build::RepoBuilder;
use thiserror::Error;
use tracing::{self, info};
use cmake;
use dirs;

mod core;
mod targets;

use targets::local_target;

pub use self::core::BuildConfig;

/// Build TVM given a build configuration.
#[tracing::instrument]
pub fn build(build_config: core::BuildConfig) -> Result<(), core::Error> {
    info!("tvm_build::build");
    let repo_path = core::init_tvm_build_dir(&build_config)?;

    let mut cmake_config = cmake::Config::new(repo_path.clone());

    let target = local_target();

    // TODO(@jroesch): map this to target triple based target directory
    // should probably be target + host + profile.
    let build_path = match build_config.output_path {
        None => repo_path.join("..").join("build"),
        _ => panic!(),
    };

    if !build_path.exists() {
        std::fs::create_dir_all(build_path.clone()).unwrap();
    }

    let dst = cmake_config
        .generator("Unix Makefiles")
        .out_dir(build_path)
        .very_verbose(true)
        .target(&target.target_str)
        .host(&target.host)
        .profile("Debug")
        .build();


    // info!(target = target.target_str);
    // info!(dst = dst.display().to_string());

    Ok(())
}
