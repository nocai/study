package main

import (
	"crypto/tls"
	"fmt"
	"time"
)

func main() {
	fmt.Println("tsar")
	s1, _ := NewServer("localhost", 1024)
	s2, _ := NewServer("localhost", 2048, Protocol("udp"))
	s3, _ := NewServer("0.0.0.0", 8080, Timeout(300*time.Second), MaxConns(1000))
}

type Server struct {
	Addr     string
	Port     int
	Protocol string
	Timeout  time.Duration
	MaxConns int
	TLS      *tls.Config
}

func NewServer(addr string, port int, opts ...Option) (*Server, error) {
	srv := Server{
		Addr:     addr,
		Port:     port,
		Protocol: "tcp",
		Timeout:  time.Second * 30,
		MaxConns: 1024,
		TLS:      nil,
	}

	for _, opt := range opts {
		opt(&srv)
	}

	return &srv, nil
}

type Option func(*Server)

func Protocol(protocol string) Option {
	return func(s *Server) {
		s.Protocol = protocol
	}
}

func Timeout(timeout time.Duration) Option {
	return func(s *Server) {
		s.Timeout = timeout
	}
}

func MaxConns(maxConns int) Option {
	return func(s *Server) {
		s.MaxConns = maxConns
	}
}

func TLS(tls *tls.Config) Option {
	return func(s *Server) {
		s.TLS = tls
	}
}
