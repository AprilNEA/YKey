import React, { useState, useEffect } from 'react'
import DeviceAPI, { DeviceInfo } from '../lib/device-api'
import { Button } from './ui/button'
import { Card } from './ui/card'

export const DeviceManager: React.FC = () => {
  const [devices, setDevices] = useState<DeviceInfo[]>([])
  const [connectedDevices, setConnectedDevices] = useState<string[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Scan for devices
  const handleScanDevices = async () => {
    setLoading(true)
    setError(null)
    try {
      const discoveredDevices = await DeviceAPI.scanDevices()
      setDevices(discoveredDevices)
      console.log('Discovered devices:', discoveredDevices)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to scan devices')
    } finally {
      setLoading(false)
    }
  }

  // Connect to a device
  const handleConnectDevice = async (deviceId: string) => {
    setLoading(true)
    setError(null)
    try {
      await DeviceAPI.connectDevice(deviceId)
      await updateConnectedDevices()
      console.log(`Connected to device: ${deviceId}`)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to connect device')
    } finally {
      setLoading(false)
    }
  }

  // Disconnect from a device
  const handleDisconnectDevice = async (deviceId: string) => {
    setLoading(true)
    setError(null)
    try {
      await DeviceAPI.disconnectDevice(deviceId)
      await updateConnectedDevices()
      console.log(`Disconnected from device: ${deviceId}`)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to disconnect device')
    } finally {
      setLoading(false)
    }
  }

  // Get device information
  const handleGetDeviceInfo = async (deviceId: string) => {
    setLoading(true)
    setError(null)
    try {
      const deviceInfo = await DeviceAPI.getDeviceInfo(deviceId)
      console.log('Device info:', deviceInfo)
      alert(`Device Info:\n${JSON.stringify(deviceInfo, null, 2)}`)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to get device info')
    } finally {
      setLoading(false)
    }
  }

  // Send test command
  const handleSendTestCommand = async (deviceId: string) => {
    setLoading(true)
    setError(null)
    try {
      // Send a simple test command (4 zero bytes)
      const response = await DeviceAPI.sendRawCommand(deviceId, [0x00, 0x00, 0x00, 0x00])
      console.log('Command response:', response)
      alert(`Command Response:\n${JSON.stringify(response)}`)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to send command')
    } finally {
      setLoading(false)
    }
  }

  // Disconnect all devices
  const handleDisconnectAll = async () => {
    setLoading(true)
    setError(null)
    try {
      await DeviceAPI.disconnectAllDevices()
      await updateConnectedDevices()
      console.log('Disconnected all devices')
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to disconnect all devices')
    } finally {
      setLoading(false)
    }
  }

  // Update connected devices list
  const updateConnectedDevices = async () => {
    try {
      const connected = await DeviceAPI.getConnectedDevices()
      setConnectedDevices(connected)
    } catch (err) {
      console.error('Failed to get connected devices:', err)
    }
  }

  // Auto-scan on component mount
  useEffect(() => {
    handleScanDevices()
    updateConnectedDevices()
  }, [])

  const isDeviceConnected = (deviceId: string) => {
    return connectedDevices.includes(deviceId)
  }

  return (
    <div className="p-6 max-w-4xl mx-auto">
      <h1 className="text-3xl font-bold mb-6">🔐 Hardware Security Key Manager</h1>

      {/* Control Buttons */}
      <div className="flex gap-4 mb-6">
        <Button 
          onClick={handleScanDevices} 
          disabled={loading}
          className="bg-blue-600 hover:bg-blue-700"
        >
          {loading ? '🔄 Scanning...' : '🔍 Scan Devices'}
        </Button>
        <Button 
          onClick={handleDisconnectAll} 
          disabled={loading || connectedDevices.length === 0}
          variant="outline"
        >
          🔌 Disconnect All
        </Button>
      </div>

      {/* Error Display */}
      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
          <strong>Error:</strong> {error}
        </div>
      )}

      {/* Connected Devices Status */}
      <div className="mb-6">
        <h2 className="text-xl font-semibold mb-2">📊 Connection Status</h2>
        <p className="text-gray-600">
          Connected devices: {connectedDevices.length > 0 ? connectedDevices.join(', ') : 'None'}
        </p>
      </div>

      {/* Device List */}
      <div className="space-y-4">
        <h2 className="text-xl font-semibold">📱 Discovered Devices ({devices.length})</h2>
        
        {devices.length === 0 ? (
          <Card className="p-6 text-center text-gray-500">
            <p>No devices found. Make sure your security key is connected and try scanning again.</p>
            <p className="mt-2 text-sm">
              Ensure your YubiKey or other FIDO2 device is properly inserted into a USB port.
            </p>
          </Card>
        ) : (
          devices.map((device) => {
            const connected = isDeviceConnected(device.id)
            return (
              <Card key={device.id} className="p-6">
                <div className="flex justify-between items-start">
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-2">
                      <h3 className="text-lg font-semibold">{device.name}</h3>
                      <span className={`px-2 py-1 text-xs rounded-full ${
                        connected 
                          ? 'bg-green-100 text-green-800' 
                          : 'bg-gray-100 text-gray-800'
                      }`}>
                        {connected ? '🟢 Connected' : '⚪ Disconnected'}
                      </span>
                    </div>
                    
                    <div className="grid grid-cols-2 gap-4 text-sm text-gray-600">
                      <div>
                        <strong>ID:</strong> {device.id}
                      </div>
                      <div>
                        <strong>Manufacturer:</strong> {device.manufacturer}
                      </div>
                      <div>
                        <strong>Product:</strong> {device.product_name}
                      </div>
                      <div>
                        <strong>Type:</strong> {device.device_type}
                      </div>
                      <div>
                        <strong>Transport:</strong> {device.transport}
                      </div>
                      <div>
                        <strong>VID/PID:</strong> 0x{device.vendor_id.toString(16).padStart(4, '0')}/0x{device.product_id.toString(16).padStart(4, '0')}
                      </div>
                    </div>
                    
                    <div className="mt-2">
                      <strong className="text-sm">Capabilities:</strong>
                      <div className="flex flex-wrap gap-1 mt-1">
                        {device.capabilities.map((cap) => (
                          <span key={cap} className="px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded">
                            {cap}
                          </span>
                        ))}
                      </div>
                    </div>
                  </div>
                  
                  <div className="flex flex-col gap-2 ml-4">
                    {!connected ? (
                      <Button 
                        onClick={() => handleConnectDevice(device.id)}
                        disabled={loading}
                        size="sm"
                        className="bg-green-600 hover:bg-green-700"
                      >
                        🔌 Connect
                      </Button>
                    ) : (
                      <>
                        <Button 
                          onClick={() => handleDisconnectDevice(device.id)}
                          disabled={loading}
                          size="sm"
                          variant="outline"
                        >
                          🔌 Disconnect
                        </Button>
                        <Button 
                          onClick={() => handleGetDeviceInfo(device.id)}
                          disabled={loading}
                          size="sm"
                          variant="outline"
                        >
                          ℹ️ Info
                        </Button>
                        <Button 
                          onClick={() => handleSendTestCommand(device.id)}
                          disabled={loading}
                          size="sm"
                          variant="outline"
                        >
                          🧪 Test
                        </Button>
                      </>
                    )}
                  </div>
                </div>
              </Card>
            )
          })
        )}
      </div>
    </div>
  )
}

export default DeviceManager 