package service

import (
	"context"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
	"strings"
	"time"
)

const (
	authorizationHeader = "Authorization"
	bearerHeaderPrefix  = "Bearer "
)

var (
	authenticationRequiredError = status.Error(codes.PermissionDenied, "authentication required")
)

type BaseService struct {
	conn    *grpc.ClientConn
	timeout time.Duration
}

func (b *BaseService) NewAuthContextFromRequestContext(requestContext context.Context) (ctx context.Context, cancel context.CancelFunc, err error) {
	md, ok := metadata.FromIncomingContext(requestContext)
	if !ok {
		return nil, nil, status.Errorf(codes.DataLoss, "failed to get metadata")
	}
	authHeader := md[authorizationHeader]
	if len(authHeader) == 0 || len(authHeader) > 1 || strings.Trim(authHeader[0], " ") == "" || !strings.HasPrefix(authHeader[0], bearerHeaderPrefix) {
		return nil, nil, authenticationRequiredError
	}
	splitToken := strings.Split(authHeader[0], bearerHeaderPrefix)
	if len(splitToken) != 2 {
		return nil, nil, authenticationRequiredError
	}

	ctx, cancel = context.WithTimeout(context.Background(), b.timeout)

	// Add token to gRPC Request.
	ctx = metadata.AppendToOutgoingContext(ctx, authorizationHeader, "Bearer "+splitToken[1])
	return ctx, cancel, nil
}
