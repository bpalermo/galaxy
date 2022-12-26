package server

import (
	"context"
	"fmt"
	grpcGw "github.com/bpalermo/galaxy-api/service/gateway/v1"
	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"github.com/sirupsen/logrus"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/status"
	"net"
	"net/http"
)

type server struct {
	grpcGw.UnimplementedApiGatewayServiceServer
}

func (g *server) Echo(_ context.Context, request *grpcGw.EchoRequest) (*grpcGw.EchoResponse, error) {
	if request.Value == "" {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	return &grpcGw.EchoResponse{
		Value: fmt.Sprintf("hello %s", request.Value),
	}, nil
}

func Run(grpcServerEndpoint string, logger *logrus.Logger) error {
	ctx := context.Background()
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	// Create a listener on TCP port
	l, err := net.Listen("tcp", grpcServerEndpoint)
	if err != nil {
		return err
	}

	// Create a gRPC server object
	s := grpc.NewServer()
	// Attach the Greeter service to the server
	grpcGw.RegisterApiGatewayServiceServer(s, &server{})
	// Serve gRPC server
	logger.Printf("Serving gRPC on %s", grpcServerEndpoint)
	go func() {
		logger.Fatalln(s.Serve(l))
	}()

	mux := runtime.NewServeMux()
	opts := []grpc.DialOption{grpc.WithTransportCredentials(insecure.NewCredentials())}
	err = grpcGw.RegisterApiGatewayServiceHandlerFromEndpoint(ctx, mux, grpcServerEndpoint, opts)
	if err != nil {
		return err
	}

	gwServer := &http.Server{
		Addr:    ":8081",
		Handler: mux,
	}

	logger.Println("Serving gRPC-Gateway on http://0.0.0.0:8081")

	return gwServer.ListenAndServe()
}
