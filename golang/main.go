package main

import (
	"fmt"

	algos "github.com/allstackdev1/algorithms-in-diff-languages/golang/algos"
)

func main() {
	fmt.Printf("Special Pairs - GO %v\n", algos.SpecialPairs(18))
	fmt.Printf("Tree Constructor - GO %v\n", algos.TreeConstructor([]string{"(1,2)", "(2,4)", "(5,7)", "(7,2)", "(9,5)"})) // 
	fmt.Printf("Tree Constructor - GO %v\n", algos.TreeConstructor([]string{"(1,2)", "(3,2)", "(2,12)", "(5,2)"}))
}