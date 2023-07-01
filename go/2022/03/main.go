package main

import (
	"fmt"
	"io/ioutil"
	"os"
)

func process(str string) int {

	return 0
}

func main() {
	raw, _ := ioutil.ReadFile("input.txt")
	content := string(raw)

	content, _ = os.Open("input.txt")

	result := process(content)
	fmt.Println(result)
}
