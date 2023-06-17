package main

import (
	"io/ioutil"
)

func process(str string) (ex_01, ex_02 int) {
}

func main() {
	raw, _ := ioutil.ReadFile("input.txt")
	content := string(raw)
	ex1, ex2 := process(content)
}
