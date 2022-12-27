package main

import (
	"flag"
	"github.com/bpalermo/galaxy/service/gateway/pkg/server"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

var (
	grpcServerEndpoint = flag.String("grpc-server-endpoint", "0.0.0.0:9090", "gRPC server endpoint")
)

func init() {
	zerolog.SetGlobalLevel(zerolog.InfoLevel)
}

func main() {
	flag.Parse()

	srv := server.New(*grpcServerEndpoint)
	if err := srv.Run(); err != nil {
		log.Fatal().
			Err(err).
			Msg("failed to run")
	}
}
