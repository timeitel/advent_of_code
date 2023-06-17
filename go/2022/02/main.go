package main

import (
	"02/utils"
	"fmt"
	"io/ioutil"
	"strings"
)

func process(str string) int {
	lines := strings.Split(str, "\n")

	totalScore := 0
	for _, line := range lines {
		moves := strings.Split(line, " ")
		if len(moves) != 2 {
			continue
		}

		theirs := utils.ParseMove([]rune(moves[0])[0])
		ours := utils.GetRequiredMove(theirs, []rune(moves[1])[0])
		totalScore += utils.GetRoundScore(theirs, ours)
	}

	return totalScore
}

func main() {
	raw, _ := ioutil.ReadFile("input.txt")
	content := string(raw)
	result := process(content)
	fmt.Println(result)
}
