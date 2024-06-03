import { Run } from "@flow/types";

export const runs: Run[] = [
  {
    id: "12342q34safd",
    project: { id: "1234", name: "ABC Project", workflow: { id: "1234" }, createdAt: "2024/04/26" },
    status: "running",
    startedAt: "2024/04/26",
    logs: false,
    trigger: "api",
  },
  {
    id: "asdf234asdfasdf2",
    project: {
      id: "1234",
      name: "ABC123 Project",
      workflow: { id: "1234" },
      createdAt: "2024/04/26",
    },
    status: "running",
    startedAt: "2024/04/26",
    logs: false,
    trigger: "api",
  },
  {
    id: "23fasfsdf3",
    project: {
      id: "1234",
      name: "asdfABC Project",
      workflow: { id: "1234" },
      createdAt: "2024/04/26",
    },
    status: "running",
    startedAt: "2024/04/26",
    logs: false,
    trigger: "api",
  },
  {
    id: "4",
    project: {
      id: "1234",
      name: "ABdC Project",
      workflow: { id: "1234" },
      createdAt: "2024/04/26",
    },
    status: "queued",
    startedAt: "2024/04/26",
    logs: false,
    trigger: "api",
  },
  {
    id: "5asdf23fsasdf",
    project: { id: "1234", name: "ABC Project", workflow: { id: "1234" }, createdAt: "2024/04/26" },
    status: "queued",
    startedAt: "2024/04/26",
    logs: false,
    trigger: "manual",
  },
  {
    id: "6123asdfasdf",
    project: {
      id: "1234",
      name: "ABfsafdsC Project",
      workflow: { id: "1234" },
      createdAt: "2024/04/26",
    },
    status: "completed",
    startedAt: "2024/05/25",
    completedAt: "2024/05/28",
    logs: true,
    trigger: "api",
  },
  {
    id: "7asf23r",
    project: {
      id: "1234",
      name: "ABC Project 1234",
      workflow: { id: "1234" },
      createdAt: "2024/04/26",
    },
    status: "completed",
    startedAt: "2021/04/26",
    completedAt: "2024/04/26",
    logs: true,
    trigger: "manual",
  },
  {
    id: "234asfdasdf8",
    project: {
      id: "1234",
      name: "ABC asdf Project",
      workflow: { id: "1234" },
      createdAt: "2024/04/26",
    },
    status: "completed",
    startedAt: "2023/01/26",
    completedAt: "2024/04/26",
    logs: true,
    trigger: "manual",
  },
  {
    id: "asf39",
    project: { id: "1234", name: "ABC Project", workflow: { id: "1234" }, createdAt: "2024/04/26" },
    status: "completed",
    startedAt: "2023/04/26",
    completedAt: "2024/04/26",
    logs: true,
    trigger: "manual",
  },
  {
    id: "10",
    project: { id: "1234", name: "ABC Project", workflow: { id: "1234" }, createdAt: "2024/04/26" },
    status: "completed",
    startedAt: "2024/04/21",
    completedAt: "2024/04/26",
    logs: true,
    trigger: "manual",
  },
  {
    id: "11",
    project: { id: "1234", name: "ABC Project", workflow: { id: "1234" }, createdAt: "2024/04/26" },
    status: "completed",
    startedAt: "2024/04/26",
    completedAt: "2024/04/26",
    logs: true,
    trigger: "api",
  },
  {
    id: "1asdf23fa2",
    project: { id: "1234", name: "ABC Project", workflow: { id: "1234" }, createdAt: "2024/04/26" },
    status: "completed",
    startedAt: "2024/04/26",
    completedAt: "2024/04/26",
    logs: true,
    trigger: "api",
  },
  {
    id: "13",
    project: { id: "1234", name: "ABC Project", workflow: { id: "1234" }, createdAt: "2024/04/26" },
    status: "failed",
    startedAt: "2024/05/26",
    completedAt: "2024/05/28",
    logs: true,
    trigger: "api",
  },
  {
    id: "14",
    project: { id: "1234", name: "ABC Project", workflow: { id: "1234" }, createdAt: "2024/04/26" },
    status: "failed",
    startedAt: "2022/04/26",
    completedAt: "2024/04/26",
    logs: true,
    trigger: "api",
  },
];
