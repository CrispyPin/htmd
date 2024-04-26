use std::{
	error::Error,
	ffi::OsStr,
	fs::{read_dir, read_to_string, File},
	io::Write,
	path::{Path, PathBuf},
};

const SRC_DIR: &str = "write";
const OUT_DIR: &str = "site";
type Result = core::result::Result<(), Box<dyn Error>>;

fn main() -> Result {
	let src_dir = PathBuf::from(SRC_DIR);
	build_dir(&src_dir)
}

fn build_dir(dir: &Path) -> Result {
	for entry in read_dir(dir).unwrap().flatten() {
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
		todo!("move file")
	}
}

#[derive(Debug, Clone, PartialEq)]
enum S {
	None,
	P,
	Code,
}

fn convert_file(path: &Path) -> Result {
	let markdown = read_to_string(path)?;
	let out_path = PathBuf::from(OUT_DIR).join(path.strip_prefix(SRC_DIR)?);
	let out_path = out_path.with_extension("html");

	let mut html = String::new();
	let mut state = S::None;

	for line in markdown.lines() {
		if line.starts_with("```") {
			if state == S::Code {
				html += "</pre>\n";
				state = S::None;
				continue;
			} else {
				if state == S::P {
					html += "</p>\n";
				}
				state = S::Code;
				html += "<pre>\n";
				continue;
			}
		}

		if state == S::Code {
			html += line;
			html += "\n";
			continue;
		}

		if let Some((start, header)) = line.split_once(' ') {
			let level = start.len();
			if (1..=6).contains(&level) && start.chars().all(|c| c == '#') {
				if state == S::P {
					state = S::None;
					html += "</p>\n";
				}
				let header = &convert_line(header);
				html += &format!("<h{level}>{header}</h{level}>\n");
				continue;
			}
		}

		if state == S::P && line.is_empty() {
			state = S::None;
			html += "</p>\n";
		} else if !line.is_empty() {
			if state == S::None {
				state = S::P;
				html += "<p>\n";
			}
			html += &convert_line(line);
			html += "<br>\n";
		}
	}

	let template = read_to_string("template.html")?;
	let html = template.replace("CONTENT HERE", &html);

	let mut file = File::create(out_path)?;
	file.write_all(html.as_bytes())?;

	Ok(())
}

fn convert_line(source: &str) -> String {
	let mut out = String::new();
	let mut is_em = false;
	let mut is_b = false;
	let mut is_code = false;
	let mut is_ul = false;
	let toggle = |state: bool, tag: &str| {
		if state {
			format!("<{tag}>")
		} else {
			format!("</{tag}>")
		}
	};

	let mut link: Option<(String, Option<String>)> = None;

	let mut chars = source.chars().peekable();
	while let Some(c) = chars.next() {
		if let Some(link_c) = &mut link {
			match link_c {
				(link_text, None) => {
					if c == ']' {
						if chars.peek() == Some(&'(') {
							_ = chars.next();
							link_c.1 = Some(String::new());
						} else {
							out += &format!("[{link_text}]");
							link = None;
						}
					} else {
						link_text.push(c);
					}
				}
				(link_text, Some(href)) => {
					if c == ')' {
						out += &format!("<a href=\"{href}\">{link_text}</a>");
						link = None;
					} else {
						href.push(c);
					}
				}
			}
			continue;
		}
		if c == '[' {
			link = Some((String::new(), None));
		} else if c == '*' {
			if chars.peek() == Some(&'*') {
				_ = chars.next();
				is_b = !is_b;
				out += &toggle(is_b, "strong");
			} else {
				is_em = !is_em;
				out += &toggle(is_em, "em");
			}
		} else if c == '`' {
			is_code = !is_code;
			out += &toggle(is_code, "code");
		} else if c == '_' {
			is_ul = !is_ul;
			out += &toggle(is_ul, "u");
		} else {
			out.push(c);
		}
	}
	if let Some((link_text, href)) = link {
		out += &format!("[{link_text}");
		if let Some(href) = href {
			out += &format!("]({href}");
		}
	}

	if is_em {
		out += &toggle(false, "em");
	}
	if is_b {
		out += &toggle(false, "b");
	}
	if is_code {
		out += &toggle(false, "code");
	}
	if is_ul {
		out += &toggle(false, "u");
	}
	out
}
