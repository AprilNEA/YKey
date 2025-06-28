import { ShieldCheck } from "lucide-react"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"

const InfoItem = ({ label, value }: { label: string; value: string }) => (
  <div>
    <p className="text-sm font-medium text-gray-500 dark:text-gray-400">{label}</p>
    <p className="font-semibold">{value}</p>
  </div>
)

const ProtocolItem = ({ name }: { name: string }) => (
  <div className="flex items-center justify-between">
    <span className="text-sm font-medium">{name}</span>
    <div className="flex h-5 w-5 items-center justify-center rounded-full bg-green-100 dark:bg-green-900">
      <ShieldCheck className="h-3 w-3 text-green-600 dark:text-green-400" />
    </div>
  </div>
)

export default function DashboardView({ device }: { device: any }) {
  return (
    <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
      <Card className="lg:col-span-2">
        <CardHeader>
          <CardTitle>Device Information</CardTitle>
          <CardDescription>Details for your connected hardware key.</CardDescription>
        </CardHeader>
        <CardContent className="grid gap-6 md:grid-cols-2">
          <div className="flex items-center justify-center">
            <img
              src={`/placeholder.svg?height=128&width=128&query=${device.name.replace(/\s/g, "+")}`}
              alt="Hardware Key"
              width={128}
              height={128}
              className="rounded-lg"
            />
          </div>
          <div className="space-y-4">
            <InfoItem label="Device Name" value={device.name} />
            <InfoItem label="Serial Number" value={device.serial} />
            <InfoItem label="Firmware Version" value={device.firmware} />
            <InfoItem label="Connection" value="USB-C" />
          </div>
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>Supported Protocols</CardTitle>
          <CardDescription>Protocols enabled on this device.</CardDescription>
        </CardHeader>
        <CardContent className="space-y-2">
          <ProtocolItem name="FIDO2/WebAuthn" />
          <ProtocolItem name="OATH (TOTP/HOTP)" />
          <ProtocolItem name="PIV" />
          <ProtocolItem name="OpenPGP" />
          <ProtocolItem name="Yubico OTP" />
        </CardContent>
      </Card>
    </div>
  )
} 