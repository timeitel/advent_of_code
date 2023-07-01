package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	content := read_file()

	result := process(content)
	fmt.Println(result)
}

func process(data []int) int {
	return 0
}

func read_file() []int {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	var fileLines []int

	for fileScanner.Scan() {
		text := fileScanner.Text()
		number, err := strconv.Atoi(text)
		if err != nil {
			log.Fatal(err)
		}

		fileLines = append(fileLines, number)
	}

	file.Close()
	return fileLines
}
