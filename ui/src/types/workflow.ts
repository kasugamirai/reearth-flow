import type { Edge } from "./edge";
import type { Node } from "./node";

export type Workflow = {
  id: string;
  nodes?: Node[];
  edges?: Edge[];
  // status??
  // params?: any;
};
