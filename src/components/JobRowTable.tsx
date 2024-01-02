// Copyright 2023 Felix Kahle. All rights reserved.

import { Sheet, Table, Typography } from "@mui/joy";
import { useSelector } from "react-redux";
import { RootState } from "../app/redux/store";
import { DispoMode } from "../tauri-api/types";

export default function JobRowTable() {
  const rows = useSelector((state: RootState) => state.jobData);

  return (
    <Sheet
      variant="outlined"
      sx={{
        flexDirection: "column",
        width: "100%",
        height: "100%", // Make Sheet fill the parent container
        maxHeight: "100%", // Prevent Sheet from growing beyond the parent container
        borderRadius: "sm",
        overflow: "auto", // Hide overflow to allow the Table to handle scrolling
      }}
    >
      <Table
        stickyHeader
        sx={{
          "--TableCell-headBackground": "var(--joy-palette-background-level1)",
          "--Table-headerUnderlineThickness": "1px",
          "--TableRow-hoverBackground": "var(--joy-palette-background-level1)",
          "--TableCell-paddingY": "4px",
          "--TableCell-paddingX": "8px",
          overflow: "auto",
        }}
      >
        <thead>
          <tr>
            <th style={{ width: "100px" }}>Job Number</th>
            <th style={{ width: "120px" }}>HAWB Number</th>
            <th style={{ width: "100px" }}>Driver</th>
            <th style={{ width: "100px" }}>Vehicle</th>
            <th style={{ width: "300px" }}>Address</th>
            <th style={{ width: "100px" }}>Postal Code</th>
            <th style={{ width: "200px" }}>City</th>
            <th style={{ width: "100px" }}>Country</th>
            <th style={{ width: "100px" }}>Planned Date</th>
            <th style={{ width: "100px" }}>Planned Time</th>
            <th style={{ width: "100px" }}>Tolerance</th>
            <th style={{ width: "300px" }}>Contact Name</th>
          </tr>
        </thead>
        <tbody>
          {rows
            .filter((row) => {
              return row.mode === DispoMode.PICKUP;
            })
            .map((row) => (
              <tr key={row.jobNumber}>
                <td>
                  <Typography level="body-xs">{row.jobNumber}</Typography>
                </td>
                <td>
                  <Typography level="body-xs">{row.hawbNumber}</Typography>
                </td>
                <td>
                  <Typography level="body-xs">{row.driver}</Typography>
                </td>
                <td>
                  <Typography level="body-xs">{row.vehicle}</Typography>
                </td>
                <td>
                  <Typography level="body-xs">{row.address}</Typography>
                </td>
                <td>
                  <Typography level="body-xs">{row.postalCode}</Typography>
                </td>
                <td>
                  <Typography level="body-xs">{row.city}</Typography>
                </td>
                <td>
                  <Typography level="body-xs">{row.country}</Typography>
                </td>
                <td>
                  <Typography level="body-xs">{new Date(row.calculatedDate).toLocaleDateString()}</Typography>
                </td>
                <td>
                  <Typography level="body-xs">{new Date(row.calculatedDate).toLocaleTimeString()}</Typography>
                </td>
                <td>
                  <Typography level="body-xs">{row.tolerance}</Typography>
                </td>
                <td>
                  <Typography level="body-xs">{row.contactName}</Typography>
                </td>
              </tr>
            ))}
        </tbody>
      </Table>
    </Sheet>
  );
}
