// Copyright 2023 Felix Kahle. All rights reserved.

import { DispoMode, ImportedJobRow, TemperatureRange } from "../../tauri-api/types";
import { PayloadAction, createSlice } from "@reduxjs/toolkit";

/**
 * The data of a job row.
 */
export interface JobRow {
  mode: DispoMode;
  address: string;
  calculatedDate: string;
  city: string;
  contactName: string;
  country: string;
  earlyDate: string;
  equipment: string;
  hawbNumber: string;
  jobNumber: string;
  lateDate: string;
  postalCode: string;
  quantities: number;
  temperatureRanges: TemperatureRange[];
  tolerance: number;
  driver: string;
  vehicle: string;
}

/**
 * Creates a job data row from an imported job row.
 *
 * @param row The imported job row.
 * @returns The job data row.
 */
export function fromImportedJobRow(row: ImportedJobRow): JobRow {
  return {
    mode: row.mode,
    address: row.address,
    calculatedDate: row.calculatedDate,
    city: row.city,
    contactName: row.contactName,
    country: row.country,
    earlyDate: row.earlyDate,
    equipment: row.equipment,
    hawbNumber: row.hawbNumber,
    jobNumber: row.jobNumber,
    lateDate: row.lateDate,
    postalCode: row.postalCode,
    quantities: row.quantities,
    temperatureRanges: row.temperatureRanges,
    tolerance: row.tolerance,
    driver: "",
    vehicle: "",
  };
}

// The initial state of the job data.
const initialState: JobRow[] = [];

export const jobDataSlice = createSlice({
  name: "jobData",
  initialState,
  reducers: {
    setJobData: (_state: JobRow[], action: PayloadAction<JobRow[]>) => {
      return action.payload;
    },
    addJobData: (state: JobRow[], action: PayloadAction<JobRow>) => {
      state.push(action.payload);
    },
    addJobDataArray: (state: JobRow[], action: PayloadAction<JobRow[]>) => {
      state.push(...action.payload);
    },
    removeJobData: (state: JobRow[], action: PayloadAction<JobRow>) => {
      return state.filter((job) => job.jobNumber !== action.payload.jobNumber);
    },
    removeJobDataArray: (state: JobRow[], action: PayloadAction<JobRow[]>) => {
      const jobNumbersToRemove = new Set(action.payload.map((job) => job.jobNumber));
      return state.filter((job) => !jobNumbersToRemove.has(job.jobNumber));
    },
    removeJobDataByIndex: (state: JobRow[], action: PayloadAction<number>) => {
      state.splice(action.payload, 1);
    },
    clearJobData: (_state: JobRow[]) => {
      return [];
    },
  },
});

export const { setJobData, addJobData, addJobDataArray, removeJobData, removeJobDataArray, removeJobDataByIndex, clearJobData } =
  jobDataSlice.actions;
