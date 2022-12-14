import argparse
import typing


class Grid:
    grid: dict[tuple[int, int], str]
    xmin: int
    xmax: int
    ymin: int
    ymax: int
    floor: int

    def __init__(self):
        self.grid = {}
        self.xmin = 500
        self.xmax = 500
        self.ymin = 0
        self.ymax = 0
        self.floor = 0

    def put(self, x: int, y: int, char: str):
        if x > self.xmax:
            self.xmax = x
        if x < self.xmin:
            self.xmin = x
        if y > self.ymax:
            self.ymax = y
        if y < self.ymin:
            ymin = y
        self.grid[(x, y)] = char

    def show(self):
        for y in range(self.ymin, self.ymax + 1):
            line = ""
            for x in range(self.xmin, self.xmax + 1):
                if (x, y) not in self.grid:
                    self.put(x, y, ('.', '#')[y >= self.floor])
                line += self.grid[(x, y)]
            print(line)

    def drop(self, x: int, y: int, cap: int) -> bool:
        while y < cap:
            if (x-1, y+1) not in self.grid:
                self.put(x-1, y+1, ('.', '#')[y+1 >= self.floor])
            if (x, y+1) not in self.grid:
                self.put(x, y+1, ('.', '#')[y+1 >= self.floor])
            if (x+1, y+1) not in self.grid:
                self.put(x+1, y+1, ('.', '#')[y+1 >= self.floor])
            if self.grid[(x, y+1)] == '.':
                y += 1
            else:
                if self.grid[(x-1, y+1)] == '.':
                    x -= 1
                    y += 1
                elif self.grid[(x+1, y+1)] == '.':
                    x += 1
                    y += 1
                else:
                    self.put(x, y, 'o')
                    return True
        return False

    def read_file(self, file: typing.IO):
        for line in file:
            pairs = line.rstrip().split(' -> ')
            first = True
            for pair in pairs:
                x, y = list(map(int, pair.split(',')))
                if first:
                    px, py = x, y
                    first = False
                    continue
                if px == x:
                    if py < y:
                        for yy in range(py, y+1):
                            self.put(x, yy, '#')
                    else:
                        for yy in range(y, py+1):
                            self.put(x, yy, '#')
                elif py == y:
                    if px < x:
                        for xx in range(px, x+1):
                            self.put(xx, y, '#')
                    else:
                        for xx in range(x, px+1):
                            self.put(xx, y, '#')

                px, py = x, y

        self.floor = self.ymax + 2


FULL = "full.txt"
SMALL = "small.txt"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-r', '--real', action='store_true')

    args = parser.parse_args()

    grid = Grid()
    sand = 0
    grid.put(500, 0, '.')

    if args.real:
        file = FULL
    else:
        file = SMALL

    with open(f"../data/{file}") as f:
        grid.read_file(file=f)

    while grid.drop(500, 0, grid.ymax):
        sand += 1

    print(f"Part one solution: {sand}")

    while grid.grid[(500, 0)] == '.':
        grid.drop(500, 0, grid.floor)
        sand += 1

    print(f"Part two solution: {sand}")


if __name__ == "__main__":
    main()
