import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { Modal, ModalHeader, ModalContent, ModalFooter } from "../modal";

describe("Modal", () => {
  it("renders nothing when closed", () => {
    render(
      <Modal open={false} onClose={() => {}}>
        <ModalHeader>Title</ModalHeader>
      </Modal>,
    );
    expect(screen.queryByText("Title")).not.toBeInTheDocument();
  });

  it("renders content when open", () => {
    render(
      <Modal open={true} onClose={() => {}}>
        <ModalHeader>Title</ModalHeader>
      </Modal>,
    );
    expect(screen.getByText("Title")).toBeInTheDocument();
  });

  it("calls onClose when backdrop is clicked", async () => {
    const handleClose = vi.fn();
    render(
      <Modal open={true} onClose={handleClose}>
        <ModalContent>Content</ModalContent>
      </Modal>,
    );
    await userEvent.click(screen.getByRole("dialog"));
    expect(handleClose).toHaveBeenCalledTimes(1);
  });

  it("calls onClose when Escape key is pressed", async () => {
    const handleClose = vi.fn();
    render(
      <Modal open={true} onClose={handleClose}>
        <ModalContent>Content</ModalContent>
      </Modal>,
    );
    await userEvent.keyboard("{Escape}");
    expect(handleClose).toHaveBeenCalledTimes(1);
  });

  it("renders Modal.Header, Modal.Content, Modal.Footer", () => {
    render(
      <Modal open={true} onClose={() => {}}>
        <ModalHeader>Header</ModalHeader>
        <ModalContent>Content</ModalContent>
        <ModalFooter>Footer</ModalFooter>
      </Modal>,
    );
    expect(screen.getByText("Header")).toBeInTheDocument();
    expect(screen.getByText("Content")).toBeInTheDocument();
    expect(screen.getByText("Footer")).toBeInTheDocument();
  });
});
