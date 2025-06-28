/**
 * Copyright 2025 AprilNEA LLC
 * SPDX-License-Identifier: MIT
 */

import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"

export default function PIVView() {
  return (
    <div className="flex h-[50vh] flex-col items-center justify-center rounded-xl border-2 border-dashed">
      <div className="text-center">
        <h2 className="text-2xl font-bold">PIV Management</h2>
        <p className="mt-2 text-muted-foreground">Manage your PIV certificates and keys.</p>
        <Button className="mt-4">Learn More</Button>
      </div>
    </div>
  )
} 