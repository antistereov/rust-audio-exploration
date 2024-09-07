import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core'

@Injectable({
  providedIn: 'root',
})
export class OutputDeviceService {
  constructor() {}

  getCurrentDeviceName(): Promise<string | null> {
    return invoke('get_current_output_device_name');
  }

  listAvailableOutputDevices(): Promise<string[]> {
    return invoke('list_available_output_devices');
  }

  selectOutputDevice(deviceName: string): Promise<void> {
    return invoke('select_output_device', { deviceName });
  }
}
