#[derive(Debug, Clone, PartialEq)]
enum S {
	None,
	P,
	Code,
	Html,
}

pub fn convert_document(markdown: &str) -> String {
	let mut html = String::new();
	let mut state = S::None;

	for line in markdown.lines() {
		if line.starts_with("```") {
			if state == S::Code {
				html += "</pre>\n";
				state = S::None;
			} else {
				if state == S::P {
					html += "</p>\n";
				}
				state = S::Code;
				html += "<pre>\n";
			}
			continue;
		}

		if line.starts_with("{RAW_HTML}") {
			if state == S::Html {
				state = S::None;
			} else {
				if state == S::P {
					html += "</p>\n";
				}
				state = S::Html;
			}
			continue;
		}

		if state == S::Code {
			html += line;
			html += "\n";
			continue;
		}

		if let Some(img_info) = line.strip_prefix("==image:") {
			let mut attributes = img_info.split(':');
			let src = attributes.next().unwrap();
			let alt = attributes.next().unwrap_or_else(|| &"\"\"");
			html += &format!("<img loading=\"lazy\" src={src} alt={alt} title={alt}></img>\n");
			continue;
		}

		if let Some(video_info) = line.strip_prefix("==video:") {
			let mut attributes = video_info.split(':');
			let src = attributes.next().unwrap();
			let alt = attributes.next().unwrap_or_else(|| &"\"\"");
			let extra = attributes.next().unwrap_or_default();
			html += &format!("<video loading=\"lazy\" src={src} alt={alt} title={alt} controls {extra}></video>\n");
			continue;
		}

		if let Some(youtube_id) = line.strip_prefix("==youtube:") {
			html += &format!("<a href=\"https://youtu.be/{youtube_id}\">https://youtu.be/{youtube_id}</a>\n<iframe loading=\"lazy\" src=\"https://www.youtube-nocookie.com/embed/{youtube_id}\" width=\"600px\" height=\"340px\" allowfullscreen></iframe>");
			continue;
		}

		if let Some((start, header)) = line.split_once(' ') {
			let level = start.len();
			if (1..=6).contains(&level) && start.chars().all(|c| c == '#') {
				if state == S::P {
					state = S::None;
					html += "</p>\n";
				}
				let id: String = header
					.to_lowercase()
					.chars()
					.filter_map(|c| match c {
						' ' => Some('-'),
						'-' | '_' => Some(c),
						_ => c.is_alphanumeric().then_some(c),
					})
					.collect();
				let header = &convert_line(header);
				html += &format!("<h{level} id=\"{id}\">{header}</h{level}>\n");
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
			if line.starts_with('<') {
				html += line;
				html += "\n";
			} else {
				html += &convert_line(line);
				html += "<br>\n";
			}
		}
	}
	html
}

fn convert_line(source: &str) -> String {
	let mut out = String::new();
	let mut is_em = false;
	let mut is_b = false;
	let mut is_code = false;
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
		out += "</em>";
	}
	if is_b {
		out += "</strong>";
	}
	if is_code {
		out += "</code>";
	}
	out
}
