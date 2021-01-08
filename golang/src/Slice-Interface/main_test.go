package main

import "testing"

// goos: linux
// goarch: amd64
// pkg: github.com/nocai/study/golang-demo/src/Slice-Interface
// BenchmarkString2Bytes-8   	  501470	      2282 ns/op	       0 B/op	       0 allocs/op
// PASS
// ok  	github.com/nocai/study/golang-demo/src/Slice-Interface	1.171s

// PS:
// 由上可知：
// string -> []byte的转换是没有内存开销的（快）
// []byte -> string有开销（慢）

func BenchmarkString2Bytes(b *testing.B) {
	for i := 0; i < b.N; i++ {
		for i := 0; i < 10000; i++ {
			_ = []byte(`尽可能地避免把String转成[]Byte 。这个转换会导致性能下降。`)
		}
	}
}

// Running tool: /usr/bin/go test -benchmem -run=^$ -bench ^(BenchmarkBytes2String)$ github.com/nocai/study/golang-demo/src/Slice-Interface

// goos: linux
// goarch: amd64
// pkg: github.com/nocai/study/golang-demo/src/Slice-Interface
// BenchmarkBytes2String-8   	    4681	    483556 ns/op	  800002 B/op	   10000 allocs/op
// PASS
// ok  	github.com/nocai/study/golang-demo/src/Slice-Interface	2.293s

func BenchmarkBytes2String(b *testing.B) {
	bs := []byte(`尽可能地避免把String转成[]Byte 。这个转换会导致性能下降。`)
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		for i := 0; i < 10000; i++ {
			_ = string(string(bs))
		}
	}
}
