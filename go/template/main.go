package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	content := read_file()

	result := process(content)
	fmt.Println(result)
}

func process(data []string) int {
	return 0
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
		fileLines = append(fileLines, fileScanner.Text())
	}

	file.Close()
	return fileLines
}
