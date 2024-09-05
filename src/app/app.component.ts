import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { invoke } from '@tauri-apps/api/tauri'

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent {
  title = 'rust-audio-exploration';
  greetingMessage = '';

  async greet() {
    this.greetingMessage = await invoke('greet', {name: 'Tauri'})
  }
}
