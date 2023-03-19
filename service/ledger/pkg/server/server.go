package server

import (
	"context"
	ledger "github.com/bpalermo/galaxy-api/service/ledger/v1"
	db "github.com/bpalermo/galaxy/service/ledger/db/sqlc"
	"github.com/rs/zerolog/log"
	"google.golang.org/grpc"
	"net"
	"time"
)

const (
	defaultTimeout = 3000 * time.Second
)

type LedgerServer struct {
	ledger.UnimplementedLedgerServiceServer
	timeout            time.Duration
	grpcServerEndpoint string
	db                 *db.Queries
}

type LedgerServerOption func(*LedgerServer)

func New(grpcServerEndpoint string, opts ...LedgerServerOption) *LedgerServer {
	l := &LedgerServer{
		timeout:            defaultTimeout,
		grpcServerEndpoint: grpcServerEndpoint,
	}

	for _, opt := range opts {
		opt(l)
	}

	return l
}

func WithTimeout(timeout time.Duration) LedgerServerOption {
	return func(c *LedgerServer) {
		c.timeout = timeout
	}
}

func (srv *LedgerServer) Run() error {
	if err := runMigration(); err != nil {
		log.Error().Err(err).Msg("failed to run migration")
	}
	return srv.runServer()
}

func runMigration() error {
	return nil
}

func (srv *LedgerServer) runServer() error {
	ctx := context.Background()
	ctx, cancel := context.WithTimeout(ctx, srv.timeout)
	defer cancel()

	// Create a listener on TCP port
	l, err := net.Listen("tcp", srv.grpcServerEndpoint)
	if err != nil {
		return err
	}

	// Create a gRPC server object
	s := grpc.NewServer()
	// Attach the Greeter service to the server
	ledger.RegisterLedgerServiceServer(s, srv)
	// Serve gRPC server
	log.Info().Msgf("Serving gRPC on %s", srv.grpcServerEndpoint)
	return s.Serve(l)
}
