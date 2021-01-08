package main

import (
	"fmt"
)

func main() {
	a := make([]int, 32)
	b := append(a, 1)
	a[0] = 1
	fmt.Println(a)
	fmt.Println(b)
	println(a)
	println(b)

	fmt.Println([]byte(`尽可能地避免把String转成[]Byte 。这个转换会导致性能下降。`))
}
