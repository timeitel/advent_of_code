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
	// var bits = make([]int, len(data[0]))
	// for _, n := range data {
	// 	for i := range n {
	// 		if n[i] == '1' {
	// 			bits[i]++
	// 		}
	// 	}
	// }

	// g := 0b010100010100
	// e := 0b101011101011

	// fmt.Println(g * e)

	ns := make([]string, len(data))
	copy(ns, data)

	i := 0
	for len(ns) > 1 {
		ones := 0

		for _, v := range ns {
			if v[i] == '1' {
				ones++
			}
		}

		var ns2 []string
		for _, v := range ns {
			if ones >= len(ns)/2 {
				if v[i] == '1' {
					ns2 = append(ns2, v)
				} else {
					if v[i] == '0' {
						ns2 = append(ns2, v)
					}
				}
			}
		}

		ns = ns2
		i++
	}

	o2 := ns[0]
	ns = make([]string, len(data))
	copy(ns, data)

	i = 0
	for len(ns) > 1 {
		ones := 0

		for _, v := range ns {
			if v[i] == '1' {
				ones++
			}
		}

		var ns2 []string
		for _, v := range ns {
			if ones >= len(ns)/2 {
				if v[i] == '0' {
					ns2 = append(ns2, v)
				} else {
					if v[i] == '1' {
						ns2 = append(ns2, v)
					}
				}
			}
		}

		ns = ns2
		i++
	}

	co2 := ns[0]
	fmt.Print(o2, co2)
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
