import { Card, CardContent, CardHeader } from "@/components/ui/card";

export default function StationFormPanel() {
  return (
    <div className="absolute bottom-4 left-4 right-4 z-[500] mx-auto max-w-lg">
      <Card>
        <CardHeader className="pb-2">
          <h2 className="text-sm font-semibold text-accent">Station Details</h2>
        </CardHeader>
        <CardContent>
          <p className="text-xs text-muted">Station form placeholder</p>
        </CardContent>
      </Card>
    </div>
  );
}
