import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { AudioHostService, WindowsHost } from '../../services/audio-host.service';
import { debug, error } from '@tauri-apps/plugin-log';
import { FormsModule } from '@angular/forms';
import { DropdownModule } from 'primeng/dropdown';

@Component({
  selector: 'rae-audio-host-dropdown',
  standalone: true,
  imports: [FormsModule, DropdownModule],
  templateUrl: './audio-host-dropdown.component.html',
  styleUrl: './audio-host-dropdown.component.scss',
})
export class AudioHostDropdown {
  audioHost!: WindowsHost;
  availableHosts!: { label: string; value: WindowsHost; }[];

  @Output() hostChanged = new EventEmitter<string>();

  constructor(private audioHostService: AudioHostService) {}

  async loadAudioHost() {
    try {
      this.audioHost = await this.audioHostService.getCurrentHost();
    } catch (err) {
      error(`Error getting current audio host: ${err}`)
      throw new AudioHostError(`Error getting current audio host: ${err}`, { cause: err })
    };

    this.availableHosts = Object.keys(WindowsHost).map((key) => ({
      label: WindowsHost[key as keyof typeof WindowsHost],
      value: WindowsHost[key as keyof typeof WindowsHost],
    }));

    debug(`Current audio host: ${this.audioHost}`);
  }

  async onAudioHostChange(audioHost: WindowsHost) {
    await this.audioHostService.switchHost(audioHost).catch((err) => {
      error(`Error selecting audio host: ${audioHost}`);
      throw new AudioHostError(`Error switching host`, { cause: err })
    });

    this.audioHost = await this.audioHostService.getCurrentHost();
    debug(`Output device changed to: ${this.audioHost}`);

    this.hostChanged.emit(audioHost);
  }
}

class AudioHostError extends Error {
  constructor(message: string, options?: ErrorOptions) {
    super(message);
    this.name = 'AudioHostError';
    if (options?.cause) this.cause = options.cause;

    Object.setPrototypeOf(this, AudioHostError.prototype);
  }
}
