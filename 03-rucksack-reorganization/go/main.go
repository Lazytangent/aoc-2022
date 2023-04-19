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
	outer:
	for _, rucksack := range rucksacks {
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
				continue outer
			}
		}
	}

	fmt.Printf("Part One: %d\n", sum)
}
