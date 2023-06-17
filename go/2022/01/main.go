package main

import (
  "io/ioutil"
  "sort"
  "strconv"
  "strings"
)

func process(str string) (ex_01, ex_02 int) {
  content := string(str)
  paragraphs := strings.Split(content, "\n\n")

  numbers := make([]int, len(paragraphs))
  for i, paragraph := range paragraphs {
    lines := strings.Split(paragraph, "\n")
    elfTotal := 0
    for _, line := range lines {
      num, _ := strconv.ParseInt(line, 10, 32)
      elfTotal += int(num)
    }
    numbers[i] = elfTotal
  }

  sort.Slice(numbers, func(i, j int) bool {
    return numbers[i] > numbers[j]
  })

  ex_02_result := 0
  for i := 0; i < 3; i++ {
    ex_02_result += numbers[i]
  }

  return numbers[0], ex_02_result
}

func main() {
  raw, _ := ioutil.ReadFile("input.txt")
  content := string(raw)
  ex1, ex2 := process(content)
  println(ex1, ex2)
}
