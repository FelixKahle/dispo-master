// Copyright 2023 Felix Kahle. All rights reserved.

import { DialogActions, DialogContent, DialogTitle, Divider, Modal, ModalClose, ModalDialog } from "@mui/joy";
import { Button } from "@mui/joy";
import React from "react";

/**
 * This component provides a modal context that can be used to open modals from anywhere in the app.
 * The modal context is provided by the ModalProvider component.
 * The ModalProvider component must be placed at the root of the app.
 * This interface describes the props of the ModalProvider component.
 */
export interface ModalProviderProps {
  children: React.ReactNode;
}

/**
 * This interface describes the modal context that is provided by the ModalProvider component.
 * It contains the openModal and closeModal functions that can be used to open and close modals.
 * The openModal function takes a React node as the first argument and a function that is called when the modal is closed as the second argument.
 * The closeModal function closes the currently open modal.
 */
export interface ModalContext {
  openModal?: (modal: React.ReactNode, onClose: () => void) => void;
  closeModal?: () => void;
}

/**
 * This hook can be used to access the modal context that is provided by the ModalProvider component.
 * It returns the openModal and closeModal functions that can be used to open and close modals.
 * The openModal function takes a React node as the first argument and a function that is called when the modal is closed as the second argument.
 * The closeModal function closes the currently open modal.
 */
export const ModalContext = React.createContext<ModalContext>({
  openModal: () => {},
  closeModal: () => {},
});

/**
 * This hook can be used to access the modal context that is provided by the ModalProvider component.
 * It returns the openModal and closeModal functions that can be used to open and close modals.
 * The openModal function takes a React node as the first argument and a function that is called when the modal is closed as the second argument.
 * The closeModal function closes the currently open modal.
 * @returns The openModal and closeModal functions that can be used to open and close modals.
 */
export const useModal = (): ModalContext => React.useContext(ModalContext);

/**
 * The ModalProvider component provides a modal context that can be used to open modals from anywhere in the app.
 * The modal context is provided by the ModalProvider component.
 * The ModalProvider component must be placed at the root of the app.
 *
 * @param props The props of the ModalProvider component.
 * @returns The ModalProvider component.
 */
export function ModalProvider(props: ModalProviderProps): JSX.Element {
  const [open, setOpen] = React.useState(false);
  const [currentModal, setCurrentModal] = React.useState<React.ReactNode | null>(null);
  const [onClose, setOnClose] = React.useState<(() => void) | null>(null);

  /**
   * Opens a modal.
   * @param modal The modal to open.
   * @param onClose Closes the modal.
   */
  const openModal = (modal: React.ReactNode, onClose: () => void) => {
    setCurrentModal(modal);
    setOnClose(() => onClose);
    setOpen(true);
  };

  const closeModal = () => {
    setOpen(false);
  };

  const handleClose = () => {
    onClose?.();
  };

  return (
    <>
      <ModalContext.Provider value={{ openModal, closeModal }}>
        <Modal
          sx={{
            zIndex: 12000,
          }}
          open={open}
          onClose={handleClose}
        >
          <>{currentModal && currentModal}</>
        </Modal>
        {props.children}
      </ModalContext.Provider>
    </>
  );
}

/**
 * This interface describes the props of the ConfirmationModal component.
 * It contains the icon, title, message, cancelText, confirmText, onCancel and onConfirm props.
 */
export interface ConfirmationModalProps {
  icon?: React.ComponentType;
  title: string;
  message?: string;
  cancelText?: string;
  confirmText?: string;
  onCancel?: () => void;
  onConfirm?: () => void;
}

/**
 * The ConfirmationModal component can be used to display a confirmation modal.
 *
 * @param props The props of the ConfirmationModal component.
 * @returns The ConfirmationModal component.
 */
export function ConfirmationModal(props: ConfirmationModalProps): JSX.Element {
  return (
    <ModalDialog variant="outlined" role="alertdialog">
      <DialogTitle>
        {props.icon && <props.icon />}
        {props.title}
      </DialogTitle>
      <Divider />
      <DialogContent>{props.message}</DialogContent>
      <DialogActions sx={{ gap: "4" }}>
        <Button variant="solid" color="danger" onClick={() => props.onConfirm?.()} sx={{ width: "120px" }}>
          {props.confirmText}
        </Button>
        <Button variant="plain" color="neutral" onClick={() => props.onCancel?.()} sx={{ width: "120px" }}>
          {props.cancelText}
        </Button>
      </DialogActions>
    </ModalDialog>
  );
}

/**
 * This interface describes the props of the AlertModal component.
 * It contains the icon, title, message and onClose props.
 * The onClose prop is called when the modal is closed.
 */
export interface AlertModalProps {
  icon?: React.ComponentType;
  title: string;
  message?: string;
  onClose?: () => void;
}

/**
 * This component can be used to display an alert modal.
 *
 * @param props The props of the AlertModal component.
 * @returns The AlertModal component.
 */
export function AlertModal(props: AlertModalProps): JSX.Element {
  return (
    <ModalDialog variant="outlined" role="alertdialog">
      <ModalClose />
      <DialogTitle>
        {props.icon && <props.icon />}
        {props.title}
      </DialogTitle>
      <Divider />
      <DialogContent>{props.message}</DialogContent>
      <DialogActions sx={{ gap: "4" }}>
        <Button variant="solid" color="danger" onClick={() => props.onClose?.()} sx={{ width: "120px" }}>
          Ok
        </Button>
      </DialogActions>
    </ModalDialog>
  );
}
