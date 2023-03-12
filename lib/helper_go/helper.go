package helper_go

import (
	"context"
	"github.com/google/uuid"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
	"strings"
)

const (
	xJwtSubjectHeader = "x-jwt-subject"
)

var (
	missingHeaderErrorMessage = "missing '%s' header"
)

func GetSubjectIdFromMetadata(ctx context.Context) (uuid.UUID, error) {
	subjectId, err := getValue(ctx, xJwtSubjectHeader)
	if err != nil {
		return uuid.Nil, err
	}
	return uuid.Parse(subjectId)
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
