import { Component, OnInit, ViewChild } from '@angular/core';
import { DropdownModule } from 'primeng/dropdown';
import { OutputDeviceDropdown } from './components/output-device-dropdown/output-device-dropdown.component';
import { AudioHostDropdown } from './components/audio-host-dropdown/audio-host-dropdown.component';
import { debug, error } from '@tauri-apps/plugin-log';

@Component({
  selector: 'rae-audio-settings',
  standalone: true,
  imports: [DropdownModule, OutputDeviceDropdown, AudioHostDropdown],
  templateUrl: './audio-settings.component.html',
  styleUrl: './audio-settings.component.scss',
})
export class AudioSettings implements OnInit {
  @ViewChild('audioHost') audioHostDropdown!: AudioHostDropdown;
  @ViewChild('outputDevice') outputDeviceDropdown!: OutputDeviceDropdown;

  ngOnInit() {}

  async ngAfterViewInit() {
    await this.audioHostDropdown.loadAudioHost().catch(err => {
      const errorMessage = `Error initializing AudioHostDropdown: ${err}`;
      error(errorMessage);
      throw new AudioSettingsError(errorMessage, { cause: err });
    });

    await this.outputDeviceDropdown.loadDevices().catch(err => {
      const errorMessage = `Error initializing OutputDeviceDropdown: ${err}`;
      error(errorMessage);
      throw new AudioSettingsError(errorMessage, { cause: err });
    });
  }

  onHostChanged(newHost: string) {
    debug(`Host changed to ${newHost}; reloading devices`);

    if (!this.outputDeviceDropdown) {
      const errorMessage = 'OutputDeviceDropdown not initialized';
      error(errorMessage);
      throw new AudioSettingsError(errorMessage);
    }

    this.outputDeviceDropdown.loadDevices().catch((err) => {
      const errorMessage = `Error while reloading devices after audio host change: ${err}`;
      error(errorMessage);
      throw new AudioSettingsError(errorMessage, { cause: err });
    });
  }
}

class AudioSettingsError extends Error {
  constructor(message: string, options?: ErrorOptions) {
    super(message)
    this.name = 'AudioSettingsError';
    if (options?.cause) this.cause = options.cause;
    Object.setPrototypeOf(this, AudioSettingsError.prototype);
  }
}
