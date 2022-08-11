import os

EXCLUDE = [
    "README.md",
    "index.md",
]

COMMENT_START = "[//]: # ("


TEMPLATE = """
## Index

- [Markdown](markdown.md) An example of a Markdown file
- [Bash Dispatch](bash-dispatch.md) How to dispatch with bash


##  
This will be generated. Eventually
"""


def process(file: str) -> dict:
    lines = open(file, "r").readlines()

    meta = {}

    for line in lines:
        pos = line.find(COMMENT_START)
        if pos < 0:
            continue

        print(f"Found line: {line}")
        comment = line[len(COMMENT_START):len(line) - 2].strip()

        pos = comment.find(":")
        if pos < 0:
            continue
        key = comment[0:pos].strip()
        val = comment[pos+1:].strip()
        print(f"Found --> {key}: {val}")
        meta[key] = val

    print(meta)
    return {}

def main():
    for file in os.listdir():
        if file.find(".md") < 0:
            continue
        if file in EXCLUDE:
            continue

        print(f"- {file}")
        process(file)


main()
