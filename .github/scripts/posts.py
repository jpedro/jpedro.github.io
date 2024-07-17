#!/usr/bin/env python3
import os
import sys

# Hmm.... https://www.irishpost.com/life-style/infamous-no-irish-no-blacks-no-dogs-signs-may-never-have-existed-racist-xenophobic-148416
NO_IRISH_NEED_APPLY = [
    "README.md",
    "index.md",
]

TRUE_FROM_A_CERTAIN_POINT_OF_VIEW = [
    "true",
    "yes",
    "on",
    "1",
]
THESE_ARE_NOT_THE_DROIDS = "<!--"
TOYDARIAN_WATTO = "-->"

CENSORED_SEXY_COMMENTS = """
 &nbsp;

<script src="https://www.gstatic.com/firebasejs/8.10.0/firebase-app.js"></script>
<script src="https://www.gstatic.com/firebasejs/8.10.0/firebase-database.js"></script>
<script src="https://jpedro.github.io/js/v1/data.js"></script>
<script src="https://jpedro.github.io/js/v1/comments.js"></script>
<script defer="">Comments.mount(document.body.children[0]);</script>
"""

COOL_SEXY_COMMENTS = """
 &nbsp;

<script src="https://raw.githubusercontent.com/jpedro/jpedro.github.io/master/.github/static/js/comments.js"></script>
<script defer="">Comments.start(document.body.children[0]);</script>
"""

SHOW_ME_SOME_MCLOVIN = """
<br />
<br />
<!-- #  &nbsp; -->

<!-- Made with some <3 [Not a lot](https://github.com/jpedro/jpedro.github.io) -->
<!-- Posts nobody asked for <font color="red" size="1px">■</font> -->
[Made](https://github.com/jpedro/jpedro.github.io) with some <3 not a lot
"""

TEMPLATE_INDEX = """
<!-- Yeah... this is not great but here we go -->
<!-- Google tag (gtag.js) -->
<script async src="https://www.googletagmanager.com/gtag/js?id=G-1ZEKDKLNJG"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());

  gtag('config', 'G-1ZEKDKLNJG');
</script>

# Index
<!-- # Posts nobody asked for -->

<!-- # Index -->
<!--  &nbsp; -->

{{ pages }}

""" # + TEMPLATE_COMMENTS

# #### Tags
# {{ tags }}


TEMPLATE_TAG = """
<!-- Yeah... this is not great but here we go -->
<!-- Google tag (gtag.js) -->
<script async src="https://www.googletagmanager.com/gtag/js?id=G-1ZEKDKLNJG"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());

  gtag('config', 'G-1ZEKDKLNJG');
</script>

## {{ name }}

{{ content }}

"""

class Colors:

    @staticmethod
    def green(text: str) -> str:
        return f"\033[32;1m{text}\033[0m"

    @staticmethod
    def yellow(text: str) -> str:
        return f"\033[33;1m{text}\033[0m"

    @staticmethod
    def gray(text: str) -> str:
        return f"\033[2m{text}\033[0m"




class Posts:

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

            start = line.find(THESE_ARE_NOT_THE_DROIDS)
            end = line.find(TOYDARIAN_WATTO)
            if start < 0:
                continue
            if end < 0:
                continue

            # print(f"Found line: {line}")
            comment = line[len(THESE_ARE_NOT_THE_DROIDS):len(
                line) - len(TOYDARIAN_WATTO)].strip()
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


    def replaceFooter(self, path: str, comments: bool):
        with open(path, "r") as f:
            text = f.read()

        startFooter = "<!-- START FOOTER -->"
        endFooter = "<!-- END FOOTER -->"
        posStart = text.find(startFooter)
        posEnd = text.find(endFooter)
        print(f"posStart: {posStart}")
        print(f"posEnd: {posEnd}")
        print(f"lenEnd: {len(endFooter)}")

        if posStart < 0:
            print(f"Failed to find {startFooter}")
            before = text
            after = ""

        elif posEnd < 0:
            print(f"Failed to find {endFooter}")
            before = text
            after = ""

        else:
            cutRest = posEnd + len(endFooter)
            before = text[0:posStart]
            after = text[cutRest:]


        print(f"comments: {comments}")
        if comments == "false":
            text = f"{before}{after}"
            print(Colors.green("Not adding comments"))
        else:
            middle = f"{startFooter}{COOL_SEXY_COMMENTS}{endFooter}"
            text = f"{before}{middle}{after}"

        with open(path, "w") as f:
            f.write(text)


    def replaceTags(self, path: str, tags: list):
        with open(path, "r") as f:
            text = f.read()

        startTags = "<!-- START TAGS -->"
        endTags = "<!-- END TAGS -->"
        posStart = text.find(startTags)
        posEnd = text.find(endTags)
        print(f"posStart: {posStart}")
        print(f"posEnd: {posEnd}")
        print(f"lenEnd: {len(endTags)}")

        if posStart < 0:
            print(f"Failed to find {startTags}")
            return

        if posEnd < 0:
            print(f"Failed to find {endTags}")
            return

        cutRest = posEnd + len(endTags)
        print(f"Found start: {posStart}, end: {posEnd}, cut: {cutRest}")
        before = text[0:posStart]
        after = text[cutRest:]
        badges = []
        badges.append(startTags)
        for tag in tags:
            clean = tag.replace(" ", "-")
            badges.append(
                f'[<img src="https://img.shields.io/badge/Tag-{tag}-brightgreen">](/tags/{clean})'
            )
        badges.append(endTags)
        middle = "\n".join(badges)

        text = f"{before}{middle}{after}"
        with open(path, "w") as f:
            f.write(text)


    def process(self, dir: str, save: bool):
        posts = {}
        uniqueTags = {}

        for name in os.listdir(dir):
            ext = name[len(name)-3:]
            if ext != ".md":
                print(f"==> Skip file {Colors.gray(name)}")
                continue
            if name in NO_IRISH_NEED_APPLY:
                print(f"==> No irish please {Colors.gray(name)}")
                continue

            path = f"{dir}/{name}"
            print(f"==> Found file {Colors.green(path)}")
            # if path == "dispatch.md":

            attrs = self.load(path)
            if str(attrs.get("hidden")).lower() in TRUE_FROM_A_CERTAIN_POINT_OF_VIEW:
                print(f"==> File {Colors.yellow(path)} is hidden")
                continue

            if not attrs.get("title"):
                title = os.path.basename(path).replace(".md", "").replace("-", " ")
                attrs["title"] = self.titlelize(title)
                print(f"==> Using file {Colors.yellow(path)} name as title: {title}")
            else:
                print(f"==> In file {Colors.green(path)} found title: {attrs.get('title')}")

            tags = attrs.get("tags")
            pageTags = []
            if tags:
                for tag in attrs["tags"].split(","):
                    tag = tag.strip()
                    if uniqueTags.get(tag) is None:
                        uniqueTags[tag] = {}
                    pageTags.append(tag)
                    uniqueTags[tag][path] = attrs["title"]

            print(f"==> Found page tags: {pageTags}")

        #     self.replaceTags(path, pageTags)
        #     self.replaceFooter(path, attrs.get("comments", "true"))

        #     posts[path] = attrs["title"]

        print(f"==> Found posts: {posts}")
        print(f"==> Found tags:  {uniqueTags}")

        # separator = "─" * 80
        # tagItems = {}
        # os.makedirs(f"tags", exist_ok=True)
        # for tag, pages in uniqueTags.items():
        #     tagItems[tag] = []
        #     content = []
        #     clean = tag.replace(" ", "-")
        #     # print(f"\nTag: {titlelize(tag)}")
        #     for page, title in pages.items():
        #         tagItems[tag].append(page)
        #         # print(f"- Page: {title}: {page}")
        #         # content.append(f"- [{title}](../{page[0:len(page)-3]})")
        #         content.append(f"- [{title}](../{page})")
        #         # content.append(f"- [{title}](/{page})")

        #     text = TEMPLATE_TAG.replace("{{ name }}", self.titlelize(tag))
        #     text = text.replace("{{ content }}", "\n".join(content)).strip()
        #     print()
        #     print(f"File tags/{Colors.green(tag)}.md")
        #     print("\033[38;5;242m" + separator)
        #     print(text)
        #     print(separator + "\033[0m")
        #     if save:
        #         os.makedirs(f"tags", exist_ok=True)
        #         with open(f"tags/{clean}.md", "w") as f:
        #             f.write(text)

        # content = []
        # for name, title in pages.items():
        #     # print(f"- {title}: {name}")
        #     # content.append(f"- [{title}]({name})")
        #     # content.append(f"- [{title}]({name[0:len(name)-3]})")
        #     content.append(f"- [{title}]({name})")

        # tagsContent = []
        # # print(tagItems)
        # for tag, pages in tagItems.items():
        #     count = len(pages)
        #     # print(f"- Tag: {tag}: {count}")
        #     tagsContent.append(f"- [{tag}](tags/{tag}) ({count})")

        # print("tagsContent", tagsContent)
        # text = TEMPLATE_INDEX.replace("{{ pages }}", "\n".join(content))
        # text = text.replace("{{ tags }}", "\n".join(tagsContent)).strip()
        # print()
        # print("File \033[32;1mindex.md")
        # print("\033[38;5;242m" + separator)
        # print(text)
        # print(separator + "\033[0m")
        # if save:
        #     with open("index.md", "w") as f:
        #         f.write(text)
        #         f.write(SHOW_ME_SOME_MCLOVIN)


if __name__ == "__main__":
    posts = Posts()
    posts.process(".posts", True)
