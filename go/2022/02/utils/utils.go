package utils

import "fmt"

type Move rune

const (
	Rock     Move = 'A'
	Paper         = 'B'
	Scissors      = 'C'
)

type Result int

const (
	Win  Result = 6
	Draw        = 3
	Lose        = 0
)

func ParseMove(r rune) Move {
	switch r {
	case 'A', 'X':
		return Rock
	case 'B', 'Y':
		return Paper
	case 'C', 'Z':
		return Scissors
	default:
		fmt.Println(r)
		panic("Invalid rune: ")
	}
}

func getResultScore(theirMove, ourMove Move) int {
	var result Result = Draw
	if ourMove == theirMove {
		return int(Draw)
	}

	switch ourMove {
	case Rock:
		if theirMove == Scissors {
			result = Win
		} else {
			result = Lose
		}
	case Paper:
		if theirMove == Rock {
			result = Win
		} else {
			result = Lose
		}
	case Scissors:
		if theirMove == Paper {
			result = Win
		} else {
			result = Lose
		}
	}
	return int(result)
}

func GetRoundScore(theirMove, ourMove Move) int {
	score := 0

	switch ourMove {
	case Rock:
		score += 1 + getResultScore(theirMove, ourMove)

	case Paper:
		score += 2 + getResultScore(theirMove, ourMove)

	case Scissors:
		score += 3 + getResultScore(theirMove, ourMove)
	}

	return score
}
