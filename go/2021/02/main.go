package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	content := read_file()

	result := process(content)
	fmt.Println(result)
}

func process(data []string) int {
	var ins []Instruction

	for _, rawIns := range data {
		split := strings.Split(rawIns, " ")
		dir := split[0]
		val, _ := strconv.Atoi(split[1])
		ins = append(ins, Instruction{dir, val})
	}

	h := 0
	d := 0
	aim := 0

	for _, instruction := range ins {
		switch instruction.dir {
		case "forward":
			h += instruction.val
			d += (instruction.val * aim)
		case "up":
			aim -= instruction.val
		case "down":
			aim += instruction.val
		default:
			log.Fatal(instruction)
		}
	}

	return h * d
}

type Instruction struct {
	dir string
	val int
}

func read_file() []string {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	var fileLines []string

	for fileScanner.Scan() {
		text := fileScanner.Text()
		fileLines = append(fileLines, text)
	}

	file.Close()
	return fileLines
}
