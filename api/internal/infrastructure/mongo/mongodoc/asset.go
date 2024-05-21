package mongodoc

import (
	"time"

	"github.com/reearth/reearth-flow/api/pkg/asset"
	"github.com/reearth/reearth-flow/api/pkg/id"
	"github.com/reearth/reearthx/account/accountdomain"
	"golang.org/x/exp/slices"
)

type AssetDocument struct {
	ID          string
	CreatedAt   time.Time
	Workspace   string
	Name        string
	Size        int64
	URL         string
	ContentType string
}

type AssetConsumer = Consumer[*AssetDocument, *asset.Asset]

func NewAssetConsumer(workspaces []accountdomain.WorkspaceID) *AssetConsumer {
	return NewConsumer[*AssetDocument, *asset.Asset](func(a *asset.Asset) bool {
		return workspaces == nil || slices.Contains(workspaces, a.Workspace())
	})
}

func NewAsset(asset *asset.Asset) (*AssetDocument, string) {
	aid := asset.ID().String()
	return &AssetDocument{
		ID:          aid,
		CreatedAt:   asset.CreatedAt(),
		Workspace:   asset.Workspace().String(),
		Name:        asset.Name(),
		Size:        asset.Size(),
		URL:         asset.URL(),
		ContentType: asset.ContentType(),
	}, aid
}

func (d *AssetDocument) Model() (*asset.Asset, error) {
	aid, err := id.AssetIDFrom(d.ID)
	if err != nil {
		return nil, err
	}
	tid, err := accountdomain.WorkspaceIDFrom(d.Workspace)
	if err != nil {
		return nil, err
	}

	return asset.New().
		ID(aid).
		CreatedAt(d.CreatedAt).
		Workspace(tid).
		Name(d.Name).
		Size(d.Size).
		URL(d.URL).
		ContentType(d.ContentType).
		Build()
}
