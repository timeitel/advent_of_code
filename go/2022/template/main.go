package main

import (
	"fmt"
	"io/ioutil"
)

func process(str string) int {

	return 0
}

func main() {
	raw, _ := ioutil.ReadFile("input.txt")
	content := string(raw)
	result := process(content)
	fmt.Println(result)
}
