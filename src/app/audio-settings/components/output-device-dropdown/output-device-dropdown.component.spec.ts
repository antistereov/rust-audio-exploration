import { ComponentFixture, TestBed } from '@angular/core/testing';

import { OutputDeviceDropdown } from './output-device-dropdown.component';

describe('HostDropdownComponent', () => {
  let component: OutputDeviceDropdown;
  let fixture: ComponentFixture<OutputDeviceDropdown>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [OutputDeviceDropdown],
    }).compileComponents();

    fixture = TestBed.createComponent(OutputDeviceDropdown);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
