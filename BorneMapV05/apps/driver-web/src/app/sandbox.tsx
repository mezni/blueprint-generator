import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardHeader, CardContent } from "@/components/ui/card";
import { Modal, ModalHeader, ModalContent, ModalFooter } from "@/components/ui/modal";
import { MapContainer } from "@/components/ui/map-container";

export function Sandbox() {
  const [modalOpen, setModalOpen] = useState(false);

  return (
    <div className="flex flex-col gap-8 p-8">
      <h1 className="text-2xl font-bold text-[var(--color-text-base)]">
        Design System Sandbox
      </h1>

      <Card>
        <CardHeader>Button — Variants</CardHeader>
        <CardContent>
          <div className="flex gap-2">
            <Button variant="primary">Primary</Button>
            <Button variant="secondary">Secondary</Button>
            <Button variant="outline">Outline</Button>
            <Button variant="ghost">Ghost</Button>
          </div>
          <div className="mt-2 flex gap-2">
            <Button size="sm">Small</Button>
            <Button size="md">Medium</Button>
            <Button size="lg">Large</Button>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>Input — States</CardHeader>
        <CardContent>
          <div className="flex flex-col gap-4">
            <Input label="Default" placeholder="Enter text..." />
            <Input label="With Error" error="This field is required" />
            <Input label="Disabled" disabled value="Cannot edit" />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>Modal</CardHeader>
        <CardContent>
          <Button onClick={() => setModalOpen(true)}>Open Modal</Button>
        </CardContent>
      </Card>

      <Modal open={modalOpen} onClose={() => setModalOpen(false)}>
        <ModalHeader>Confirm Action</ModalHeader>
        <ModalContent>Are you sure you want to proceed?</ModalContent>
        <ModalFooter>
          <Button variant="secondary" onClick={() => setModalOpen(false)}>
            Cancel
          </Button>
          <Button variant="primary" onClick={() => setModalOpen(false)}>
            Confirm
          </Button>
        </ModalFooter>
      </Modal>

      <Card>
        <CardHeader>Map Container</CardHeader>
        <CardContent>
          <MapContainer className="h-[400px]" />
        </CardContent>
      </Card>
    </div>
  );
}
