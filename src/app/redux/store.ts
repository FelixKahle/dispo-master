// Copyright 2023 Felix Kahle. All rights reserved.

import { configureStore } from "@reduxjs/toolkit";
import { jobDataSlice } from "./jobDataSlice";

/**
 * The Redux store of the application.
 * (Render process)
 */
export const store = configureStore({
  reducer: {
    jobData: jobDataSlice.reducer,
  },
});

/**
 * The root state of the application.
 */
export type RootState = ReturnType<typeof store.getState>;

/**
 * The dispatch function of the application.
 */
export type AppDispatch = typeof store.dispatch;
