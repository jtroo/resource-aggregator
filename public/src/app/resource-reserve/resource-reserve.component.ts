import { Component, OnInit, Input } from '@angular/core';

import { Resource } from '../resource';

const RESERVATION_DURATIONS = [
  { label: '30 minutes', value: 1800 },
  { label: '1 hour', value: 3600},
  { label: '2 hours', value: 7200},
  { label: '4 hours', value: 14400},
  { label: '1 day', value: 86400},
  { label: '1 week', value: 604800},
  { label: 'Until cleared', value: 0},
];

@Component({
  selector: 'app-resource-reserve',
  templateUrl: './resource-reserve.component.html',
  styleUrls: ['./resource-reserve.component.css']
})
export class ResourceReserveComponent implements OnInit {

  @Input() resource?: Resource;
  durations = RESERVATION_DURATIONS;

  constructor() {}

  ngOnInit(): void {}

  reserve(): void {}
  clearReservation(): void {}
}
