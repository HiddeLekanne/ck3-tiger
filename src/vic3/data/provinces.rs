use std::path::{Path, PathBuf};

use fnv::FnvHashSet;
use image::{DynamicImage, Rgb};

use crate::everything::Everything;
use crate::fileset::{FileEntry, FileHandler};
use crate::report::{err, warn, ErrorKey};
use crate::token::Token;

#[derive(Clone, Debug, Default)]
pub struct Provinces {
    /// Colors in the provinces.png
    colors: FnvHashSet<Rgb<u8>>,

    /// Kept and used for error reporting.
    provinces_png: Option<FileEntry>,
}

impl Provinces {
    pub fn verify_exists_implied(&self, key: &str, item: &Token) {
        if !self.exists(key) {
            // TODO: determine the severity of a missing province
            let msg = "province not found on map";
            warn(ErrorKey::MissingItem).msg(msg).loc(item).push();
        }
    }

    pub fn exists(&self, key: &str) -> bool {
        // If we failed to load the provinces.png, then don't complain about individual provinces not being found.
        if self.provinces_png.is_none() {
            return true;
        }
        if key.len() != 7 {
            return false; // not a valid province id
        }
        if let Some(hexid) = key.strip_prefix("x") {
            if let Ok(r) = u8::from_str_radix(&hexid[0..2], 16) {
                if let Ok(g) = u8::from_str_radix(&hexid[2..4], 16) {
                    if let Ok(b) = u8::from_str_radix(&hexid[4..6], 16) {
                        return self.colors.contains(&Rgb([r, g, b]));
                    }
                }
            }
        }
        false
    }

    #[allow(clippy::unused_self)]
    pub fn validate(&self, _data: &Everything) {}
}

impl FileHandler<DynamicImage> for Provinces {
    fn subpath(&self) -> PathBuf {
        PathBuf::from("map_data/provinces.png")
    }

    fn load_file(&self, entry: &FileEntry, fullpath: &Path) -> Option<DynamicImage> {
        if entry.path().components().count() == 2 {
            let img = match image::open(fullpath) {
                Ok(img) => img,
                Err(e) => {
                    let msg = format!("could not read `{}`: {e:#}", entry.path().display());
                    // TODO: does this crash?
                    err(ErrorKey::ReadError).msg(msg).loc(entry).push();
                    return None;
                }
            };
            if let DynamicImage::ImageRgb8(_) = img {
                return Some(img);
            } else {
                let msg = format!(
                    "`{}` has wrong color format `{:?}`, should be Rgb8",
                    entry.path().display(),
                    img.color()
                );
                // TODO: does this crash?
                err(ErrorKey::ImageFormat).msg(msg).loc(entry).push();
            }
        }
        None
    }

    fn handle_file(&mut self, entry: &FileEntry, img: DynamicImage) {
        self.provinces_png = Some(entry.clone());
        if let DynamicImage::ImageRgb8(img) = img {
            for pixel in img.pixels() {
                self.colors.insert(*pixel);
            }
        }
    }
}