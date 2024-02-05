import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CharacterSelecterComponent } from './character-selecter.component';

describe('CharacterSelecterComponent', () => {
  let component: CharacterSelecterComponent;
  let fixture: ComponentFixture<CharacterSelecterComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CharacterSelecterComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(CharacterSelecterComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
