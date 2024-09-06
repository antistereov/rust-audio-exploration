import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';

@Injectable({
  providedIn: 'root',
})
export class OutputDeviceService {
  constructor() {}

  getCurrentDeviceName(): Promise<string | null> {
    return invoke('get_current_output_device_name');
  }

  listOutputDevices(): Promise<string[]> {
    return invoke('list_output_devices');
  }

  selectOutputDevice(deviceName: string): Promise<void> {
    return invoke('select_output_device', { deviceName });
  }
}
