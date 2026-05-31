import { Card, CardContent, CardHeader } from "@/components/ui/card";

export default function SearchPanel() {
  return (
    <div className="absolute top-4 left-4 z-[500] w-72">
      <Card>
        <CardHeader className="pb-2">
          <h2 className="text-sm font-semibold text-accent">Search</h2>
        </CardHeader>
        <CardContent>
          <p className="text-xs text-muted">Station search placeholder</p>
        </CardContent>
      </Card>
    </div>
  );
}
