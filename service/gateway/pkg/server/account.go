package server

import (
	"context"
	ledger "github.com/bpalermo/galaxy-api/service/ledger/v1"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (srv *Server) MyNewAccount(ctx context.Context, request *ledger.MyNewAccountRequest) (*ledger.MyNewAccountResponse, error) {
	if request.Currency == "" || len(request.Currency) != 3 {
		return nil, status.Error(codes.InvalidArgument, "invalid currency")
	}

	clientContext, cancel, err := srv.ledgerClient.NewClientAuthContext(ctx)
	defer cancel()
	if err != nil {
		return nil, status.Error(codes.DataLoss, "failed to get metadata")
	}

	return srv.ledgerClient.MyNewAccount(clientContext, request)
}
