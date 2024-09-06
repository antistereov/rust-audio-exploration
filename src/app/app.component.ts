import { Component } from '@angular/core';
import { ButtonModule } from 'primeng/button'

@Component({
  selector: 'app-root',
  standalone: true,
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
  imports: [ButtonModule]
})
export class AppComponent {
  title = 'rust-audio-exploration';
}
