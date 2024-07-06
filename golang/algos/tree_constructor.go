package algos

import (
	"strings"
)

//
func TreeConstructor(strArr []string) bool {
	parents := make(map[string][]string)
	children := make(map[string]string)

	var replacer = strings.NewReplacer("(", "", ")", "")

	for _, pair := range strArr {
		arr := strings.Split(replacer.Replace(pair), ",")

		child, parent := arr[0], arr[1]

		if _, exists := parents[parent]; !exists {
			parents[parent] = []string{}
		}

		parents[parent] = append(parents[parent], child)

		// Check if the parent has more than 2 children (invalid binary tree)
		if len(parents[parent]) > 2 {
			return false
		}

		// Check if the child already has a parent (invalid binary tree)
		if _, exists := children[child]; exists {
			return false
		}

		children[child] = parent
	}


	rootCount := 0
	for parent := range parents {
		 if _, exists := children[parent]; !exists {
			rootCount++
		 }
	}
	// here should be exactly one root for a valid binary tree
	return rootCount == 1
}
