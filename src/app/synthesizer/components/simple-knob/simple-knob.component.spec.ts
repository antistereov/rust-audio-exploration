import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SimpleKnob } from './simple-knob.component';

describe('SimpleKnob', () => {
  let component: SimpleKnob;
  let fixture: ComponentFixture<SimpleKnob>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SimpleKnob],
    }).compileComponents();

    fixture = TestBed.createComponent(SimpleKnob);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
