use std::{
	error::Error,
	ffi::OsStr,
	fs::{read_dir, read_to_string, DirBuilder, File},
	io::Write,
	path::{Path, PathBuf},
};

mod convert;
use convert::convert_document;

const SRC_DIR: &str = "write";
const OUT_DIR: &str = "site";
const CONTENT_MARKER: &str = "{CONTENT}";
const FILENAME_MARKER: &str = "{FILENAME}";

const TEMPLATE_FILE: &str = "template.html";
const DEFAULT_TEMPLATE: &[u8] = include_bytes!("../template.html");

type Result = core::result::Result<(), Box<dyn Error>>;

fn main() -> Result {
	if !PathBuf::from(TEMPLATE_FILE).is_file() {
		File::create(TEMPLATE_FILE)?.write_all(DEFAULT_TEMPLATE)?;
		println!("created {TEMPLATE_FILE}");
	}
	if !PathBuf::from(SRC_DIR).is_dir() {
		DirBuilder::new().create(SRC_DIR)?;
		println!("created {SRC_DIR}/");
	}
	if !PathBuf::from(OUT_DIR).is_dir() {
		DirBuilder::new().create(OUT_DIR)?;
		println!("created {OUT_DIR}/");
	}

	let src_dir = PathBuf::from(SRC_DIR);
	build_dir(&src_dir)
}

fn build_dir(dir: &Path) -> Result {
	for entry in read_dir(dir)?.flatten() {
		let ftype = entry.file_type()?;
		if ftype.is_dir() {
			build_dir(&entry.path())?;
		} else if ftype.is_file() {
			build_file(&entry.path())?;
		}
	}
	Ok(())
}

fn build_file(path: &Path) -> Result {
	if path.extension().and_then(OsStr::to_str) == Some("md") {
		convert_file(path)
	} else {
		println!("file ignored: {}", path.display());
		// TODO copy non dotfiles
		Ok(())
	}
}

fn convert_file(path: &Path) -> Result {
	let out_path = PathBuf::from(OUT_DIR).join(path.strip_prefix(SRC_DIR)?);
	let out_path = out_path.with_extension("html");

	let markdown = read_to_string(path)?;
	let content = convert_document(&markdown);
	let template = read_to_string(TEMPLATE_FILE)?;
	let filename = path.file_stem().unwrap().to_string_lossy().to_string();
	let html = template
		.replacen(CONTENT_MARKER, &content, 1)
		.replace(FILENAME_MARKER, &filename);

	DirBuilder::new()
		.recursive(true)
		.create(out_path.parent().unwrap())?;
	let mut file = File::create(&out_path)?;
	file.write_all(html.as_bytes())?;
	println!("built {}", out_path.display());
	Ok(())
}
