package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"

	"lazytangent/aoc/2022/utils"
)

const TEST_DATA = "../data/small.txt"
const FULL_DATA = "../data/full.txt"

func main() {
	input, err := utils.ParseInput(os.Args)
	if err != nil {
		log.Fatal(err)
	}

	input = strings.TrimSpace(input)
	elves := strings.Split(input, "\n\n")

	max := 0
	second, third := 0, 0
	for _, elf := range elves {
		curr := 0
		for _, item := range strings.Split(elf, "\n") {
			val, _ := strconv.Atoi(item)
			curr += val
		}

		if curr > max {
			third = second
			second = max
			max = curr
		} else if curr > second {
			third = second
			second = curr
		} else if curr > third {
			third = curr
		}
	}

	fmt.Printf("First solution: %d\n", max)
	fmt.Printf("Second solution: %d\n", max + second + third)
}

type dataType int

const (
	small dataType = 0
	full dataType = 1
)

func getData(type_ dataType) (string, error) {
	var filename string

	switch (type_) {
	case small:
		filename = TEST_DATA
	case full:
		filename = FULL_DATA
	}

	data, err := os.ReadFile(filename)
	if err != nil {
		return "", err
	}

	return string(data), nil
}

func parseDataType(args []string) dataType {
	type_ := small
	if len(os.Args) >= 2 {
		if os.Args[1] == "full" {
			type_ = full
		}
	}

	return type_
}
