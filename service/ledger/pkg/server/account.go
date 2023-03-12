package server

import (
	"context"
	ledger "github.com/bpalermo/galaxy-api/service/ledger/v1"
	helper "github.com/bpalermo/galaxy/lib/helper_go"
	db "github.com/bpalermo/galaxy/service/ledger/db/sqlc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (srv *LedgerServer) MyNewAccount(ctx context.Context, request *ledger.MyNewAccountRequest) (*ledger.MyNewAccountResponse, error) {
	if err := request.Validate(); err != nil {
		return nil, status.Error(codes.InvalidArgument, "bad payload")
	}
	ownerId, err := helper.GetSubjectIdFromMetadata(ctx)
	if err != nil {
		return nil, err
	}
	_, err = srv.db.CreateAccount(ctx, db.CreateAccountParams{
		OwnerID: ownerId[:],
	})
	if err != nil {
		return nil, err
	}
	return nil, nil
}
