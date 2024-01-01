// Copyright 2023 Felix Kahle. All rights reserved.

import Box from "@mui/joy/Box";
import Sheet from "@mui/joy/Sheet";
import Typography from "@mui/joy/Typography";
import { isRouteErrorResponse, useRouteError } from "react-router-dom";

/**
 * The error page.
 * Shown when an error occurs.
 * This should nether happen as routing is done completly locally.
 * Keep it in here for safety and good practice.
 *
 * @returns The error page.
 */
export default function ErrorPage() {
  const error: any = useRouteError();
  return (
    <Box
      sx={{
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        flexDirection: "column",
        minHeight: "100vh",
      }}
    >
      <Sheet
        sx={{
          maxWidth: "600px", // max width
          width: "50%", // full width
          mx: "auto", // margin left & right
          my: "auto", // margin top & bottom
          padding: 2,
          display: "flex",
          flexDirection: "column",
          gap: 2,
          borderWidth: 1,
          borderRadius: "md",
          boxShadow: "md",
        }}
        variant="outlined"
      >
        {isRouteErrorResponse(error) ? (
          // Render content for the route error condition
          <>
            <Typography color="danger" level="title-lg">
              Error
            </Typography>
            <Typography>{error.status}</Typography>
            <Typography>{error.statusText}</Typography>
            {error.data?.message && <Typography>{error.data.message}</Typography>}
          </>
        ) : (
          <>
            <Typography color="danger" level="title-lg">
              Error
            </Typography>
            <Typography>An unexpected error occured</Typography>
          </>
        )}
      </Sheet>
    </Box>
  );
}
