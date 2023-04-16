package utils

import (
	"os"
	"strings"
)

const testData = "../data/small.txt"
const fullData = "../data/full.txt"

type dataType int

const (
	small dataType = 0
	full dataType = 1
)

func GetData(t dataType) (string, error) {
	var filename string

	switch t {
	case small:
		filename = testData
	case full:
		filename = fullData
	}

	data, err := os.ReadFile(filename)
	if err != nil {
		return "", err
	}

	return string(data), nil
}

func ParseDataType(args []string) dataType {
	t := small
	if len(os.Args) >= 2 {
		if strings.ToLower(os.Args[1]) == "full" {
			t = full
		}
	}

	return t
}

func ParseInput(args []string) (string, error) {
	t := ParseDataType(args)
	return GetData(t)
}
