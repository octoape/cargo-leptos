use crate::{
    config::Project,
    ext::{eyre::CustomWrapErr, PathBufExt},
    internal_prelude::*,
};
use base64ct::{Base64UrlUnpadded, Encoding};
use camino::Utf8PathBuf;
use eyre::{ContextCompat, Result};
use md5::{Digest, Md5};
use std::{collections::HashMap, fs};

///Adds hashes to the filenames of the css, js, and wasm files in the output
pub fn add_hashes_to_site(proj: &Project) -> Result<()> {
    let files_to_hashes = compute_front_file_hashes(proj).dot()?;

    debug!("Hash computed: {files_to_hashes:?}");

    let renamed_files = rename_files(&files_to_hashes).dot()?;
    let pkg_dir = proj.site.root_relative_pkg_dir();

    replace_in_file(
        &renamed_files[&proj.lib.js_file.dest],
        &renamed_files,
        &pkg_dir,
    );

    if proj.split {
        let old_wasm_split = proj
            .lib
            .js_file
            .dest
            .clone()
            .without_last()
            .join("__wasm_split.______________________.js");
        let new_wasm_split = &renamed_files[&old_wasm_split];
        replace_in_file(new_wasm_split, &renamed_files, &pkg_dir);

        let old_wasm_split_filename = old_wasm_split.file_name().unwrap();
        let new_wasm_split_filename = new_wasm_split.file_name().unwrap();

        for entry in fs::read_dir(&pkg_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                    if filename.ends_with(".wasm") {
                        replace_in_binary_file(
                            &Utf8PathBuf::try_from(path).unwrap(),
                            old_wasm_split_filename,
                            new_wasm_split_filename,
                        );
                    } else if filename.starts_with("__wasm_split") {
                        replace_in_file(
                            &Utf8PathBuf::try_from(path).unwrap(),
                            &renamed_files,
                            &pkg_dir,
                        );
                    }
                }
            }
        }
    }

    fs::create_dir_all(
        proj.hash_file
            .abs
            .parent()
            .wrap_err_with(|| format!("no parent dir for {}", proj.hash_file.abs))?,
    )
    .wrap_err_with(|| format!("Failed to create parent dir for {}", proj.hash_file.abs))?;

    fs::write(
        &proj.hash_file.abs,
        format!(
            "{}: {}\n{}: {}\n{}: {}\n",
            proj.lib
                .js_file
                .dest
                .extension()
                .ok_or(eyre!("no extension"))?,
            files_to_hashes[&proj.lib.js_file.dest],
            proj.lib
                .wasm_file
                .dest
                .extension()
                .ok_or(eyre!("no extension"))?,
            files_to_hashes[&proj.lib.wasm_file.dest],
            proj.style
                .site_file
                .dest
                .extension()
                .ok_or(eyre!("no extension"))?,
            files_to_hashes[&proj.style.site_file.dest]
        ),
    )
    .wrap_err_with(|| format!("Failed to write hash file to {}", proj.hash_file.abs))?;

    debug!("Hash written to {}", proj.hash_file.abs);

    Ok(())
}

fn compute_front_file_hashes(proj: &Project) -> Result<HashMap<Utf8PathBuf, String>> {
    let mut files_to_hashes = HashMap::new();

    let mut stack = vec![proj.site.root_relative_pkg_dir().into_std_path_buf()];

    while let Some(path) = stack.pop() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "css" && path != proj.style.site_file.dest {
                            continue;
                        }
                    }

                    // Check if the path contains snippets and also if it
                    // contains inline{}.js. We do not want to hash these files
                    // as the webassembly will look for an unhashed version of
                    // the .js file. The folder though can be hashed.
                    if let Some(path_str) = path.to_str() {
                        if path_str.contains("snippets") {
                            if let Some(file_name) = path.file_name() {
                                let file_name_str = file_name.to_string_lossy();
                                if file_name_str.contains("inline") {
                                    if let Some(extension) = path.extension() {
                                        if extension == "js" {
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let hash = Base64UrlUnpadded::encode_string(
                        &Md5::new().chain_update(fs::read(&path)?).finalize(),
                    );

                    files_to_hashes.insert(
                        Utf8PathBuf::from_path_buf(path).expect("invalid path"),
                        hash,
                    );
                } else if path.is_dir() {
                    stack.push(path);
                }
            }
        }
    }

    Ok(files_to_hashes)
}

fn rename_files(
    files_to_hashes: &HashMap<Utf8PathBuf, String>,
) -> Result<HashMap<Utf8PathBuf, Utf8PathBuf>> {
    const HASH_PLACEHOLDER: &str = "______________________";
    let mut old_to_new_paths = HashMap::new();

    for (path, hash) in files_to_hashes {
        let mut new_path = path.clone();

        let file_name = new_path.file_name().unwrap_or_default();

        let new_file_name = if file_name.contains(HASH_PLACEHOLDER) {
            if hash.len() != HASH_PLACEHOLDER.len() {
                return Err(anyhow!(
                    "File hash length did not match placeholder hash length."
                ));
            }
            file_name.replace(HASH_PLACEHOLDER, hash)
        } else {
            format!(
                "{}.{}.{}",
                path.file_stem().ok_or(eyre!("no file stem"))?,
                hash,
                path.extension().ok_or(eyre!("no extension"))?,
            )
        };

        new_path.set_file_name(new_file_name);

        fs::rename(path, &new_path)
            .wrap_err_with(|| format!("Failed to rename {path} to {new_path}"))?;

        old_to_new_paths.insert(path.clone(), new_path);
    }

    Ok(old_to_new_paths)
}

fn replace_in_file(
    path: &Utf8PathBuf,
    old_to_new_paths: &HashMap<Utf8PathBuf, Utf8PathBuf>,
    root_dir: &Utf8PathBuf,
) {
    let mut contents = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("error {e}: could not read file {path}"));

    for (old_path, new_path) in old_to_new_paths {
        let old_path = old_path
            .strip_prefix(root_dir)
            .expect("could not strip root path");
        let new_path = new_path
            .strip_prefix(root_dir)
            .expect("could not strip root path");

        contents = contents.replace(old_path.as_str(), new_path.as_str());
    }

    fs::write(path, contents).expect("could not write file");
}

fn replace_in_binary_file(path: &Utf8PathBuf, old_wasm_split: &str, new_wasm_split: &str) {
    let mut contents =
        fs::read(path).unwrap_or_else(|e| panic!("error {e}: could not read file {path}"));

    let old_path = old_wasm_split.as_bytes();
    let new_path = new_wasm_split.as_bytes();

    for i in 0..=contents.len() - old_path.len() {
        if contents[i..].starts_with(old_path) {
            contents[i..(i + old_path.len())].clone_from_slice(new_path);
        }
    }

    fs::write(path, contents).expect("could not write file");
}
