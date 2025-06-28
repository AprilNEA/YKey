/**
 * Copyright 2025 AprilNEA LLC
 * SPDX-License-Identifier: MIT
 */

import { Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { useState, ReactNode } from "react"

// Simple input component
const Input = ({ id, placeholder }: { id: string; placeholder?: string }) => (
  <input
    id={id}
    placeholder={placeholder}
    className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
  />
)

// Simple label component
const Label = ({ htmlFor, children }: { htmlFor: string; children: ReactNode }) => (
  <label htmlFor={htmlFor} className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
    {children}
  </label>
)

// Simple tabs components
const Tabs = ({ defaultValue, children }: { defaultValue: string; children: ReactNode[] }) => {
  const [activeTab, setActiveTab] = useState(defaultValue)
  return (
    <div>
      <div className="flex space-x-1 rounded-lg bg-muted p-1">
        <button
          onClick={() => setActiveTab("credentials")}
          className={`rounded-md px-3 py-1.5 text-sm font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 ${
            activeTab === "credentials" ? "bg-background text-foreground shadow-sm" : "text-muted-foreground hover:text-foreground"
          }`}
        >
          Manage Credentials
        </button>
        <button
          onClick={() => setActiveTab("test")}
          className={`rounded-md px-3 py-1.5 text-sm font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 ${
            activeTab === "test" ? "bg-background text-foreground shadow-sm" : "text-muted-foreground hover:text-foreground"
          }`}
        >
          Test & Debug
        </button>
      </div>
      <div className="mt-4">
        {activeTab === "credentials" && children[0]}
        {activeTab === "test" && children[1]}
      </div>
    </div>
  )
}

const webAuthnCredentials = [
  { id: "1", rp: "github.com", user: "alicia.koch", date: "2024-10-26" },
  { id: "2", rp: "vercel.com", user: "alicia.koch", date: "2024-10-25" },
  { id: "3", rp: "google.com", user: "alicia.k", date: "2024-09-12" },
]

export default function WebAuthnView() {
  return (
    <Tabs defaultValue="credentials">
      <Card>
        <CardHeader>
          <CardTitle>WebAuthn Credentials</CardTitle>
          <CardDescription>Manage credentials registered with online services.</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="rounded-md border">
            <div className="bg-muted px-4 py-2 font-medium">
              <div className="grid grid-cols-4 gap-4">
                <div>Relying Party</div>
                <div>User</div>
                <div>Date Added</div>
                <div className="text-right">Actions</div>
              </div>
            </div>
            <div className="divide-y">
              {webAuthnCredentials.map((cred) => (
                <div key={cred.id} className="px-4 py-3">
                  <div className="grid grid-cols-4 gap-4 items-center">
                    <div className="font-medium">{cred.rp}</div>
                    <div>{cred.user}</div>
                    <div>{cred.date}</div>
                    <div className="text-right">
                      <Button variant="ghost" size="icon">
                        <Trash2 className="h-4 w-4 text-red-500" />
                        <span className="sr-only">Delete</span>
                      </Button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>WebAuthn Test Environment</CardTitle>
          <CardDescription>Simulate registration and authentication flows.</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="username">Username</Label>
            <Input id="username" placeholder="example_username" />
          </div>
          <div className="flex gap-2">
            <Button>Register New Credential</Button>
            <Button variant="secondary">Authenticate</Button>
          </div>
          <details>
            <summary className="cursor-pointer text-sm font-medium">Advanced Settings</summary>
            <div className="mt-4 space-y-4 rounded-lg border p-4">
              <p className="text-sm text-muted-foreground">
                Configure advanced WebAuthn parameters here (e.g., attestation, user verification).
              </p>
            </div>
          </details>
        </CardContent>
      </Card>
    </Tabs>
  )
} 