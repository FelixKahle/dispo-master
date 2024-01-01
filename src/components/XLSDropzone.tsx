// Copyright 2023 Felix Kahle. All rights reserved.

import { Box, IconButton, Stack, Typography } from "@mui/joy";
import InsertDriveFileIcon from "@mui/icons-material/InsertDriveFile";
import DeleteIcon from "@mui/icons-material/Delete";
import TaskIcon from "@mui/icons-material/Task";
import { useDrop } from "./DropProvider";
import { useRef } from "react";
import { open } from "@tauri-apps/api/dialog";

/**
 * XLSDropzone props.
 */
export interface XLSDropzoneProps {
  onDrop: (filePath: string) => void;
  onDelete?: () => void;
  selectedFilePath: string | null;
}

/**
 * XLS Dropzone component.
 * Uses Tauri API to open a file dialog and select a .xls file.
 * Also uses Tauri API for drag and drop.
 *
 * @param props The component props.
 * @returns The component.
 */
export default function XLSDropzone(props: XLSDropzoneProps) {
  const dropzoneArea = useRef<HTMLElement>(null);
  const drop = useDrop();

  // Check if a file is selected.
  const hasFile = () => {
    return props.selectedFilePath !== null;
  };

  // Handle drop event.
  const dropHandler = async (event: string[]) => {
    if (event === null) {
      return;
    }

    if (event.length !== 1) {
      return;
    }
    const filePath = event[0];

    if (!filePath.endsWith(".xls")) {
      return;
    }

    props.onDrop(filePath);
  };

  // Handle click event.
  const clickHandler = async () => {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Excel Spreadsheet",
          extensions: ["xls"],
        },
      ],
    });

    if (selected === null) {
      return;
    }

    if (Array.isArray(selected)) {
      return;
    }

    if (!selected.endsWith(".xls")) {
      return;
    }

    props.onDrop(selected);
  };

  // Register drop handler.
  drop.onDrop = dropHandler;

  return (
    <Box
      ref={dropzoneArea}
      sx={{
        width: "100%",
        height: "100%",
        background: (theme) => theme.vars.palette.background.level1,
        borderRadius: (theme) => theme.vars.radius.md,
      }}
    >
      <Stack sx={{ width: "100%", height: "100%" }}>
        <Box display="flex" justifyContent="flex-end">
          <IconButton onClick={props?.onDelete} sx={{ margin: 1 }}>
            <DeleteIcon />
          </IconButton>
        </Box>
        <Box onClick={clickHandler} style={{ width: "100%", height: "100%" }}>
          <Stack display="flex" alignItems="center" justifyContent="center" sx={{ width: "100%", height: "100%" }}>
            {hasFile() ? (
              <>
                <TaskIcon sx={{ width: 40, height: 40 }} />
                <Typography level="h4">You selected the file</Typography>
                <ul style={{ listStyle: "none", margin: "0px", padding: "0px" }}>
                  <Typography>{props?.selectedFilePath}</Typography>
                </ul>
              </>
            ) : (
              <>
                <InsertDriveFileIcon sx={{ width: 40, height: 40 }} />
                <Typography level="h4">Drop your file here or click</Typography>
                <Typography>Drag and drop your .xls file here. You can also click</Typography>
              </>
            )}
          </Stack>
        </Box>
      </Stack>
    </Box>
  );
}
