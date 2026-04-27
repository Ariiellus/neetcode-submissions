func longestConsecutive(nums []int) int {
  	numSet := make(map[int]bool)
  	for _, n := range nums {
  		numSet[n] = true
  	}

  	longest := 0

  	for _, n := range nums {
  		if !numSet[n-1] {
  			length := 0
  			for numSet[n+length] {
  				length++
  			}
  			if length > longest {
  				longest = length
  			}
  		}
  	}

  	return longest
}