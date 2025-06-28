/**
 * Copyright 2025 AprilNEA LLC
 * SPDX-License-Identifier: MIT
 */

import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"

// Simple input component
const Input = ({ id, type = "text", placeholder }: { id: string; type?: string; placeholder?: string }) => (
  <input
    id={id}
    type={type}
    placeholder={placeholder}
    className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
  />
)

// Simple label component
const Label = ({ htmlFor, children }: { htmlFor: string; children: React.ReactNode }) => (
  <label htmlFor={htmlFor} className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
    {children}
  </label>
)

// Simple separator component
const Separator = () => <div className="border-t border-gray-200 dark:border-gray-700 my-4" />

export default function SettingsView() {
  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>PIN Management</CardTitle>
          <CardDescription>Change the PIN for your hardware key.</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="current-pin">Current PIN</Label>
            <Input id="current-pin" type="password" />
          </div>
          <div className="space-y-2">
            <Label htmlFor="new-pin">New PIN</Label>
            <Input id="new-pin" type="password" />
          </div>
          <div className="space-y-2">
            <Label htmlFor="confirm-pin">Confirm New PIN</Label>
            <Input id="confirm-pin" type="password" />
          </div>
          <Button>Change PIN</Button>
        </CardContent>
      </Card>
      <Card className="border-red-200 dark:border-red-800">
        <CardHeader>
          <CardTitle>Danger Zone</CardTitle>
          <CardDescription>These actions are irreversible. Please proceed with caution.</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div>
            <h3 className="font-semibold">Firmware Update</h3>
            <p className="text-sm text-muted-foreground">Check for and install the latest firmware for your device.</p>
            <Button variant="secondary" className="mt-2">
              Check for Updates
            </Button>
          </div>
          <Separator />
          <div>
            <h3 className="font-semibold">Device Reset</h3>
            <p className="text-sm text-muted-foreground">This will erase all data and credentials from your device.</p>
            <Button variant="destructive" className="mt-2">
              Reset Device
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  )
} 