import { createLazyFileRoute } from "@tanstack/react-router";

import { WorkspaceSettings } from "@flow/features/WorkspaceSettings";

export const Route = createLazyFileRoute("/workspace/$workspaceId/settings/$tab")({
  component: () => <WorkspaceSettings />,
});
