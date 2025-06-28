/**
 * Copyright 2025 AprilNEA LLC
 * SPDX-License-Identifier: MIT
 */

import { Link } from "wouter"
import { KeyRound, Fingerprint, Clock, ShieldCheck, FileKey, Settings, ArrowRight, Lock } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"

const features = [
  {
    icon: KeyRound,
    title: "Device Management",
    description: "View device info, manage PINs, and update firmware.",
    href: "/manager/dashboard",
  },
  {
    icon: Fingerprint,
    title: "WebAuthn (FIDO2)",
    description: "Manage and test your FIDO2 credentials with ease.",
    href: "/manager/webauthn",
  },
  {
    icon: Clock,
    title: "OATH Codes",
    description: "Generate and manage TOTP/HOTP codes for 2FA.",
    href: "/manager/oath",
  },
  {
    icon: ShieldCheck,
    title: "PIV Smart Card",
    description: "Handle PIV certificates for enterprise access.",
    href: "/manager/piv",
  },
  {
    icon: FileKey,
    title: "OpenPGP",
    description: "Manage OpenPGP keys for secure communication.",
    href: "/manager/openpgp",
  },
  {
    icon: Settings,
    title: "Advanced Settings",
    description: "Configure the app and manage backups.",
    href: "/manager/settings",
  },
]

export default function LandingPage() {
  return (
    <div className="flex flex-col min-h-screen">
      <header className="px-4 lg:px-6 h-14 flex items-center">
        <Link href="#" className="flex items-center justify-center">
          <Lock className="h-6 w-6" />
          <span className="ml-2 font-semibold">XKey Manager</span>
        </Link>
      </header>
      <main className="flex-1">
        <section className="w-full py-12 md:py-24 lg:py-32">
          <div className="container px-4 md:px-6">
            <div className="flex flex-col items-center space-y-4 text-center">
              <h1 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl/none">
                The Ultimate Cross-Platform Hardware Key Manager
              </h1>
              <p className="mx-auto max-w-[700px] text-gray-500 md:text-xl dark:text-gray-400">
                One app to rule them all. Manage your YubiKeys, CanoKeys, Nitrokeys, and any other FIDO2 device with a
                unified, modern interface.
              </p>
            </div>
          </div>
        </section>
        <section className="w-full pb-12 md:pb-24 lg:pb-32">
          <div className="container grid items-center justify-center gap-4 px-4 md:px-6">
            <div className="grid w-full grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
              {features.map((feature) => (
                <Card key={feature.title} className="flex flex-col">
                  <CardHeader>
                    <feature.icon className="w-8 h-8 mb-2 text-primary" />
                    <CardTitle>{feature.title}</CardTitle>
                    <CardDescription>{feature.description}</CardDescription>
                  </CardHeader>
                  <CardContent className="mt-auto">
                    <Link href={feature.href}>
                      <Button variant="outline" className="w-full bg-transparent">
                        Go to Feature <ArrowRight className="w-4 h-4 ml-2" />
                      </Button>
                    </Link>
                  </CardContent>
                </Card>
              ))}
            </div>
          </div>
        </section>
      </main>
      <footer className="flex flex-col gap-2 sm:flex-row py-6 w-full shrink-0 items-center px-4 md:px-6 border-t">
        <p className="text-xs text-gray-500 dark:text-gray-400">&copy; 2024 XKey Manager. All rights reserved.</p>
      </footer>
    </div>
  )
}
