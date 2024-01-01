// Copyright 2023 Felix Kahle. All rights reserved.

import Button from "@mui/joy/Button/Button";
import Card from "@mui/joy/Card/Card";
import { getPrinterNames } from "../../tauri-api/tauriAPI";

/**
 * The about page.
 *
 * @returns The about page.
 */
export default function About() {
  const handleClick = () => {
    getPrinterNames()
      .then((names) => {
        names.forEach((name) => {
          console.log(name);
        });
      })
      .catch((err) => {
        console.log(err);
      });
  };

  return (
    <Card variant="outlined" sx={{ minWidth: "100vh" }}>
      <Button onClick={handleClick}>Test</Button>
    </Card>
  );
}
