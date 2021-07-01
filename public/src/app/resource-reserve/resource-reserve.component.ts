import { Component, OnInit, Input } from '@angular/core';

import { Resource } from '../resource';
import { ResourceService } from '../resource.service';

const RESERVATION_DURATIONS = [
  { label: '30 minutes', value: 1800 },
  { label: '1 hour', value: 3600 },
  { label: '2 hours', value: 7200 },
  { label: '4 hours', value: 14400 },
  { label: '1 day', value: 86400 },
  { label: '1 week', value: 604800 },
  { label: 'Until cleared', value: 0 },
];

@Component({
  selector: 'app-resource-reserve',
  templateUrl: './resource-reserve.component.html',
  styleUrls: ['./resource-reserve.component.css']
})
export class ResourceReserveComponent implements OnInit {

  @Input() resource?: Resource;

  durations = RESERVATION_DURATIONS;
  reservedBy = "";
  reservedFor: number;

  constructor(
    private resourceService: ResourceService,
  ) {
    this.reservedFor = RESERVATION_DURATIONS[0].value;
  }

  ngOnInit(): void {}

  reserve(): void {
    if (!this.resource) {
      return;
    }
    this.resourceService.reserve(this.resource, this.reservedBy, this.reservedFor)
      .subscribe((errmsg) => {
        if (errmsg.trim()) {
          console.log(`Error reserving ${this.resource && this.resource.name || ''}: ${errmsg}`);
        }
      });
  }

  clear(): void {
    if (!this.resource) {
      return;
    }
    this.resourceService.clearReservation(this.resource)
      .subscribe((errmsg) => {
        if (errmsg.trim()) {
          console.log(`Error clearing reservation for ${this.resource && this.resource.name || ''}: ${errmsg}`);
        }
      });
  }
}
