import { useReactFlow } from "@xyflow/react";
import { Dispatch, SetStateAction, useCallback } from "react";

import { Node } from "@flow/types";

export default () => {
  const { getInternalNode, isNodeIntersecting } = useReactFlow();

  const handleAddToBatch = useCallback(
    (draggedNode: Node, hoveredNode: Node, setNodes: Dispatch<SetStateAction<Node[]>>) => {
      // Check if dragged node isn't already a child to the group
      if (!draggedNode.parentId) {
        const updatedNode = { ...draggedNode, parentId: hoveredNode.id };
        const posX = getInternalNode(updatedNode.id)?.position.x;
        const posY = getInternalNode(updatedNode.id)?.position.y;
        if (posX && posY) {
          updatedNode.position = {
            x: posX - hoveredNode.position.x,
            y: posY - hoveredNode.position.y,
          };
        }
        setNodes(nodes => {
          const newNodes: Node[] = nodes.filter(n => n.id !== updatedNode.id);
          newNodes.push(updatedNode);
          return newNodes;
        });
      }
    },
    [getInternalNode],
  );

  const handleRemoveFromBatch = useCallback(
    (draggedNode: Node, hoveredNode: Node, setNodes: Dispatch<SetStateAction<Node[]>>) => {
      // Check if dragged node is a child to the group
      if (draggedNode.parentId === hoveredNode.id) {
        draggedNode.parentId = undefined;
        const posX = getInternalNode(draggedNode.id)?.position.x;
        const posY = getInternalNode(draggedNode.id)?.position.y;
        if (posX && posY) {
          draggedNode.position = {
            x: posX + hoveredNode.position.x,
            y: posY + hoveredNode.position.y,
          };
        }
        setNodes(nodes =>
          nodes.map(n => {
            if (n.id === draggedNode.id) {
              n = draggedNode;
            }
            return n;
          }),
        );
      }
    },
    [getInternalNode],
  );

  const handleNodeDropInBatch = useCallback(
    (droppedNode: Node, nodes: Node[], setNodes: Dispatch<SetStateAction<Node[]>>) => {
      nodes.forEach(nd => {
        if (nd.type === "batch") {
          //safety check to make sure there's a height and width
          if (nd.measured?.height && nd.measured?.width) {
            const rec = {
              height: nd.measured.height,
              width: nd.measured.width,
              ...nd.position,
            };

            // Check if the dragged node is inside the group
            if (isNodeIntersecting(droppedNode, rec, false)) {
              handleAddToBatch(droppedNode, nd, setNodes);
            } else {
              handleRemoveFromBatch(droppedNode, nd, setNodes);
            }
          }
        }
      });
    },
    [handleAddToBatch, handleRemoveFromBatch, isNodeIntersecting],
  );

  return {
    handleNodeDropInBatch,
  };
};
