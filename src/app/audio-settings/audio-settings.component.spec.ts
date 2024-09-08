import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AudioSettings } from './audio-settings.component';

describe('AudioSettingsComponent', () => {
  let component: AudioSettings;
  let fixture: ComponentFixture<AudioSettings>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AudioSettings],
    }).compileComponents();

    fixture = TestBed.createComponent(AudioSettings);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
