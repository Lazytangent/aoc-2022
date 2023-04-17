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

		switch shapes[1] {
		case "X": // Rock
			switch shapes[0] {
			case "A": // Rock
				score += tie
			case "B": // Paper
				score += loss
			case "C": // Scissors
				score += win
			}
			score += rock
		case "Y": // Paper
			switch shapes[0] {
			case "A": // Rock
				score += win
			case "B": // Paper
				score += tie
			case "C": // Scissors
				score += loss
			}
			score += paper
		case "Z": // Scissors
			switch shapes[0] {
			case "A": // Rock
				score += loss
			case "B": // Paper
				score += win
			case "C": // Scissors
				score += tie
			}
			score += scissors
		}
	}

	fmt.Println("Part one:", score)
}
