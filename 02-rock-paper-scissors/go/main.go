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
	partTwo := 0
	win, tie, loss := 6, 3, 0
	rock, paper, scissors := 1, 2, 3
	for _, round := range strings.Split(input, "\n") {
		shapes := strings.Split(round, " ")

		switch shapes[1] {
		case "X": // Rock // Loss
			switch shapes[0] {
			case "A": // Rock
				score += tie
				partTwo += scissors
			case "B": // Paper
				score += loss
				partTwo += rock
			case "C": // Scissors
				score += win
				partTwo += paper
			}
			score += rock
			partTwo += loss
		case "Y": // Paper // Tie
			switch shapes[0] {
			case "A": // Rock
				score += win
				partTwo += rock
			case "B": // Paper
				score += tie
				partTwo += paper
			case "C": // Scissors
				score += loss
				partTwo += scissors
			}
			score += paper
			partTwo += tie
		case "Z": // Scissors // Win
			switch shapes[0] {
			case "A": // Rock
				score += loss
				partTwo += paper
			case "B": // Paper
				score += win
				partTwo += scissors
			case "C": // Scissors
				score += tie
				partTwo += rock
			}
			score += scissors
			partTwo += win
		}
	}

	fmt.Println("Part one:", score)
	fmt.Println("Part two:", partTwo)
}
