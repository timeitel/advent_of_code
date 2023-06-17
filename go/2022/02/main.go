package main

import (
	"02/utils"
	"fmt"
	"io/ioutil"
	"strings"
)

func process(str string) (ex_01, ex_02 int) {
	lines := strings.Split(str, "\n")

	totalScore := 0
	for _, line := range lines {
		moves := strings.Split(line, " ")
		if len(moves) != 2 {
			continue
		}

		theirs := utils.ParseMove([]rune(moves[0])[0])
		ours := utils.ParseMove([]rune(moves[1])[0])
		totalScore += utils.GetRoundScore(theirs, ours)
	}

	return totalScore, 1
}

func main() {
	raw, _ := ioutil.ReadFile("input.txt")
	content := string(raw)
	ex1, _ := process(content)
	fmt.Println(ex1)
}
