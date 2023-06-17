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

func GetRequiredMove(theirMove Move, requiredOutcome rune) Move {
	switch theirMove {
	case Rock:
		switch requiredOutcome {
		case 'X':
			return Scissors
		case 'Y':
			return Rock
		case 'Z':
			return Paper
		}

	case Paper:
		switch requiredOutcome {
		case 'X':
			return Rock
		case 'Y':
			return Paper
		case 'Z':
			return Scissors
		}

	case Scissors:
		switch requiredOutcome {
		case 'X':
			return Paper
		case 'Y':
			return Scissors
		case 'Z':
			return Rock
		}

		return Paper
	}

	panic("Missed outcome")
}

func ParseMove(r rune) Move {
	switch r {
	case 'A':
		return Rock
	case 'B':
		return Paper
	case 'C':
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
