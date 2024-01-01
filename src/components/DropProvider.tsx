// Copyright 2023 Felix Kahle. All rights reserved.

import { TauriEvent, listen } from "@tauri-apps/api/event";
import React, { useEffect } from "react";

/**
 * Props for the DropProvider component.
 */
export interface DropProviderProps {
  children: React.ReactNode;
}

/**
 * Context for the DropProvider component.
 */
export interface DropContext {
  onDrop?: (filePaths: string[]) => void;
}

/**
 * The DropContext.
 */
export const DropContext = React.createContext<DropContext>({
  onDrop: () => {},
});

/**
 * The useDrop hook.
 * @returns
 */
export const useDrop = (): DropContext => React.useContext(DropContext);

/**
 * DropProvider component.
 * Provides a context for droppping files.
 *
 * @param props The component props.
 * @returns The component.
 */
export function DropProvider(props: DropProviderProps): JSX.Element {
  const onDrop = (event: any) => {
    if (event.payload === null) {
      return;
    }
    // Handle the dropped files here
    const filePaths: string[] = event.payload;
    // Use the onDrop function from the context
    const { onDrop: onDropContext } = dropContextValue;
    onDropContext?.(filePaths);
  };

  useEffect(() => {
    // Subscribe to the file drop event
    const unsubscribe = listen(TauriEvent.WINDOW_FILE_DROP, onDrop);

    // Cleanup function to unsubscribe on component unmount
    return () => {
      unsubscribe.then((u) => u());
    };
  }, []); // Empty dependency array ensures the effect runs once on mount

  const dropContextValue: DropContext = {
    onDrop: () => {}, // Default or overridden onDrop function
  };

  return <DropContext.Provider value={dropContextValue}>{props.children}</DropContext.Provider>;
}
