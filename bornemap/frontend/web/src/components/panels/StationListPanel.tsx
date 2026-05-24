import { Card, CardContent, CardHeader } from "@/components/ui/card";

export default function StationListPanel() {
  return (
    <div className="absolute top-4 right-4 z-[500] w-80 max-h-[70vh] overflow-y-auto">
      <Card>
        <CardHeader className="pb-2">
          <h2 className="text-sm font-semibold text-accent">Stations</h2>
        </CardHeader>
        <CardContent>
          <p className="text-xs text-muted">Station list placeholder</p>
        </CardContent>
      </Card>
    </div>
  );
}
