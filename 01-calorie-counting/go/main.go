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
