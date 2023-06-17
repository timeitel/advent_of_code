package main

import ("testing")

func TestMain(t *testing.T) {
  testStr := `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`

  answer := 24000
  result := process(testStr)
  if result != answer {
    t.Errorf("wow not good, expected %d and got %d", answer, result)
  }
}
