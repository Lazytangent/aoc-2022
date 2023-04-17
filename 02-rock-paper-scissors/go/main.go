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

	score := 0
	win, tie, loss := 6, 3, 0
	rock, paper, scissors := 1, 2, 3
	for _, round := range strings.Split(input, "\n") {
		shapes := strings.Split(round, " ")

		switch shapes[0] {
		case "A":
			switch shapes[1] {
			case "X":
				score += tie
			case "Y":
				score += loss
			case "Z":
				score += win
			}
			score += rock
		case "B":
			switch shapes[1] {
			case "X":
				score += win
			case "Y":
				score += tie
			case "Z":
				score += loss
			}
			score += paper
		case "C":
			switch shapes[1] {
			case "X":
				score += loss
			case "Y":
				score += win
			case "Z":
				score += tie
			}
			score += scissors
		}
	}

	fmt.Println("Part one:", score)
}
