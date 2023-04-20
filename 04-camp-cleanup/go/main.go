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
	elves := strings.Split(input, "\n")

	count := 0
	for _, elf := range elves {
		split := strings.Split(elf, ",")
		left, right := split[0], split[1]

		if (leftContainsRight(left, right) || rightContainsLeft(left, right)) {
			count++
		}
	}

	fmt.Println("Part One:", count)
}

func leftContainsRight(left, right string) bool {
	leftRange := strings.Split(left, "-")
	rightRange := strings.Split(right, "-")
	leftStart, _ := strconv.Atoi(leftRange[0])
	leftEnd, _ := strconv.Atoi(leftRange[1])
	rightStart, _ := strconv.Atoi(rightRange[0])
	rightEnd, _ := strconv.Atoi(rightRange[1])

	return leftStart <= rightStart && leftEnd >= rightEnd
}

func rightContainsLeft(left, right string) bool {
	leftRange := strings.Split(left, "-")
	rightRange := strings.Split(right, "-")
	leftStart, _ := strconv.Atoi(leftRange[0])
	leftEnd, _ := strconv.Atoi(leftRange[1])
	rightStart, _ := strconv.Atoi(rightRange[0])
	rightEnd, _ := strconv.Atoi(rightRange[1])

	return rightStart <= leftStart && rightEnd >= leftEnd
}
