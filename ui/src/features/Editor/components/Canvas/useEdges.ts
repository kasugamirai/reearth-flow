import { OnConnect, OnEdgesChange, addEdge, applyEdgeChanges } from "@xyflow/react";
import { useCallback } from "react";

import { Edge } from "@flow/types";

type Props = {
  edges: Edge[];
  onEdgeChange: (edges: Edge[]) => void;
};

export default ({ edges, onEdgeChange }: Props) => {
  const handleEdgesChange: OnEdgesChange = useCallback(
    changes => onEdgeChange(applyEdgeChanges(changes, edges)),
    [edges, onEdgeChange],
  );

  const handleConnect: OnConnect = useCallback(
    connection => onEdgeChange(addEdge(connection, edges)),
    [edges, onEdgeChange],
  );

  return {
    handleEdgesChange,
    handleConnect,
  };
};
