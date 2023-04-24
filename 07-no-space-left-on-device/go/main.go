package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"

	"lazytangent/aoc/2022/utils"
)

func main() {
	input, err := utils.ParseInput(os.Args)
	if err != nil {
		log.Fatal(err)
	}
	input = strings.TrimSpace(input)
	lines := strings.Split(input, "\n")

	parentDirs := []string{}
	dirSizes := make(map[string]int64)

	for _, line := range lines {
		l := strings.Split(line, " ")

		if l[0] == "$" {
			if l[1] == "cd" && l[2] != ".." {
				parentDirs = append(parentDirs, l[2])
				dirSizes[strings.Join(l, "/")] = 0
			} else if l[1] == "cd" && l[2] == ".." {
				parentDirs = parentDirs[:len(parentDirs) - 1]
			}
		} else if val, err := strconv.ParseInt(l[0], 10, 64); err == nil {
			tempParentDirs := make([]string, len(parentDirs))
			copy(tempParentDirs, parentDirs)

			for len(tempParentDirs) > 0 {
				dirSizes[strings.Join(tempParentDirs, "/")] += val
				tempParentDirs = tempParentDirs[:len(tempParentDirs) - 1]
			}
		}
	}

	// totalDiskSpace := 70_000_000
	// requiredFreeSpace := 30_000_000
	var partOne int64 = 0

	for _, v := range dirSizes {
		if v <= 100_000 {
			partOne += v
		}
	}

	fmt.Println("Part One:", partOne)
}
