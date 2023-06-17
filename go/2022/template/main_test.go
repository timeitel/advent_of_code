package main

import (
	"testing"
)

func TestMain(t *testing.T) {
	testStr := ``

	answer := 24000
	ex1, _ := process(testStr)
	if ex1 != answer {
		t.Errorf("wow not good, expected %d and got %d", answer, ex1)
	}
}
