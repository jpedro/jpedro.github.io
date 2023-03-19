import os
import sys

EXCLUDE = [
    "README.md",
    "index.md",
]

TRUTHY = [
    "true",
    "yes",
    "1",
]
COMMENT_START = "<!--"
COMMENT_END = "-->"

TEMPLATE_COMMENTS = """
 &nbsp;

<!--
Made with some <3 [Not a lot](https://github.com/jpedro/jpedro.github.io)

<script src="https://cdnjs.cloudflare.com/ajax/libs/firebase/9.18.0/firebase-app.js" integrity="sha512-djpBImoa+ot4QGqENJMYq/16OroxOsbJJemqnOpe4wMi8jDEM6iqRZl8H0JLpq1ao/sc5O7+weKuqIX3HbLtsQ==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/firebase/9.18.0/firebase-database.min.js" integrity="sha512-POVwgPHOXVwnhRlyoI5kWg4C9tnDu9FvMUWUVt17i93pEkhgNHNd75fyN/Cp3rDm78kgirFE+tHs+1uPifZ3hg==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>

<script src="https://cdnjs.cloudflare.com/ajax/libs/firebase/8.10.0/firebase-app.js" integrity="sha512-BGba5na4KpjxEWMOuUzaJ5esHUMfU/qotd2zv5sugqedOx3+oHFaeieOzFQs3COa2sq6BAksRirtAFztryVZFA==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/firebase/8.10.0/firebase-database.min.js" integrity="sha512-XDKFSZOhFNmwmx69Xr0j3zmePQ3NoSgpzZPr49P6oV7ME5ZhEXUqu+KUA0vQtof87P6IX+ycg4PmSms/EF8/pw==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
-->

<script src="https://www.gstatic.com/firebasejs/8.10.0/firebase-app.js"></script>
<script src="https://www.gstatic.com/firebasejs/8.10.0/firebase-database.js"></script>

<script src="https://jpedro.github.io/js/v1/data.js"></script>
<script src="https://jpedro.github.io/js/v1/comments.js"></script>
<script defer>Comments.mount(document.body.children[0]);</script>
"""

TEMPLATE_INDEX = """
# Index

{{ pages }}

""" + TEMPLATE_COMMENTS

# #### Tags
# {{ tags }}


TEMPLATE_TAG = """
## {{ name }}

{{ content }}

"""


class Index:

    def load(self, path: str) -> dict:
        lines = open(path, "r").readlines()
        attrs = {}

        for line in lines:
            line = line.strip()

            if line.find("# ") == 0:
                if attrs.get("title") is None:
                    print(f"==> Using first '#' header as title: {line}")
                    attrs["title"] = line[2:]
                else:
                    print(f"==> Skip. Using previous title: {attrs['title']}")
                continue

            start = line.find(COMMENT_START)
            end = line.find(COMMENT_END)
            if start < 0:
                continue
            if end < 0:
                continue

            # print(f"Found line: {line}")
            comment = line[len(COMMENT_START):len(
                line) - len(COMMENT_END)].strip()
            # print(f"  Comment: {comment}")

            pos = comment.find(":")
            if pos < 0:
                attrs[comment] = True
                continue

            key = comment[0:pos].strip()
            val = comment[pos+1:].strip()
            # print(f"  {key}: {val}")
            attrs[key] = val

        print(f"==> Found attrs in file {path}: {attrs}")
        return attrs


    def titlelize(self, text: str) -> str:
        return text[0].upper() + text[1:]


    def process(self, save: bool):
        blogPages = {}
        blogTags = {}

        for path in os.listdir():
            ext = path[len(path)-3:]
            if ext != ".md":
                continue
            if path in EXCLUDE:
                continue

            print(f"==> Found file {path}")
            attrs = self.load(path)
            if str(attrs.get("hidden")).lower() in TRUTHY:
                print(f"==> File {path} is hidden")
                continue

            if not attrs.get("title"):
                title = os.path.basename(path).replace(
                    ".md", "").replace("-", " ")
                attrs["title"] = self.titlelize(title)
                print(f"==> Using file {path} name as title: {title}")

            if attrs.get("tags"):
                for tag in attrs["tags"].split(","):
                    tag = tag.strip()
                    if blogTags.get(tag) is None:
                        blogTags[tag] = {}
                    blogTags[tag][path] = attrs["title"]

            blogPages[path] = attrs["title"]

        print(f"==> Using pages {blogPages}")
        print(f"==> Using tags {blogTags}")

        separator = "─" * 80
        tagItems = {}
        os.makedirs(f"tags", exist_ok=True)
        for tag, pages in blogTags.items():
            tagItems[tag] = []
            content = []
            # print(f"\nTag: {titlelize(tag)}")
            for page, title in pages.items():
                tagItems[tag].append(page)
                # print(f"- Page: {title}: {page}")
                # content.append(f"- [{title}](../{page[0:len(page)-3]})")
                content.append(f"- [{title}](../{page})")
                # content.append(f"- [{title}](/{page})")

            text = TEMPLATE_TAG.replace("{{ name }}", self.titlelize(tag))
            text = text.replace("{{ content }}", "\n".join(content)).strip()
            print()
            print(f"File \033[32;1mtags/{tag}.md")
            print("\033[38;5;242m" + separator)
            print(text)
            print(separator + "\033[0m")
            if save:
                os.makedirs(f"tags", exist_ok=True)
                with open(f"tags/{tag}.md", "w") as f:
                    f.write(text)

        content = []
        for name, title in blogPages.items():
            # print(f"- {title}: {name}")
            # content.append(f"- [{title}]({name})")
            # content.append(f"- [{title}]({name[0:len(name)-3]})")
            content.append(f"- [{title}]({name})")

        tagsContent = []
        # print(tagItems)
        for tag, pages in tagItems.items():
            count = len(pages)
            # print(f"- Tag: {tag}: {count}")
            tagsContent.append(f"- [{tag}](tags/{tag}) ({count})")

        # print("tagsContent", tagsContent)
        text = TEMPLATE_INDEX.replace("{{ pages }}", "\n".join(content))
        text = text.replace("{{ tags }}", "\n".join(tagsContent)).strip()
        print()
        print("File \033[32;1mindex.md")
        print("\033[38;5;242m" + separator)
        print(text)
        print(separator + "\033[0m")
        if save:
            with open("index.md", "w") as f:
                f.write(text)


if __name__ == "__main__":
    index = Index()
    save = True if "--save" in sys.argv else False
    index.process(save)
