import { Component, OnInit, Input } from '@angular/core';

import { Resource } from '../resource';

const RESERVED_TIMES = {
  '30 minutes': 1800,
  '1 hour': 3600,
  '2 hours': 7200,
  '4 hours': 14400,
  '1 day': 86400,
  '1 week': 604800,
  'Indefinitely': 0,
}

@Component({
  selector: 'app-resource-reserve',
  templateUrl: './resource-reserve.component.html',
  styleUrls: ['./resource-reserve.component.css']
})
export class ResourceReserveComponent implements OnInit {

  @Input() resource?: Resource;


  constructor() { }

  ngOnInit(): void {
  }

}
