// Copyright 2023 Felix Kahle. All rights reserved.

import { ImportedJobRow } from "../../tauri-api/types";
import { PayloadAction, createSlice } from "@reduxjs/toolkit";

// The initial state of the job data.
const initialState: ImportedJobRow[] = [];

export const jobDataSlice = createSlice({
  name: "jobData",
  initialState,
  reducers: {
    setJobData: (_state: ImportedJobRow[], action: PayloadAction<ImportedJobRow[]>) => {
      return action.payload;
    },
    addJobData: (state: ImportedJobRow[], action: PayloadAction<ImportedJobRow>) => {
      state.push(action.payload);
    },
    addJobDataArray: (state: ImportedJobRow[], action: PayloadAction<ImportedJobRow[]>) => {
      state.push(...action.payload);
    },
    removeJobData: (state: ImportedJobRow[], action: PayloadAction<ImportedJobRow>) => {
      return state.filter((job) => job.jobNumber !== action.payload.jobNumber);
    },
    removeJobDataArray: (state: ImportedJobRow[], action: PayloadAction<ImportedJobRow[]>) => {
      const jobNumbersToRemove = new Set(action.payload.map((job) => job.jobNumber));
      return state.filter((job) => !jobNumbersToRemove.has(job.jobNumber));
    },
    removeJobDataByIndex: (state: ImportedJobRow[], action: PayloadAction<number>) => {
      state.splice(action.payload, 1);
    },
    clearJobData: (_state: ImportedJobRow[]) => {
      return [];
    },
  },
});

export const { setJobData, addJobData, addJobDataArray, removeJobData, removeJobDataArray, removeJobDataByIndex, clearJobData } =
  jobDataSlice.actions;
