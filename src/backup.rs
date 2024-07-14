use anyhow::{bail, Result};
use chrono::prelude::*;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

const MANIFEST_NAME: &str = "manifest.yaml";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Manifest {
    pairs: Vec<Pair>,
}

impl Manifest {
    pub fn find_by_name<P>(&self, name: P) -> Option<&Pair>
    where
        P: AsRef<Path>,
    {
        self.pairs.iter().find_map(|pair| {
            if Path::new(&pair.name) == name.as_ref() {
                return Some(pair);
            }

            None
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Pair {
    name: String,
    path: PathBuf,
}

pub struct Backup {
    manifest: Manifest,
}

impl Backup {
    pub fn new() -> Self {
        Self {
            manifest: Manifest { pairs: vec![] },
        }
    }

    pub fn add<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        let name = match path.as_ref().file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => {
                // TODO: Better message
                warn!(
                    "Couldn't get the file name of {}. It will not be in the backup",
                    path.as_ref().display()
                );
                return;
            }
        };

        let pair = Pair {
            name,
            path: path.as_ref().to_path_buf(),
        };

        self.manifest.pairs.push(pair)
    }

    pub fn save<P>(&self, path: P) -> Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        let date = Local::now().format("%Y-%m-%d_%Hh-%Mm-%Ss").to_string();
        let tar_path = path.as_ref().join(date).with_extension("tar.gz");
        let tar = fs::File::create(&tar_path)?;
        let enconder = GzEncoder::new(tar, Compression::default());
        let mut archive = tar::Builder::new(enconder);

        for pair in self.manifest.pairs.iter() {
            archive.append_path_with_name(&pair.path, &pair.name)?;
        }

        let temp = tempdir()?;
        let manifest_path = temp.path().join("manifest");
        fs::write(&manifest_path, serde_yaml::to_string(&self.manifest)?)?;
        archive.append_path_with_name(&manifest_path, MANIFEST_NAME)?;
        archive.finish()?;

        info!("Backup written at {}", tar_path.display());
        Ok(tar_path)
    }

    pub fn save_as_last<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let backup_path = self.save(&path)?;

        std::os::unix::fs::symlink(backup_path, path.as_ref().join("last"))?;

        Ok(())
    }

    fn extract_manifest<P>(path: P) -> Result<Manifest>
    where
        P: AsRef<Path>,
    {
        let tar = fs::File::open(&path)?;
        let decoder = GzDecoder::new(&tar);
        let mut archive = tar::Archive::new(decoder);
        let manifest = archive.entries()?.find_map(|entry| {
            let mut entry = entry.ok()?;

            if !(entry.path().ok()? == Path::new(MANIFEST_NAME)) {
                return None;
            }

            let mut yaml = String::new();
            entry.read_to_string(&mut yaml).ok()?;

            Some(serde_yaml::from_str::<Manifest>(&yaml).ok()?)
        });

        if manifest.is_none() {
            bail!(
                "Could not read {} from {}",
                MANIFEST_NAME,
                path.as_ref().display()
            );
        }

        Ok(manifest.unwrap())
    }

    pub fn restore<P>(path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let tar = fs::File::open(&path)?;
        let decoder = GzDecoder::new(&tar);
        let mut archive = tar::Archive::new(decoder);
        let manifest = Backup::extract_manifest(&path)?;

        for entry in archive.entries()? {
            let mut entry = entry?;

            if entry.path()? == Path::new(MANIFEST_NAME) {
                continue;
            }

            let pair = match manifest.find_by_name(entry.path()?) {
                Some(pair) => pair,
                None => bail!(
                    "Unknow file {} at {}",
                    entry.path()?.display(),
                    path.as_ref().display()
                ),
            };

            entry.unpack(&pair.path)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use tempfile::tempdir;

    #[test]
    fn test_restore() -> Result<()> {
        let dir = tempdir()?;
        let files = ["file1", "file2", "file3"];
        let mut backup = Backup::new();

        for file in files.iter() {
            let path = dir.path().join(file);

            fs::write(&path, "some text here")?;
            backup.add(&path);
        }

        let path = backup.save(dir.path())?;

        for file in files.iter() {
            let path = dir.path().join(file);

            fs::write(&path, "")?;
        }

        Backup::restore(path)?;

        files.map(|file| {
            assert_eq!(
                fs::read_to_string(dir.path().join(file)).unwrap(),
                "some text here"
            )
        });

        Ok(())
    }
}
