use color_eyre::eyre::Result;
use serde::Deserialize;

use std::path::Path;

use crate::Dirs;
use crate::Install;

#[derive(Deserialize)]
pub enum Type {
    #[serde(rename(deserialize = "custom"))]
    Custom,
    #[serde(rename(deserialize = "rust"))]
    Rust,
}

#[derive(Deserialize)]
pub struct Package {
    name: String,
    version: String,
    program_type: Type,
    #[serde(default)]
    exe: Vec<Install>,
    #[serde(default)]
    libs: Vec<Install>,
    #[serde(default)]
    man: Vec<Install>,
    #[serde(default)]
    data: Vec<Install>,
    #[serde(default)]
    docs: Vec<Install>,
}

impl Package {
    pub fn paths(
        self,
        dirs: Dirs,
    ) -> Result<Vec<Install>> {
        let mut results = Vec::new();

        let bin_local_root = match self.program_type {
            Type::Rust => Some(Path::new("target/release")),
            Type::Custom => None,
        };

        macro_rules! install_files {
            ( $files:tt, $install_dir:expr, $parent_dir:expr ) => {
                self.$files
                    .into_iter()
                    .map(|install| -> Result<Install> {
                        install.sanitize($install_dir, $parent_dir)
                    })
                    .collect::<Result<Vec<Install>>>()?
            };
        }

        results.extend(install_files!(exe, &dirs.bindir, bin_local_root));
        results.extend(install_files!(libs, &dirs.libdir, None));
        results.extend(install_files!(data, &dirs.datadir, None));
        if let Some(mandir) = &dirs.mandir {
            results.extend(install_files!(man, mandir, None));
        }

        let package_dir = format!("{}-{}", self.name.to_owned(), self.version);
        if let Some(docdir) = &dirs.docdir {
            results.extend(install_files!(
                docs,
                &docdir.join(Path::new(&package_dir)),
                None
            ));
        }

        Ok(results)
    }
}
