import os
import sys

EXCLUDE = [
    "README.md",
    "index.md",
]

COMMENT_START = "<!--"
COMMENT_END = "-->"

TEMPLATE_INDEX = """
## Pages

{{ pages }}


#### Tags
{{ tags }}


##  
This will be generated. Eventually
"""

TEMPLATE_TAG = """
## {{ name }}

{{ content }}

"""


def process(name: str) -> dict:
    lines = open(name, "r").readlines()
    meta = {}

    for line in lines:
        line = line.strip()

        if line.find("# ") == 0 and meta.get("title") is None:
            # print(f"Using first # header as title: {line}")
            meta["title"] = line[2:]
            continue

        start = line.find(COMMENT_START)
        end = line.find(COMMENT_END)
        if start < 0:
            continue
        if end < 0:
            continue

        # print(f"Found line: {line}")
        comment = line[len(COMMENT_START):len(line) - len(COMMENT_END)].strip()
        # print(f"  Comment: {comment}")

        pos = comment.find(":")
        if pos < 0:
            meta[comment] = True
            continue

        key = comment[0:pos].strip()
        val = comment[pos+1:].strip()
        # print(f"  {key}: {val}")
        meta[key] = val

    # print("meta", meta)
    return meta

def titlelize(text: str) -> str:
    return text[0].upper() + text[1:]


def main(save: bool):
    blogPages = {}
    blogTags = {}

    for name in os.listdir():
        ext = name[len(name)-3:]
        if ext != ".md":
            continue
        if name in EXCLUDE:
            continue

        # print(f"- File {name}")
        meta = process(name)
        if meta.get("hidden"):
            # print(f"  - {name} is hidden")
            continue

        if meta.get("title") is None:
            title = os.path.basename(name).replace(".md", "").replace("-", " ")
            meta["title"] = titlelize(title)
            # print(f"  - Using name name as title: {title}")

        if meta.get("tags"):
            for tag in meta["tags"].split(","):
                tag = tag.strip()
                if blogTags.get(tag) is None:
                    blogTags[tag] = {}
                blogTags[tag][name] = meta["title"]

        blogPages[name] = meta["title"]

    # print("Using pages", pages)
    # print("Using tags", tags)

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
            # content.append(f"- [{title}](/{page[0:len(page)-3]})")
            content.append(f"- [{title}](/{page})")

        text = TEMPLATE_TAG.replace("{{ name }}", titlelize(tag))
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

save = True if "--save" in sys.argv else False
main(save)
