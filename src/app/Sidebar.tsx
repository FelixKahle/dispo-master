// Copyright 2023 Felix Kahle. All rights reserved.

import React from "react";
import Box from "@mui/joy/Box";
import ListItemButton, { listItemButtonClasses } from "@mui/joy/ListItemButton";
import Sheet from "@mui/joy/Sheet";
import ColorSchemeToggle from "../components/ColorSchemeToggle";
import Typography from "@mui/joy/Typography";
import GlobalStyles from "@mui/joy/GlobalStyles";
import ListItem from "@mui/joy/ListItem";
import List from "@mui/joy/List";
import ListItemContent from "@mui/joy/ListItemContent";
import Divider from "@mui/joy/Divider";

// Icons
import DashboardRoundedIcon from "@mui/icons-material/DashboardRounded";
import AddIcon from "@mui/icons-material/Add";
import InfoIcon from "@mui/icons-material/Info";
import SettingsRoundedIcon from "@mui/icons-material/SettingsRounded";
import LocalShippingIcon from "@mui/icons-material/LocalShipping";
import InventoryIcon from "@mui/icons-material/Inventory";

/**
 * Opens the {@link Sidebar}.
 */
export const openSidebar = () => {
  if (typeof document !== "undefined") {
    document.body.style.overflow = "hidden";
    document.documentElement.style.setProperty("--SideNavigation-slideIn", "1");
  }
};

/**
 * Closes the {@link Sidebar}.
 */
export const closeSidebar = () => {
  if (typeof document !== "undefined") {
    document.documentElement.style.removeProperty("--SideNavigation-slideIn");
    document.body.style.removeProperty("overflow");
  }
};

/**
 * Toggles the {@link Sidebar}.
 */
export const toggleSidebar = () => {
  if (typeof window !== "undefined" && typeof document !== "undefined") {
    const slideIn = window.getComputedStyle(document.documentElement).getPropertyValue("--SideNavigation-slideIn");
    if (slideIn) {
      closeSidebar();
    } else {
      openSidebar();
    }
  }
};

/**
 * The props for the {@link Sidebar}.
 */
export interface SidebarProps {
  /**
   * Called when the index of the sidebar changes.
   * @param index The new index of the sidebar.
   * @returns {void}
   */
  onIndexChange?: (index: number) => void;
}

/**
 * The sidebar component.
 *
 * @param props The props for the sidebar.
 * @returns The sidebar component.
 */
export default function Sidebar(props: SidebarProps): JSX.Element {
  // Remember the current index of the sidebar.
  const [index, setIndex] = React.useState(0);

  // Switches the index of the sidebar.
  const switchIndex = (newIndex: number) => {
    // No change
    if (newIndex === index) {
      return;
    }

    // Set the new index
    setIndex(newIndex);

    // Call the callback, if any is set
    if (props.onIndexChange) {
      props.onIndexChange(newIndex);
    }

    // Close the sidebar
    closeSidebar();
  };

  return (
    <Sheet
      className="Sidebar"
      sx={{
        position: {
          xs: "fixed",
          md: "sticky",
        },
        transform: {
          xs: "translateX(calc(100% * (var(--SideNavigation-slideIn, 0) - 1)))",
          md: "none",
        },
        transition: "transform 0.4s, width 0.4s",
        zIndex: 10000,
        height: "100dvh",
        width: "var(--Sidebar-width)",
        top: 0,
        p: 2,
        flexShrink: 0,
        display: "flex",
        flexDirection: "column",
        gap: 2,
        borderRight: "1px solid",
        borderColor: "divider",
      }}
    >
      <GlobalStyles
        styles={(theme) => ({
          ":root": {
            "--Sidebar-width": "220px",
            [theme.breakpoints.up("lg")]: {
              "--Sidebar-width": "240px",
            },
          },
        })}
      />
      <Box
        className="Sidebar-overlay"
        sx={{
          position: "fixed",
          zIndex: 9998,
          top: 0,
          left: 0,
          width: "100vw",
          height: "100vh",
          opacity: "var(--SideNavigation-slideIn)",
          backgroundColor: "var(--joy-palette-background-backdrop)",
          transition: "opacity 0.4s",
          transform: {
            xs: "translateX(calc(100% * (var(--SideNavigation-slideIn, 0) - 1) + var(--SideNavigation-slideIn, 0) * var(--Sidebar-width, 0px)))",
            lg: "translateX(-100%)",
          },
        }}
        onClick={() => closeSidebar()}
      />
      <Box sx={{ display: "flex", gap: 1, alignItems: "center" }}>
        <Typography level="h4">Dispo Master</Typography>
        <ColorSchemeToggle sx={{ ml: "auto" }} />
      </Box>
      <Divider />
      <Box
        sx={{
          minHeight: 0,
          overflow: "hidden auto",
          flexGrow: 1,
          display: "flex",
          flexDirection: "column",
          [`& .${listItemButtonClasses.root}`]: {
            gap: 1.5,
          },
        }}
      >
        <List
          size="sm"
          sx={{
            gap: 1,
            "--List-nestedInsetStart": "30px",
            "--ListItem-radius": (theme) => theme.vars.radius.sm,
          }}
        >
          <ListItem>
            <ListItemButton selected={index === 0} onClick={() => switchIndex(0)}>
              <DashboardRoundedIcon />
              <ListItemContent>
                <Typography level="title-sm">Dispo</Typography>
              </ListItemContent>
            </ListItemButton>
          </ListItem>

          <ListItem>
            <ListItemButton selected={index === 1} onClick={() => switchIndex(1)}>
              <InventoryIcon />
              <ListItemContent>
                <Typography level="title-sm">Pickup</Typography>
              </ListItemContent>
            </ListItemButton>
          </ListItem>

          <ListItem>
            <ListItemButton selected={index === 2} onClick={() => switchIndex(2)}>
              <LocalShippingIcon />
              <ListItemContent>
                <Typography level="title-sm">Delivery</Typography>
              </ListItemContent>
            </ListItemButton>
          </ListItem>

          <ListItem>
            <ListItemButton selected={index === 3} onClick={() => switchIndex(3)}>
              <AddIcon />
              <ListItemContent>
                <Typography level="title-sm">Add</Typography>
              </ListItemContent>
            </ListItemButton>
          </ListItem>
        </List>

        <List
          size="sm"
          sx={{
            mt: "auto",
            flexGrow: 0,
            "--ListItem-radius": (theme) => theme.vars.radius.sm,
            "--List-gap": "8px",
            mb: 2,
          }}
        >
          <ListItem>
            <ListItemButton selected={index === 4} onClick={() => switchIndex(4)}>
              <SettingsRoundedIcon />
              <ListItemContent>
                <Typography level="title-sm">Settings</Typography>
              </ListItemContent>
            </ListItemButton>
          </ListItem>

          <ListItem>
            <ListItemButton selected={index === 5} onClick={() => switchIndex(5)}>
              <InfoIcon />
              <ListItemContent>
                <Typography level="title-sm">About</Typography>
              </ListItemContent>
            </ListItemButton>
          </ListItem>
        </List>
      </Box>
    </Sheet>
  );
}
