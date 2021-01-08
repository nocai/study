package main

import (
	"crypto/tls"
	"fmt"
	"time"
)

func main() {
	fmt.Println("tsar")
	svr, _ := NewServer("localhost", 9000, nil)
	conf := Config{Protocol: "tcp", Timeout: time.Second * 60}
	svr2, _ := NewServer("localhost", 9000, &conf)

	_, _ = svr, svr2
}

type Server struct {
	Addr string
	Port int
	// Protocol string
	// Timeout  time.Duration
	// MaxConns int
	// TLS      *tls.Config
	Config Config
}

type Config struct {
	Protocol string
	Timeout  time.Duration
	Maxconns int
	TLS      *tls.Config
}

func NewServer(addr string, prot int, config *Config) (*Server, error) {
	// ...
	return nil, nil
}

// func NewDefaultServer(addr string, port int) (*Server, error) {
// 	return &Server{addr, port, "tcp", 30 * time.Second, 100, nil}, nil
// }
// func NewTLSServer(addr string, port int, tls *tls.Config) (*Server, error) {
// 	return &Server{addr, port, "tcp", 30 * time.Second, 100, tls}, nil
// }
// func NewServerWithTimeout(addr string, port int, timeout time.Duration) (*Server, error) {
// 	return &Server{addr, port, "tcp", timeout, 100, nil}, nil
// }
// func NewTLSServerWithMaxConnAndTimeout(addr string, port int, maxconns int, timeout time.Duration, tls *tls.Config) (*Server, error) {
// 	return &Server{addr, port, "tcp", 30 * time.Second, maxconns, tls}, nil
// }
