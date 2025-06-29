import { invoke } from '@tauri-apps/api/core'

// Device information interface matching Rust FrontendDeviceInfo
export interface DeviceInfo {
  id: string
  name: string
  manufacturer: string
  product_name: string
  device_type: string
  transport: string
  vendor_id: number
  product_id: number
  capabilities: string[]
  is_connected: boolean
}

// Device API class for managing hardware security keys
export class DeviceAPI {
  /**
   * Scan for available FIDO2/security key devices
   * @returns Promise<DeviceInfo[]> - Array of discovered devices
   */
  static async scanDevices(): Promise<DeviceInfo[]> {
    try {
      return await invoke<DeviceInfo[]>('scan_devices')
    } catch (error) {
      console.error('Failed to scan devices:', error)
      throw new Error(`Failed to scan devices: ${error}`)
    }
  }

  /**
   * Connect to a specific device
   * @param deviceId - Unique identifier of the device
   */
  static async connectDevice(deviceId: string): Promise<void> {
    try {
      await invoke<void>('connect_device', { deviceId })
    } catch (error) {
      console.error(`Failed to connect to device ${deviceId}:`, error)
      throw new Error(`Failed to connect to device ${deviceId}: ${error}`)
    }
  }

  /**
   * Disconnect from a specific device
   * @param deviceId - Unique identifier of the device
   */
  static async disconnectDevice(deviceId: string): Promise<void> {
    try {
      await invoke<void>('disconnect_device', { deviceId })
    } catch (error) {
      console.error(`Failed to disconnect from device ${deviceId}:`, error)
      throw new Error(`Failed to disconnect from device ${deviceId}: ${error}`)
    }
  }

  /**
   * Get detailed information about a connected device
   * @param deviceId - Unique identifier of the device
   * @returns Promise<DeviceInfo> - Detailed device information
   */
  static async getDeviceInfo(deviceId: string): Promise<DeviceInfo> {
    try {
      return await invoke<DeviceInfo>('get_device_info', { deviceId })
    } catch (error) {
      console.error(`Failed to get info for device ${deviceId}:`, error)
      throw new Error(`Failed to get info for device ${deviceId}: ${error}`)
    }
  }

  /**
   * Send raw command to a device
   * @param deviceId - Unique identifier of the device
   * @param command - Raw command bytes to send
   * @returns Promise<number[]> - Response bytes from device
   */
  static async sendRawCommand(deviceId: string, command: number[]): Promise<number[]> {
    try {
      return await invoke<number[]>('send_raw_command', { deviceId, command })
    } catch (error) {
      console.error(`Failed to send command to device ${deviceId}:`, error)
      throw new Error(`Failed to send command to device ${deviceId}: ${error}`)
    }
  }

  /**
   * Get list of currently connected device IDs
   * @returns Promise<string[]> - Array of connected device IDs
   */
  static async getConnectedDevices(): Promise<string[]> {
    try {
      return await invoke<string[]>('get_connected_devices')
    } catch (error) {
      console.error('Failed to get connected devices:', error)
      throw new Error(`Failed to get connected devices: ${error}`)
    }
  }

  /**
   * Disconnect all currently connected devices
   */
  static async disconnectAllDevices(): Promise<void> {
    try {
      await invoke<void>('disconnect_all_devices')
    } catch (error) {
      console.error('Failed to disconnect all devices:', error)
      throw new Error(`Failed to disconnect all devices: ${error}`)
    }
  }
}

// Export for convenience
export default DeviceAPI 