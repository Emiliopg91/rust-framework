#[cfg(test)]
pub mod tests;

pub mod errors;

use std::{
    env, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::{
    fs::{self, File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::utils::file::errors::{FileError, Result};

pub struct FileUtils;

impl FileUtils {
    pub fn exists<T>(path: &T) -> bool
    where
        T: AsRef<Path>,
    {
        path.as_ref().exists()
    }

    pub fn is_file<T>(path: &T) -> bool
    where
        T: AsRef<Path>,
    {
        path.as_ref().is_file()
    }

    pub fn is_dir<T>(path: &T) -> bool
    where
        T: AsRef<Path>,
    {
        path.as_ref().is_dir()
    }

    pub fn delete<T>(path: &T) -> Result<()>
    where
        T: AsRef<Path>,
    {
        let path = path.as_ref();

        type RemoveFn = fn(&Path) -> io::Result<()>;
        let fn_pointer: Option<RemoveFn> = if path.is_file() {
            Some(|p| std::fs::remove_file(p))
        } else if path.is_dir() {
            Some(|p| std::fs::remove_dir_all(p))
        } else {
            None
        };

        if let Some(f) = fn_pointer {
            f(path).map_err(|e| FileError::FileDeletionError(path.to_path_buf(), e))
        } else {
            Ok(())
        }
    }

    pub async fn touch<T>(path: &T) -> Result<()>
    where
        T: AsRef<Path>,
    {
        let path = path.as_ref();
        if !Self::exists(&path) {
            let _ = fs::File::create_new(path)
                .await
                .map_err(|e| FileError::FileTouchError(path.to_path_buf(), e));
        } else {
            filetime::set_file_mtime(path, filetime::FileTime::now())
                .map_err(|e| FileError::FileTouchError(path.to_path_buf(), e))?
        }

        Ok(())
    }

    pub fn mkdir<T>(path: &T, parents: bool) -> Result<()>
    where
        T: AsRef<Path>,
    {
        type MkDirFn = fn(&Path) -> io::Result<()>;
        let fn_pointer: Option<MkDirFn> = if parents {
            Some(|p| std::fs::create_dir_all(p))
        } else {
            Some(|p| std::fs::create_dir(p))
        };

        if let Some(f) = fn_pointer {
            f(path.as_ref()).map_err(|e| FileError::MakeDirError(path.as_ref().to_path_buf(), e))?
        }

        Ok(())
    }

    pub async fn write<T, I>(path: &T, append: bool, content: &I) -> Result<()>
    where
        T: AsRef<Path>,
        I: AsRef<[u8]>,
    {
        let mut file = FileUtils::open_file(&path, false, true, append, true).await?;
        file.write_all(content.as_ref())
            .await
            .map_err(|e| FileError::FileWriteError(path.as_ref().to_path_buf(), Box::new(e)))?;
        file.flush()
            .await
            .map_err(|e| FileError::FileWriteError(path.as_ref().to_path_buf(), Box::new(e)))?;

        Ok(())
    }

    pub async fn read<T>(path: T) -> Result<String>
    where
        T: AsRef<Path>,
    {
        let mut file = FileUtils::open_file(&path, true, false, false, false)
            .await
            .map_err(|e| FileError::FileReadError(path.as_ref().to_path_buf(), Box::new(e)))?;

        let mut buf = String::new();

        file.read_to_string(&mut buf)
            .await
            .map_err(|e| FileError::FileReadError(path.as_ref().to_path_buf(), Box::new(e)))?;

        Ok(buf)
    }

    pub async fn open_file<T>(
        path: &T,
        read: bool,
        write: bool,
        append: bool,
        create: bool,
    ) -> Result<File>
    where
        T: AsRef<Path>,
    {
        let mut options = OpenOptions::new();
        let mut options = options.read(read).write(write);
        if write {
            if append {
                options = options.append(true);
            } else {
                options = options.truncate(true);
            }
        }
        if create {
            options = options.create_new(true);
        }

        options
            .open(path.as_ref())
            .await
            .map_err(|e| FileError::OpenFileError(path.as_ref().display().to_string(), e))
    }

    pub fn new_tmp_file() -> Result<PathBuf> {
        let mut tmp_path = env::temp_dir();
        tmp_path.push(env!("CARGO_PKG_NAME"));
        if !FileUtils::exists(&tmp_path) {
            FileUtils::mkdir(&tmp_path, true).unwrap();
        }

        let mut now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        loop {
            let random_suffix = (now as u64) ^ (now as u64).wrapping_mul(6364136223846793005);
            let path_str = &format!("{}/{}", tmp_path.display(), random_suffix);
            let path = Path::new(path_str);
            if path.exists() {
                now += 1;
                continue;
            }
            return Ok(path.to_path_buf());
        }
    }
}
