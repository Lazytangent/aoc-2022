package main

import (
	"fmt"
	"lazytangent/aoc/2022/utils"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	input, err := utils.ParseInput(os.Args)
	if err != nil {
		log.Fatal(err)
	}
	input = strings.TrimSpace(input)
	elves := strings.Split(input, "\n")

	count := 0
	partTwo := 0
	for _, elf := range elves {
		split := strings.Split(elf, ",")
		left, right := split[0], split[1]

		if leftContainsRight(left, right) || rightContainsLeft(left, right) {
			count++
		}

		if anyOverlap(left, right) {
			partTwo++
		}
	}

	fmt.Println("Part One:", count)
	fmt.Println("Part Two:", partTwo)
}

func leftContainsRight(left, right string) bool {
	leftRange, rightRange := getRanges(left, right)

	return leftRange.Start <= rightRange.Start && leftRange.End >= rightRange.End
}

func rightContainsLeft(left, right string) bool {
	leftRange, rightRange := getRanges(left, right)

	return rightRange.Start <= leftRange.Start && rightRange.End >= leftRange.End
}

func anyOverlap(left, right string) bool {
	leftRange, rightRange := getRanges(left, right)

	return (leftRange.Start >= rightRange.Start && leftRange.Start <= rightRange.End) ||
		(leftRange.End >= rightRange.Start && leftRange.End <= rightRange.End) ||
		(rightRange.Start >= leftRange.Start && rightRange.Start <= leftRange.End) ||
		(rightRange.End >= leftRange.Start && rightRange.End <= leftRange.End)
}

type numRange struct {
	Start, End int
}

func getRanges(left, right string) (numRange, numRange) {
	leftRange := strings.Split(left, "-")
	rightRange := strings.Split(right, "-")
	leftStart, _ := strconv.Atoi(leftRange[0])
	leftEnd, _ := strconv.Atoi(leftRange[1])
	rightStart, _ := strconv.Atoi(rightRange[0])
	rightEnd, _ := strconv.Atoi(rightRange[1])

	return numRange{Start: leftStart, End: leftEnd}, numRange{Start: rightStart, End: rightEnd}
}
