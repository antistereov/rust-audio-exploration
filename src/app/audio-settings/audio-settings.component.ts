import { Component, OnInit } from '@angular/core';
import { DropdownModule } from 'primeng/dropdown';
import { OutputDeviceDropdown } from './components/output-device-dropdown/output-device-dropdown.component';
import { AudioHostDropdown } from './components/audio-host-dropdown/audio-host-dropdown.component';

@Component({
  selector: 'audio-settings',
  standalone: true,
  imports: [DropdownModule, OutputDeviceDropdown, AudioHostDropdown],
  templateUrl: './audio-settings.component.html',
  styleUrl: './audio-settings.component.scss',
})
export class AudioSettingsComponent {}
