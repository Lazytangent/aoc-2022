import enum
from typing import Optional


class NodeType(str, enum.Enum):
    DIR = "dir"
    FILE = "file"


class TreeNode:
    def __init__(self, name: str, type: NodeType, parent: Optional["TreeNode"] = None, size: int = 0):
        self.name = name
        self.type = type
        self.parent = parent
        self.size = size


class FileTree:
    def __init__(self):
        self.root = None


def parse():
    file = "../data/small.txt"
    # file = "../data/full.txt"

    with open(file) as f:
        read_data = f.read()

    read_data = read_data.strip()
    contents = read_data.split('\n')

    print(f"{contents}")


if __name__ == "__main__":
    parse()
