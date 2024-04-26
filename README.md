# static site generator

all files get copied from the `write` dir to the `site` dir

md files get converted to html in the process

using `template.html`, which should include `TITLE` and `CONTENT`

utf-8 is required


ideas:
```
/blog/post_1.md -> /blog/post_1/index.html
/blog/ferret.mp4 -> /blog/post_1/ferret.mp4

/blog/post_2/post_2.md -> /blog/post_2/index.html
/blog/post_2/cat.png -> /blog/post_2/cat.png
```
