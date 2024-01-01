// Copyright 2023 Felix Kahle. All rights reserved.

import { CssBaseline, CssVarsProvider } from "@mui/joy";
import { RouterProvider } from "react-router-dom";
import { router } from "./Routes/Root";
import { DropProvider } from "../components/DropProvider";
import { ModalProvider } from "../components/ModalProvider";

/**
 * The main component of the application.
 *
 * @returns The main component of the application.
 */
export default function App() {
  return (
    <div className="container">
      <CssVarsProvider defaultMode="system">
        <CssBaseline />
        <ModalProvider>
          <DropProvider>
            <RouterProvider router={router} />
          </DropProvider>
        </ModalProvider>
      </CssVarsProvider>
    </div>
  );
}
