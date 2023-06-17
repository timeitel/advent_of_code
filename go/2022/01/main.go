package main

import (
  "fmt"
  "io/ioutil"
  "sort"
  "strconv"
  "strings"
)

func main() {
  raw, _ := ioutil.ReadFile("input.txt")
  content := string(raw)
  paragraphs := strings.Split(content, "\n\n")

  numbers := make([]int, len(paragraphs))
  for _, paragraph := range paragraphs {
    lines := strings.Split(paragraph, "\n")
    for i, line := range lines {
      num, _ := strconv.ParseInt(line, 10, 32)
      numbers[i] = int(num)
    }
  }

  sort.Slice(numbers, func(i, j int) bool {
    return numbers[i] > numbers[j]
  })

  result := 0
  for i := 0; i < 3; i++ {
    result += numbers[i]
  }

  fmt.Println(result)
}