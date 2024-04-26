# static site generator
All files get copied from the `write` dir to the `site` dir

Markdown files get converted to html in the process

HTML is generated from `template.html`, with`{FILENAME}`(optional) and `{CONTENT}` getting replaced.

This markdown is non compliant and I don't care. `*Emphasis*`, `**bold**` and `_underlined_` are implemented, as well as ``code``.
Standard `## Headers` up to 6 levels, and \`\`\` at the start of a line for code block (html `<pre>`) toggling. Backslashes for escaping may be added later.
There are no lists, but also automatic line breaks everywhere, so they look fine without needing html lists


ideas:
```
/blog/post_1.md -> /blog/post_1/index.html
/blog/ferret.mp4 -> /blog/post_1/ferret.mp4

/blog/post_2/post_2.md -> /blog/post_2/index.html
/blog/post_2/cat.png -> /blog/post_2/cat.png
```
