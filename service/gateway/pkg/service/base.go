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
	AuthorizationHeader = "authorization"
)

var (
	missingHeaderErrorMessage = "missing '%s' header"
)

type BaseClient struct {
	conn    *grpc.ClientConn
	timeout time.Duration
}

func (c *BaseClient) NewClientContext() (context.Context, context.CancelFunc) {
	return context.WithTimeout(context.Background(), c.timeout)
}

func (c *BaseClient) NewClientAuthContext(inCtx context.Context) (outCtx context.Context, cancel context.CancelFunc, err error) {
	outCtx, cancel = context.WithTimeout(context.Background(), c.timeout)
	authHeader, err := getAuthorizationHeader(inCtx)
	if err != nil {
		cancel()
		return nil, nil, err
	}
	md := metadata.New(map[string]string{AuthorizationHeader: authHeader})
	outCtx = metadata.NewOutgoingContext(outCtx, md)
	return outCtx, cancel, nil
}

func (c *BaseClient) Close() error {
	return c.conn.Close()
}

func getAuthorizationHeader(inCtx context.Context) (string, error) {
	return getValue(inCtx, AuthorizationHeader)
}

func getValue(inCtx context.Context, key string) (string, error) {
	v := metadata.ValueFromIncomingContext(inCtx, key)
	if v == nil {
		return "", status.Errorf(codes.DataLoss, missingHeaderErrorMessage, key)
	}
	if len(v) == 0 {
		return "", status.Errorf(codes.InvalidArgument, missingHeaderErrorMessage, key)
	}
	if strings.Trim(v[0], " ") == "" {
		return "", status.Errorf(codes.InvalidArgument, missingHeaderErrorMessage, key)
	}
	return v[0], nil
}
