import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { OutputDeviceService } from '../../services/output-device.service';
import { DropdownModule } from 'primeng/dropdown';

@Component({
  selector: 'output-device-dropdown',
  templateUrl: './output-device-dropdown.component.html',
  styleUrls: ['./output-device-dropdown.component.scss'],
  standalone: true,
  imports: [FormsModule, DropdownModule],
})
export class OutputDeviceDropdown implements OnInit {
  devices: string[] = [];
  selectedDevice: string | null = null;

  constructor(private outputDeviceService: OutputDeviceService) {}

  ngOnInit() {
    this.loadDevices();
  }

  async loadDevices() {
    try {
      this.devices = await this.outputDeviceService.listOutputDevices();
      this.selectedDevice =
        await this.outputDeviceService.getCurrentDeviceName();
    } catch (err) {
      console.error('Error loading devices:', err);
    }
  }

  async onDeviceChange(deviceName: string) {
    await this.outputDeviceService
      .selectOutputDevice(deviceName)
      .catch((err) => {
        console.error('Error selecting device:', err);
      });

    this.selectedDevice =
      await this.outputDeviceService.getCurrentDeviceName();
  }
}
