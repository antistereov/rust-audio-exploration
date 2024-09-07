import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AudioHostDropdown } from './audio-host-dropdown.component';

describe('HostDropdownComponent', () => {
  let component: AudioHostDropdown;
  let fixture: ComponentFixture<AudioHostDropdown>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AudioHostDropdown],
    }).compileComponents();

    fixture = TestBed.createComponent(AudioHostDropdown);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
