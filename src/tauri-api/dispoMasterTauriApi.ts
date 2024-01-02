// Copyright 2023 Felix Kahle. All rights reserved.

import { invoke } from "@tauri-apps/api/tauri";
import { DispoMode, TemperatureRange } from "./types";

export interface ImportedJobRow {
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
  tolerance: number;
  temperatureRanges: TemperatureRange[];
}

/**
 * Calls the `get_printer_names` function of the Tauri API.
 *
 * @returns A promise that resolves to an array of printer names.
 */
export function getPrinterNames(): Promise<string[]> {
  return invoke("get_printer_names");
}

/**
 * Close the splash screen.
 *
 * @returns Resolves when the splash screen is closed.
 */
export function closeSplashScreen(): Promise<void> {
  return invoke("close_splashscreen");
}

/**
 * Returns the current mouse position.
 *
 * @returns The current mouse position.
 */
export async function getMousePosition(): Promise<{ x: number; y: number }> {
  return await invoke("get_mouse_position");
}

/**
 * Parses the given files and returns the parsed data.
 * The files must be in the format of the CL View and the Shipper Site downloaded from TMS.
 *
 * @param clViewFile The path to the CL View file.
 * @param shipperSiteFile The path to the Shipper Site file.
 * @param mode The mode of the shipment.
 * @returns Array of parsed job rows.
 */
export async function parseFiles(clViewFile: string, shipperSiteFile: string, mode: DispoMode): Promise<ImportedJobRow[]> {
  const result: Promise<ImportedJobRow[]> = invoke("parse_files", { clView: clViewFile, shipperSite: shipperSiteFile, mode: mode });
  return result;
}
