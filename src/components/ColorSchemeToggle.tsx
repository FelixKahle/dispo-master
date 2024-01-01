// Copyright 2023 Felix Kahle. All rights reserved.

import IconButton, { IconButtonProps } from "@mui/joy/IconButton";
import { useColorScheme } from "@mui/joy/styles/CssVarsProvider";
import React from "react";

// Icons
import DarkModeRoundedIcon from "@mui/icons-material/DarkModeRounded";
import LightModeRoundedIcon from "@mui/icons-material/LightModeRounded";

/**
 * The color scheme toggle component.
 * Displays a sun moon if the current color scheme is light mode,
 * and a sun icon if the current color scheme is dark mode.
 *
 * @returns The color scheme toggle component.
 */
export default function ColorSchemeToggle({ onClick, sx, ...props }: IconButtonProps) {
  const { mode, setMode } = useColorScheme();
  const [mounted, setMounted] = React.useState(false);
  React.useEffect(() => {
    setMounted(true);
  }, []);
  if (!mounted) {
    return <IconButton size="sm" variant="outlined" color="neutral" {...props} sx={sx} disabled />;
  }
  return (
    <IconButton
      id="toggle-mode"
      size="sm"
      variant="outlined"
      color="neutral"
      {...props}
      onClick={(event) => {
        if (mode === "light") {
          setMode("dark");
        } else {
          setMode("light");
        }
        onClick?.(event);
      }}
      sx={[
        {
          "& > *:first-of-type": {
            display: mode === "dark" ? "none" : "initial",
          },
          "& > *:last-of-type": {
            display: mode === "light" ? "none" : "initial",
          },
        },
        ...(Array.isArray(sx) ? sx : [sx]),
      ]}
    >
      <DarkModeRoundedIcon />
      <LightModeRoundedIcon />
    </IconButton>
  );
}
