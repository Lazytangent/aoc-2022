def parse():
    # file = "../data/small.txt"
    file = "../data/full.txt"

    with open(file) as f:
        read_data = f.read()

    read_data = read_data.strip()
    contents = read_data.splitlines()

    parent_dirs = []
    dir_sizes = {}

    for line in contents:
        line = line.split(' ')

        if line[0] == "$":
            if line[1] == "cd" and line[2] != '..':
                parent_dirs.append(line[2])
                dir_sizes['/'.join(parent_dirs)] = 0
            elif line[1] == "cd" and line[2] == "..":
                parent_dirs.pop()
        elif line[0].isnumeric():
            temp_parent_dirs = parent_dirs.copy()
            while len(temp_parent_dirs) > 0:
                dir_sizes["/".join(temp_parent_dirs)] += int(line[0])
                temp_parent_dirs.pop()

    total_disk_space = 70_000_000
    required_free_space = 30_000_000

    # print(f"Size of all directories: {dir_sizes}")
    small_dirs = { k: v for (k, v) in dir_sizes.items() if v <= 100_000 }
    print(f"Part one solution: {sum(small_dirs.values())}")

    current_free_space = total_disk_space - dir_sizes['/']
    # print(f"Current free space: {current_free_space}")

    required_change = required_free_space - current_free_space
    # print(f"Required change: {required_change}")

    eligible_dirs = { k: v for (k, v) in dir_sizes.items() if v >= required_change }
    # print(f"Eligible dirs: {eligible_dirs}")

    smallest_eligible_dir = "/"

    for (dir, size) in eligible_dirs.items():
        if size < eligible_dirs[smallest_eligible_dir]:
            smallest_eligible_dir = dir

    print(f"Part two solution: {eligible_dirs[smallest_eligible_dir]}")


if __name__ == "__main__":
    parse()
