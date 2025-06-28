/**
 * Copyright 2025 AprilNEA LLC
 * SPDX-License-Identifier: MIT
 */

"use client"

import { useState } from "react"
import {
  KeyRound,
  ShieldCheck,
  Clock,
  Fingerprint,
  FileKey,
  Settings,
  Sun,
  Moon,
  Lock,
  ChevronDown,
  Usb,
} from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuLabel,
  DropdownMenuSeparator,
} from "@/components/ui/dropdown-menu"
import { Route, Switch, useLocation } from "wouter"

import DashboardView from "./manager/dashboard"
import WebAuthnView from "./manager/webauthn"
import OATHView from "./manager/oath"
import PIVView from "./manager/piv"
import OpenPGPView from "./manager/openpgp"
import SettingsView from "./manager/settings"

type NavItem = "dashboard" | "webauthn" | "oath" | "piv" | "openpgp" | "settings"

const navItems = [
  { id: "dashboard", label: "Dashboard", icon: KeyRound },
  { id: "webauthn", label: "WebAuthn (FIDO2)", icon: Fingerprint },
  { id: "oath", label: "OATH Codes", icon: Clock },
  { id: "piv", label: "PIV", icon: ShieldCheck },
  { id: "openpgp", label: "OpenPGP", icon: FileKey },
  { id: "settings", label: "Settings", icon: Settings },
]

const mockDevices = [
  { id: "yubikey-1", name: "YubiKey 5C NFC", serial: "12345678", firmware: "5.4.3" },
  { id: "canokey-1", name: "CanoKey Pigeon", serial: "87654321", firmware: "1.2.0" },
]

export default function XKeyManager() {
  const [connectedDevices, setConnectedDevices] = useState(mockDevices)
  const [selectedDevice, setSelectedDevice] = useState<(typeof mockDevices)[0] | null>(mockDevices[0])
  const [location] = useLocation()

  const handleDeviceSelect = (deviceId: string | null) => {
    if (deviceId === null) {
      setSelectedDevice(null)
    } else {
      const device = connectedDevices.find((d) => d.id === deviceId)
      setSelectedDevice(device || null)
    }
  }

  if (!selectedDevice) {
    return <ConnectDeviceView />
  }

  return (
    <div className="grid min-h-screen w-full lg:grid-cols-[280px_1fr]">
      <div className="hidden border-r bg-gray-100/40 lg:block dark:bg-gray-800/40">
        <div className="flex h-full max-h-screen flex-col gap-2">
          <div className="flex h-[60px] items-center border-b px-6">
            <a href="/" className="flex items-center gap-2 font-semibold">
              <Lock className="h-6 w-6" />
              <span>XKey Manager</span>
            </a>
          </div>
          <div className="flex-1 overflow-auto py-2">
            <nav className="grid items-start px-4 text-sm font-medium">
              {navItems.map((item) => (
                <a
                  key={item.id}
                  href={`/manager/${item.id}`}
                  className={`flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary ${
                    location === `/manager/${item.id}`
                      ? "bg-gray-200 text-primary dark:bg-gray-700"
                      : "text-gray-500 dark:text-gray-400"
                  }`}
                >
                  <item.icon className="h-4 w-4" />
                  {item.label}
                </a>
              ))}
            </nav>
          </div>
        </div>
      </div>
      <div className="flex flex-col">
        <header className="flex h-14 lg:h-[60px] items-center gap-4 border-b bg-gray-100/40 px-6 dark:bg-gray-800/40">
          <div className="lg:hidden">
            <DropdownMenu
              trigger={
                <Button variant="outline" size="icon">
                  <Lock className="h-6 w-6" />
                  <span className="sr-only">Open Menu</span>
                </Button>
              }
            >
              {navItems.map((item) => (
                <DropdownMenuItem key={item.id}>
                  <a href={`/manager/${item.id}`} className="flex items-center">
                    <item.icon className="mr-2 h-4 w-4" />
                    {item.label}
                  </a>
                </DropdownMenuItem>
              ))}
            </DropdownMenu>
          </div>
          <div className="flex-1">
            <DeviceSelector devices={connectedDevices} selectedDevice={selectedDevice} onSelect={handleDeviceSelect} />
          </div>
          <Button variant="ghost" size="icon">
            <Sun className="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
            <Moon className="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
            <span className="sr-only">Toggle theme</span>
          </Button>
        </header>
        <main className="flex-1 p-4 sm:p-6 bg-gray-50 dark:bg-gray-900 overflow-auto">
          <Switch>
            <Route path="/manager/dashboard" component={() => <DashboardView device={selectedDevice} />} />
            <Route path="/manager/webauthn" component={WebAuthnView} />
            <Route path="/manager/oath" component={OATHView} />
            <Route path="/manager/piv" component={PIVView} />
            <Route path="/manager/openpgp" component={OpenPGPView} />
            <Route path="/manager/settings" component={SettingsView} />
            <Route path="/manager" component={() => <DashboardView device={selectedDevice} />} />
          </Switch>
        </main>
      </div>
    </div>
  )
}

const DeviceSelector = ({
  devices,
  selectedDevice,
  onSelect,
}: { devices: any[]; selectedDevice: any | null; onSelect: (id: string | null) => void }) => (
  <DropdownMenu
    trigger={
      <Button variant="outline" className="w-full max-w-xs justify-between bg-transparent">
        {selectedDevice ? selectedDevice.name : "No device connected"}
        <ChevronDown className="h-4 w-4 opacity-50" />
      </Button>
    }
  >
    <DropdownMenuContent className="w-56">
      <DropdownMenuLabel>Connected Devices</DropdownMenuLabel>
      <DropdownMenuSeparator />
      {devices.map((device) => (
        <DropdownMenuItem key={device.id} onSelect={() => onSelect(device.id)}>
          {device.name}
        </DropdownMenuItem>
      ))}
      <DropdownMenuSeparator />
      <DropdownMenuItem onSelect={() => onSelect(null)}>Disconnect</DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
)

const ConnectDeviceView = () => (
  <div className="flex h-full flex-col items-center justify-center rounded-xl border-2 border-dashed">
    <div className="text-center p-8">
      <Usb className="mx-auto h-12 w-12 text-gray-400" />
      <h2 className="mt-4 text-2xl font-bold">No Device Connected</h2>
      <p className="mt-2 text-muted-foreground">Please insert your hardware key to begin managing it.</p>
    </div>
  </div>
) 