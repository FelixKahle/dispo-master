// Copyright 2023 Felix Kahle. All rights reserved.

/**
 * The temperature range of a shipment.
 */
export enum TemperatureRange {
  DRY_ICE = "DryIce",
  DRY_SHIPPER = "DryShipper",
  REFRIGERATED = "Refrigerated",
  CONTROLLED_AMBIENT = "ControlledAmbient",
  FROZEN = "Frozen",
  AMBIENT = "Ambient",
  NON_SOP = "NonSOP",
  INVALID = "Invalid",
}

/**
 * Row data of an imported job.
 */
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
  temperatureRanges: TemperatureRange[];
  tolerance: number;
}

/**
 * The disposition mode of a shipment.
 */
export enum DispoMode {
  PICKUP = "Pickup",
  DELIVERY = "Delivery",
}
