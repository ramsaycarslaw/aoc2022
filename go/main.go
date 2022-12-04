package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

// readFile reads a file and returns a slice of strings, one for each line
func readFile(path string) ([]string, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}

func day2a(input []string) {
	score := 0
	for _, line := range input {
		x := strings.Split(line, " ")
		switch x[1] {
		case "X":
			score += 1

			// calculate h2h score
			switch x[0] {
			case "A":
				score += 3
			case "C":
				score += 6
			}

		case "Y":
			score += 2

			switch x[0] {
			case "A":
				score += 6
			case "B":
				score += 3
			}

		case "Z":
			score += 3

			switch x[0] {
			case "B":
				score += 6
			case "C":
				score += 3
			}

		default:
			log.Fatal("Invalid input")
		}
	}
	fmt.Println("day 2 part a: ", score)
}

func day2b(input []string) {
	score := 0

	for _, line := range input {
		x := strings.Split(line, " ")

		switch x[0] {
		case "A":
			switch x[1] {
			case "X":
				score += 0
				score += 3
			case "Y":
				score += 3
				score += 1
			case "Z":
				score += 6
				score += 2
			}
		case "B":
			switch x[1] {
			case "X":
				score += 0
				score += 1
			case "Y":
				score += 3
				score += 2
			case "Z":
				score += 6
				score += 3
			}
		case "C":
			switch x[1] {
			case "X":
				score += 0
				score += 2
			case "Y":
				score += 3
				score += 3
			case "Z":
				score += 6
				score += 1
			}
		}
	}
	fmt.Println("day 2 part b: ", score)
}

func main() {
	day2Input, err := readFile("input/day2")
	if err != nil {
		panic(err)
	}

	day2a(day2Input)
	day2b(day2Input)
}
