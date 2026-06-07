import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardHeader, CardContent, CardFooter } from "@/components/ui/card";
import { Modal, ModalHeader, ModalContent, ModalFooter } from "@/components/ui/modal";
import { MapContainer } from "@/components/ui/map-container";
import { useState } from "react";

export function RtlTest() {
  const [modalOpen, setModalOpen] = useState(false);

  return (
    <div dir="rtl" className="flex flex-col gap-8 p-8">
      <h1 className="text-2xl font-bold">RTL Test Page</h1>

      <section>
        <h2 className="mb-4 text-lg font-semibold">Button</h2>
        <div className="flex gap-2">
          <Button variant="primary">زر أساسي</Button>
          <Button variant="secondary">زر ثانوي</Button>
          <Button variant="outline">زر حدود</Button>
          <Button variant="ghost">زر شفاف</Button>
        </div>
      </section>

      <section>
        <h2 className="mb-4 text-lg font-semibold">Input</h2>
        <Input label="الاسم الكامل" placeholder="أدخل اسمك..." />
        <div className="mt-2">
          <Input label="البريد الإلكتروني" error="حقل مطلوب" />
        </div>
      </section>

      <section>
        <h2 className="mb-4 text-lg font-semibold">Card</h2>
        <Card>
          <CardHeader>بطاقة المعلومات</CardHeader>
          <CardContent>هذا هو محتوى البطاقة في وضع RTL</CardContent>
          <CardFooter>
            <Button variant="secondary">إلغاء</Button>
            <Button variant="primary" className="me-2">حفظ</Button>
          </CardFooter>
        </Card>
      </section>

      <section>
        <h2 className="mb-4 text-lg font-semibold">Modal</h2>
        <Button onClick={() => setModalOpen(true)}>فتح النافذة</Button>
        <Modal open={modalOpen} onClose={() => setModalOpen(false)}>
          <ModalHeader>تأكيد الحذف</ModalHeader>
          <ModalContent>هل أنت متأكد من رغبتك في حذف هذا العنصر؟</ModalContent>
          <ModalFooter>
            <Button variant="secondary" onClick={() => setModalOpen(false)}>إلغاء</Button>
            <Button variant="primary">تأكيد</Button>
          </ModalFooter>
        </Modal>
      </section>

      <section>
        <h2 className="mb-4 text-lg font-semibold">Map</h2>
        <MapContainer className="h-[400px]" />
      </section>
    </div>
  );
}
