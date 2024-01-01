// Copyright 2023 Felix Kahle. All rights reserved.

import { Check } from "@mui/icons-material";
import { Button, Grid, Stack, Step, StepIndicator, Stepper } from "@mui/joy";
import React from "react";

export interface StepInfo {
  name: string;
  component: React.ReactElement;
}

export interface MultiStepProps {
  steps: StepInfo[];
  canContinue: (currentIndex: number) => boolean;
  onNext?: () => void;
  onPrevious?: () => void;
  onFinish?: () => void;
  currentIndex: number;
}

export default function MultiStep(props: MultiStepProps) {
  const currentIndex = props.currentIndex;

  const handleNext = () => {
    if (!props.canContinue(currentIndex)) {
      return;
    }
    if (props.onNext) {
      props.onNext();
    }
  };

  const handlePrevious = () => {
    if (currentIndex > 0) {
      if (props.onPrevious) {
        props.onPrevious();
      }
    }
  };

  const handleFinish = () => {
    if (!props.canContinue(currentIndex)) {
      return;
    }
    if (props.onFinish) {
      props.onFinish();
    }
  };

  const isFinish = currentIndex === props.steps.length - 1;

  return (
    <>
      <Stack gap={2} sx={{ display: "flex", flexDirection: "column", flex: 1 }}>
        <Stepper sx={{ width: "100%" }}>
          {props.steps.map((step, index) => (
            <Step
              key={step.name}
              indicator={
                <StepIndicator variant={currentIndex <= index ? "soft" : "solid"} color={currentIndex < index ? "neutral" : "primary"}>
                  {currentIndex <= index ? index + 1 : <Check />}
                </StepIndicator>
              }
              sx={{
                "&::after": {
                  ...(currentIndex > index && index !== 2 && { bgcolor: "primary.solidBg" }),
                },
              }}
            >
              {step.name}
            </Step>
          ))}
        </Stepper>
        <div style={{ flex: 1 }}>{props.steps[currentIndex].component}</div>
        <Grid container justifyContent="space-between">
          <Button disabled={currentIndex === 0} variant="soft" onClick={handlePrevious} sx={{ width: "130px" }}>
            Previous
          </Button>
          {isFinish ? (
            <Button disabled={!props.canContinue(currentIndex)} onClick={handleFinish} sx={{ width: "130px" }}>
              Finish
            </Button>
          ) : (
            <Button disabled={!props.canContinue(currentIndex)} onClick={handleNext} sx={{ width: "130px" }}>
              Next
            </Button>
          )}
        </Grid>
      </Stack>
    </>
  );
}
