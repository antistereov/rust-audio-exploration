import { Component } from '@angular/core';
import { ButtonModule } from 'primeng/button';
import { AudioSettingsComponent } from './audio-settings/audio-settings.component';

@Component({
  selector: 'app-root',
  standalone: true,
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
  imports: [ButtonModule, AudioSettingsComponent],
})
export class AppComponent {
  title = 'rust-audio-exploration';
}
