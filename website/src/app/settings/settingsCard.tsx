"use client";

import { Button } from "@/components/ui/button";

export default function SettingsCard() {
  return (
    <div className="flex flex-col gap-8 px-12 py-8 w-96 border border-border rounded-md shadow-md">
      <span>Settings</span>
      <Button>Apply</Button>
    </div>
  );
}
