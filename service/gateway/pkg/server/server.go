package server

import (
	"context"
	"fmt"
	grpcGw "github.com/bpalermo/galaxy-api/service/gateway/v1"
	ledger "github.com/bpalermo/galaxy-api/service/ledger/v1"
	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"github.com/rs/zerolog/log"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/status"
	"net"
	"net/http"
	"time"

	"github.com/bpalermo/galaxy/service/gateway/pkg/service"
)

type Server struct {
	grpcGw.UnimplementedApiGatewayServiceServer
	ledgerClient       *service.LedgerClient
	grpcServerEndpoint string
}

func New(grpcServerEndpoint string) *Server {
	return &Server{
		ledgerClient:       service.NewLedgerClient("ledger:50051"),
		grpcServerEndpoint: grpcServerEndpoint,
	}
}

func (srv *Server) Echo(_ context.Context, request *grpcGw.EchoRequest) (*grpcGw.EchoResponse, error) {
	if request.Value == "" {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	return &grpcGw.EchoResponse{
		Value: fmt.Sprintf("Hello %s", request.Value),
	}, nil
}

func (srv *Server) NewAccount(ctx context.Context, request *ledger.NewAccountRequest) (*ledger.NewAccountResponse, error) {
	if request.OwnerId == "" {
		return nil, status.Error(codes.InvalidArgument, "invalid owner_id")
	}
	if request.Currency == "" || len(request.Currency) != 3 {
		return nil, status.Error(codes.InvalidArgument, "invalid currency")
	}

	return srv.ledgerClient.NewAccount(ctx, request)
}

func (srv *Server) Run() error {
	ctx := context.Background()
	ctx, cancel := context.WithTimeout(ctx, 3000*time.Second)
	defer cancel()

	// Create a listener on TCP port
	l, err := net.Listen("tcp", srv.grpcServerEndpoint)
	if err != nil {
		return err
	}

	// Create a gRPC server object
	s := grpc.NewServer()
	// Attach the Greeter service to the server
	grpcGw.RegisterApiGatewayServiceServer(s, srv)
	// Serve gRPC server
	log.Info().Msgf("Serving gRPC on %s", srv.grpcServerEndpoint)
	go func() {
		log.Fatal().Err(s.Serve(l)).Msg("Failed to serve gRPC server")
	}()

	mux := runtime.NewServeMux()
	opts := []grpc.DialOption{grpc.WithTransportCredentials(insecure.NewCredentials())}
	err = grpcGw.RegisterApiGatewayServiceHandlerFromEndpoint(ctx, mux, srv.grpcServerEndpoint, opts)
	if err != nil {
		return err
	}

	gwServer := &http.Server{
		Addr:    ":8081",
		Handler: mux,
	}

	log.Info().Msg("Serving gRPC-Gateway on http://0.0.0.0:8081")

	return gwServer.ListenAndServe()
}
