import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AudioSettingsService } from '../../services/audio-settings.service';
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

  constructor(private audioSettingsService: AudioSettingsService) {}

  ngOnInit() {
    this.loadDevices();
  }

  async loadDevices() {
    try {
      this.devices = await this.audioSettingsService.listOutputDevices();
      this.selectedDevice =
        await this.audioSettingsService.getCurrentDeviceName();
    } catch (err) {
      console.error('Error loading devices:', err);
    }
  }

  async onDeviceChange(deviceName: string) {
    await this.audioSettingsService
      .selectOutputDevice(deviceName)
      .catch((err) => {
        console.error('Error selecting device:', err);
      });

    this.selectedDevice =
      await this.audioSettingsService.getCurrentDeviceName();
  }
}
