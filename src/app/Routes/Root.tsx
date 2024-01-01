// Copyright 2023 Felix Kahle. All rights reserved.

import Box from "@mui/joy/Box";
import Sidebar from "../Sidebar";
import Header from "../Header";
import { Navigate, Outlet, createHashRouter, useNavigate } from "react-router-dom";
import ErrorPage from "./ErrorPage";
import About from "./About";
import Add from "./Add";

/**
 * The names of the pages.
 */
export enum PageName {
  Dispo = "dispo",
  Pickup = "pickup",
  Delivery = "delivery",
  Add = "add",
  Settings = "settings",
  About = " about",
}

/**
 * Generates the path for a page.
 *
 * @param pageName The name of the page.
 * @returns The path of the page.
 */
function generatePagePath(pageName: PageName): string {
  return "/" + pageName;
}

// The router.
export const router = createHashRouter([
  {
    path: "/",
    element: <Root />,
    errorElement: <ErrorPage />,
    children: [
      {
        path: "/",
        element: <Navigate to={generatePagePath(PageName.Dispo)} />,
      },
      {
        path: generatePagePath(PageName.Dispo),
        element: <div>Dispo</div>,
      },
      {
        path: generatePagePath(PageName.Pickup),
        element: <div>Pickup</div>,
      },
      {
        path: generatePagePath(PageName.Delivery),
        element: <div>Delivery</div>,
      },
      {
        path: generatePagePath(PageName.Add),
        element: <Add />,
      },
      {
        path: generatePagePath(PageName.Settings),
        element: <div>Settings</div>,
      },
      {
        path: generatePagePath(PageName.About),
        element: <About />,
      },
    ],
  },
]);

/**
 * The root component of the application.
 *
 * @returns The root component.
 */
export function Root(): JSX.Element {
  const navigate = useNavigate();

  const pageIndexChange = (pageIndex: number) => {
    const routesNames = Object.values(PageName);

    // Check if the page index is valid (i.e. in the range of the routes)
    if (pageIndex < 0 || pageIndex >= routesNames.length) {
      return;
    }
    console.log("Navigate to: " + routesNames[pageIndex]);
    navigate("/" + routesNames[pageIndex]);
  };

  return (
    <Box sx={{ display: "flex", minHeight: "100dvh" }}>
      <Sidebar onIndexChange={pageIndexChange} />
      <Header />
      <Box
        component="main"
        className="MainContent"
        sx={{
          px: {
            xs: 2,
            md: 6,
          },
          pt: {
            xs: "calc(12px + var(--Header-height))",
            sm: "calc(12px + var(--Header-height))",
            md: "calc(60px + var(--Header-height))",
          },
          pb: {
            xs: 2,
            sm: 2,
            md: 3,
          },
          minWidth: 0,
          width: "100%",
          height: "100vh",
        }}
      >
        <Outlet />
      </Box>
    </Box>
  );
}
