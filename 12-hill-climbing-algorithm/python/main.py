import typing


def char_to_ht(ht_char: str) -> int:
    return ord(ht_char) - ord("a")


def make_map(map_txt: typing.IO) -> dict[tuple[int, int], str]:
    map = {}

    for (r, row) in enumerate(map_txt):
        for (c, ht_char) in enumerate(row):
            if ht_char == "S":
                map["start"] = (c, r)
                map[(c, r)] = char_to_ht("a")
            elif ht_char == "E":
                map["end"] = (c, r)
                map[(c, r)] = char_to_ht("z")
            else:
                map[(c, r)] = char_to_ht(ht_char)

    return map


OFFSETS = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
]


def move(location: tuple[int, int], offset: tuple[int, int]) -> tuple[int, int]:
    return tuple(map(sum, zip(location, offset)))


def next_to(
    point: tuple[int, int], map: dict[tuple[int, int], str]
) -> list[tuple[int, int]]:
    return [move(point, offset) for offset in OFFSETS if move(point, offset) in map]


def find_distances(map: dict[tuple[int, int], str]) -> dict[tuple[int, int], int]:
    distances = {map["start"]: 0}
    seen = [map["start"]]

    for point in seen:
        steps = distances[point]

        for next_point in next_to(point, map):
            if next_point not in distances or distances[next_point] > (steps + 1):
                if map[next_point] - map[point] <= 1:
                    distances[next_point] = steps + 1
                    seen.append(next_point)

    return distances


def find_distances_back(map: dict[tuple[int, int], str]):
    distances = {map["end"]: 0}
    seen = [map["end"]]

    for point in seen:
        steps = distances[point]

        for next_point in next_to(point, map):
            if not (next_point in distances) or distances[next_point] > (steps + 1):
                if map[point] - map[next_point] <= 1:
                    distances[next_point] = steps + 1
                    seen.append(next_point)

    return distances


FILE = "full.txt"
SMALL = "small.txt"


def main():
    with open(f"../data/{FILE}") as f:
        map = make_map(map_txt=f)

    part_one_solution = find_distances(map=map)
    part_two_solution = find_distances_back(map=map)

    print(f"Part one solution: {part_one_solution[map['end']]}")

    a_s = [c for c in map if map[c] == 0]
    distances = [part_two_solution[l] for l in a_s if l in part_two_solution]
    print(f"Part two solution: {min(distances)}")


if __name__ == "__main__":
    main()
