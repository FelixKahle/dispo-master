// Copyright 2023 Felix Kahle. All rights reserved.

import { Card, Stack } from "@mui/joy";
import Select from "@mui/joy/Select/Select";
import Option from "@mui/joy/Option/Option";
import MultiStep from "../../components/MultiStep";
import XLSDropzone from "../../components/XLSDropzone";
import { useState } from "react";
import { DispoMode } from "../../tauri-api/types";
import { parseFiles } from "../../tauri-api/tauriAPI";
import { AlertModal, useModal } from "../../components/ModalProvider";

import ErrorOutlineOutlinedIcon from "@mui/icons-material/ErrorOutlineOutlined";

/**
 * The add route.
 * Used to add new dispo files via a cl-view and a shipper site file
 * that are parsed and saved to the database.
 * These files are .xls files and downloaded from TMS.
 *
 * @returns The component.
 */
export default function Add() {
  const [clViewFile, setCLViewFile] = useState<string | null>(null);
  const [shipperSiteFile, setShipperSiteFile] = useState<string | null>(null);
  const [currentIndex, setCurrentIndex] = useState(0);
  const [mode, setMode] = useState<DispoMode | null>(null);
  const { openModal, closeModal } = useModal();

  const onModeChange = (
    _event: React.MouseEvent<Element, MouseEvent> | React.KeyboardEvent<Element> | React.FocusEvent<Element, Element> | null,
    value: any
  ) => {
    if (value === DispoMode.PICKUP) {
      setMode(DispoMode.PICKUP);
      return;
    } else if (value === DispoMode.DELIVERY) {
      setMode(DispoMode.DELIVERY);
      return;
    }

    setMode(null);
  };

  const onCLViewFileChange = (file: string) => {
    setCLViewFile(file);
  };

  const onDeleteCLViewHandler = () => {
    if (clViewFile === null) {
      return;
    }
    setCLViewFile(null);
  };

  const onShipperSiteFileChange = (file: string) => {
    setShipperSiteFile(file);
  };

  const onDeleteShipperSiteHandler = () => {
    if (shipperSiteFile === null) {
      return;
    }
    setShipperSiteFile(null);
  };

  const showErrorMessage = (message: string) => {
    const handleClose = () => {
      if (closeModal) {
        closeModal();
      }
    };

    if (openModal) {
      openModal(<AlertModal icon={ErrorOutlineOutlinedIcon} title="Error" message={message} onClose={handleClose} />, handleClose);
    }
  };

  const onFinish = () => {
    if (clViewFile === null || shipperSiteFile === null || mode === null) {
      return;
    }

    // Parse the files
    // This happens on a different thread in the backend.
    parseFiles(clViewFile, shipperSiteFile, mode)
      .then((rows) => {
        console.log(rows);
        rows.forEach((row) => {
          console.log(row.country);
        });
      })
      .catch((error) => {
        showErrorMessage(error);
      });

    // Reset the state
    setCLViewFile(null);
    setShipperSiteFile(null);
    setMode(null);
    setCurrentIndex(0);
  };

  const steps = [
    {
      name: "Select CL-View",
      component: <XLSDropzone onDelete={onDeleteCLViewHandler} onDrop={onCLViewFileChange} selectedFilePath={clViewFile} />,
    },
    {
      name: "Select Shipper Site",
      component: <XLSDropzone onDelete={onDeleteShipperSiteHandler} onDrop={onShipperSiteFileChange} selectedFilePath={shipperSiteFile} />,
    },
  ];

  const canContinue = (index: number) => {
    switch (index) {
      case 0:
        return clViewFile !== null;
      case 1:
        return shipperSiteFile !== null && mode !== null;
      default:
        return false;
    }
  };

  return (
    <Card variant="outlined" sx={{ minWidth: "100vh", height: "100%" }}>
      <Stack gap={4} sx={{ minWidth: "100vh", height: "100%" }}>
        <Select value={mode} onChange={onModeChange} placeholder="Select a mode" sx={{ width: "200px" }}>
          <Option value={DispoMode.PICKUP}>Pickup</Option>
          <Option value={DispoMode.DELIVERY}>Delivery</Option>
        </Select>
        <MultiStep
          steps={steps}
          currentIndex={currentIndex}
          onNext={() => setCurrentIndex(currentIndex + 1)}
          onPrevious={() => setCurrentIndex(currentIndex - 1)}
          canContinue={canContinue}
          onFinish={onFinish}
        />
      </Stack>
    </Card>
  );
}
