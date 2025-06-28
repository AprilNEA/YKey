/**
 * Copyright 2025 AprilNEA LLC
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from "react"
import { PlusCircle, Copy } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"

const oathAccounts = [
  { id: "1", issuer: "GitHub", user: "alicia.koch", code: "754 832" },
  { id: "2", issuer: "Google", user: "alicia@google.com", code: "192 384" },
  { id: "3", issuer: "Vercel", user: "alicia.koch", code: "821 093" },
]

// Simple progress bar component
const Progress = ({ value }: { value: number }) => (
  <div className="w-full bg-gray-200 rounded-full h-1 dark:bg-gray-700">
    <div 
      className="bg-blue-600 h-1 rounded-full transition-all duration-300" 
      style={{ width: `${value}%` }}
    />
  </div>
)

export default function OATHView() {
  const [progress, setProgress] = useState(13)

  useEffect(() => {
    const timer = setInterval(() => {
      setProgress((prev) => (prev >= 100 ? 0 : prev + 5))
    }, 1500)
    return () => clearInterval(timer)
  }, [])

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between">
        <div>
          <CardTitle>OATH Accounts</CardTitle>
          <CardDescription>One-time passwords for your accounts.</CardDescription>
        </div>
        <Button size="sm">
          <PlusCircle className="mr-2 h-4 w-4" />
          Add Account
        </Button>
      </CardHeader>
      <CardContent className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {oathAccounts.map((acc) => (
          <Card key={acc.id}>
            <CardHeader>
              <CardTitle className="text-lg">{acc.issuer}</CardTitle>
              <CardDescription>{acc.user}</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="flex items-center justify-between">
                <p className="text-2xl font-bold tracking-widest">{acc.code}</p>
                <Button variant="ghost" size="icon">
                  <Copy className="h-4 w-4" />
                  <span className="sr-only">Copy code</span>
                </Button>
              </div>
              <Progress value={progress} />
            </CardContent>
          </Card>
        ))}
      </CardContent>
    </Card>
  )
} 