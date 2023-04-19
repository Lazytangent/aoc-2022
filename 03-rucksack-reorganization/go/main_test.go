package main

import "testing"

func TestSmallInput(t *testing.T) {
	input := `vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
`

	partOne, partTwo := day03(input)
	wantOne, wantTwo := 157, 70

	if wantOne != partOne {
		t.Fatalf(`day03(input) = %q, want %q`, partOne, wantOne)
	}
	if wantTwo != partTwo {
		t.Fatalf(`day03(input) = %q, want %q`, partTwo, wantTwo)
	}
}
