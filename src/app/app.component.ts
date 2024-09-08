import { Component } from '@angular/core';
import { ButtonModule } from 'primeng/button';
import { AudioSettingsComponent } from './audio-settings/audio-settings.component';
import { SimpleKnob } from './synthesizer/components/simple-knob/simple-knob.component';
import { SimpleButton } from './synthesizer/components/simple-button/simple-button.component';

@Component({
  selector: 'app-root',
  standalone: true,
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
  imports: [ButtonModule, AudioSettingsComponent, SimpleKnob, SimpleButton],
})
export class AppComponent {
  title = 'rust-audio-exploration';
}
