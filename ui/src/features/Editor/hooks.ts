import { useReactFlow } from "@xyflow/react";
import { MouseEvent, useCallback, useState } from "react";

import { useYjsStore } from "@flow/lib/yjs";
import { useCurrentWorkflowId } from "@flow/stores";
import type { Edge, Node } from "@flow/types";
import { cancellableDebounce } from "@flow/utils";

export default () => {
  const [currentWorkflowId, setCurrentWorkflowId] = useCurrentWorkflowId();
  const { getNodes } = useReactFlow();

  const handleWorkflowIdChange = useCallback(
    (id?: string) => {
      if (!id) return setCurrentWorkflowId(undefined);
      setCurrentWorkflowId(id);
    },
    [setCurrentWorkflowId],
  );

  const {
    openWorkflows,
    nodes,
    edges,
    handleWorkflowAdd,
    handleWorkflowClose,
    handleNodesUpdate,
    handleEdgesUpdate,
  } = useYjsStore({
    workflowId: currentWorkflowId,
    handleWorkflowIdChange,
  });

  // Will be used to keep track of all locked nodes, local and for other users (while collaborative editing)
  const [lockedNodeIds, setLockedNodeIds] = useState<string[]>([]);

  // Can have only one node locked at a time (locally)
  const [locallyLockedNode, setLocallyLockedNode] = useState<Node | undefined>(undefined);

  // consider making a node context and supplying vars and functions like this to the nodes that way
  const handleNodeLocking = useCallback(
    (nodeId: string) => {
      handleNodesUpdate(
        getNodes().map(n => {
          if (n.id === nodeId) {
            const newNode = {
              ...n,
              data: {
                ...n.data,
                locked: !n.data.locked,
              },
            };

            setLockedNodeIds(ids => {
              if (ids.includes(newNode.id)) {
                return ids.filter(id => id !== nodeId);
              }
              return [...ids, newNode.id];
            });

            setLocallyLockedNode(lln => (lln?.id === newNode.id ? undefined : newNode));

            return newNode;
          }
          return n;
        }),
      );
    },
    [getNodes, handleNodesUpdate],
  );

  const [hoveredDetails, setHoveredDetails] = useState<Node | Edge | undefined>();

  const hoverActionDebounce = cancellableDebounce((callback: () => void) => callback(), 100);

  const handleNodeHover = useCallback(
    (e: MouseEvent, node?: Node) => {
      hoverActionDebounce.cancel();
      if (e.type === "mouseleave" && hoveredDetails) {
        hoverActionDebounce(() => setHoveredDetails(undefined));
      } else {
        setHoveredDetails(node);
      }
    },
    [hoveredDetails, hoverActionDebounce],
  );

  const handleEdgeHover = useCallback(
    (e: MouseEvent, edge?: Edge) => {
      if (e.type === "mouseleave" && hoveredDetails) {
        setHoveredDetails(undefined);
      } else {
        setHoveredDetails(edge);
      }
    },
    [hoveredDetails],
  );

  return {
    currentWorkflowId,
    openWorkflows,
    nodes,
    edges,
    lockedNodeIds,
    locallyLockedNode,
    hoveredDetails,
    handleWorkflowClose,
    handleWorkflowAdd,
    handleWorkflowChange: handleWorkflowIdChange,
    handleNodesUpdate,
    handleNodeHover,
    handleNodeLocking,
    handleEdgesUpdate,
    handleEdgeHover,
  };
};
