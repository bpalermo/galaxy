package main

import (
	"flag"
	"github.com/bpalermo/galaxy/service/gateway/pkg/server"
	log "github.com/sirupsen/logrus"
	"os"
)

var (
	logger             *log.Logger
	grpcServerEndpoint = flag.String("grpc-server-endpoint", "localhost:9090", "gRPC server endpoint")
)

func init() {
	flag.Parse()

	logger = log.New()
	logger.SetFormatter(&log.JSONFormatter{})
	logger.SetOutput(os.Stdout)

	logLevel, err := log.ParseLevel(os.Getenv("LOG_LEVEL"))
	if err != nil {
		logLevel = log.WarnLevel
	}

	logger.SetLevel(logLevel)
}

func main() {
	if err := server.Run(*grpcServerEndpoint, logger); err != nil {
		logger.Fatal(err)
	}
}
