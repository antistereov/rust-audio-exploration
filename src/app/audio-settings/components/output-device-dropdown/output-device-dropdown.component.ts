import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { OutputDeviceService } from '../../services/output-device.service';
import { DropdownModule } from 'primeng/dropdown';
import { debug, error } from '@tauri-apps/plugin-log';

@Component({
  selector: 'rae-output-device-dropdown',
  templateUrl: './output-device-dropdown.component.html',
  styleUrls: ['./output-device-dropdown.component.scss'],
  standalone: true,
  imports: [FormsModule, DropdownModule],
})
export class OutputDeviceDropdown {
  devices: string[] = [];
  selectedDevice: string | null = null;

  constructor(private outputDeviceService: OutputDeviceService) {}

  /**
   * Loading available output devices and currently selected output device.
   *
   * @throws {OutputDeviceError} If loading failed
   */
  async loadDevices(): Promise<void> {
    await this.loadOutputDevices();
    await this.loadSelectedDevice();
  }

  private async loadOutputDevices(): Promise<void> {
    this.devices = await this.outputDeviceService
      .listAvailableOutputDevices()
      .catch((err) => {
        const errorMessage = `Error loading devices: ${err}`;
        error(errorMessage);
        throw new OutputDeviceError(errorMessage, { cause: err });
      });

    debug(`Available output devices:`);
    this.devices.forEach((device, index) => {
      debug(`  ${index + 1}: ${device}`);
    });
  }

  private async loadSelectedDevice(): Promise<void> {
    this.selectedDevice = await this.outputDeviceService
      .getCurrentDeviceName()
      .catch((err) => {
        const errorMessage = `Error selecting output device: ${err}`;
        error(errorMessage);
        throw new OutputDeviceError(errorMessage, { cause: err });
      });

    debug(`Current output device: ${this.selectedDevice}`);
  }

  async onDeviceChange(deviceName: string) {
    await this.outputDeviceService
      .selectOutputDevice(deviceName)
      .catch((err) => {
        const errorMessage = `Error selecting device: ${err}`;
        error(errorMessage);
        throw new OutputDeviceError(errorMessage, { cause: err });
      });

    try {
      this.selectedDevice = await this.outputDeviceService.getCurrentDeviceName();
    } catch(err) {
      const errorMessage = `Error setting selected device: ${err}`;
      error(errorMessage);
      throw new OutputDeviceError(errorMessage, { cause: error });
    }
    debug(`Output device changed to: ${this.selectedDevice}`)
  }
}

class OutputDeviceError extends Error {
  constructor(message: string, options?: ErrorOptions) {
    super(message);
    this.name = "OutputDeviceError";
    if (options?.cause) this.cause = options?.cause;
    Object.setPrototypeOf(this, OutputDeviceError.prototype);
  }
}
