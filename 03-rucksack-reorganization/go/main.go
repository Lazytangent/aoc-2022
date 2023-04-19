package main

import (
	"fmt"
	"log"
	"os"
	"strings"

	"lazytangent/aoc/2022/utils"
)

func main() {
	input, err := utils.ParseInput(os.Args)
	if err != nil {
		log.Fatal(err)
	}
	input = strings.TrimSpace(input)
	rucksacks := strings.Split(input, "\n")

	sum := 0
	partTwo := 0
	for idx, rucksack := range rucksacks {
		n := len(rucksack)
		halfIndex := n / 2
		firstHalf := rucksack[:halfIndex]
		secondHalf := rucksack[halfIndex:]

		for _, char := range firstHalf {
			if strings.ContainsRune(secondHalf, char) {
				c := int(char)
				if c >= int('A') && c <= int('Z') {
					sum += c - int('A') + 27
				} else if c >= int('a') && c <= int('z') {
					sum += c - int('a') + 1
				}
				break
			}
		}

		// Every three indices, check for duplicate across the previous two
		if (idx + 1) % 3 == 0 {
			for _, char := range rucksacks[idx] {
				if strings.ContainsRune(rucksacks[idx - 1], char) && strings.ContainsRune(rucksacks[idx - 2], char) {
					c := int(char)
					if c >= int('A') && c <= int('Z') {
						partTwo += c - int('A') + 27
					} else if c >= int('a') && c <= int('z') {
						partTwo += c - int('a') + 1
					}
					break
				}
			}
		}
	}

	fmt.Printf("Part One: %d\n", sum)
	fmt.Printf("Part Two: %d\n", partTwo)
}
