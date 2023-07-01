package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	content := read_file()

	process(content)
}

func process(data []string) {
	var bits = make([]int, len(data[0]))
	for _, n := range data {
		for i := range n {
			if n[i] == '1' {
				bits[i]++
			}
		}
	}

	g := 0b010100010100
	e := 0b101011101011

	fmt.Println(g * e)
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
		if err != nil {
			log.Fatal(err)
		}

		fileLines = append(fileLines, fileScanner.Text())
	}

	file.Close()
	return fileLines
}
