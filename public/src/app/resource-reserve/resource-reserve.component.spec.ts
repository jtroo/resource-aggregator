import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ResourceReserveComponent } from './resource-reserve.component';

describe('ResourceReserveComponent', () => {
  let component: ResourceReserveComponent;
  let fixture: ComponentFixture<ResourceReserveComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ResourceReserveComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ResourceReserveComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
