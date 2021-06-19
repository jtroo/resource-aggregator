import { Component, OnInit, Input } from '@angular/core';
import { Resource } from '../resource';

@Component({
  selector: 'app-resource-detail',
  templateUrl: './resource-detail.component.html',
  styleUrls: ['./resource-detail.component.css']
})

export class ResourceDetailComponent implements OnInit {
  @Input() resource?: Resource;

  constructor() { }

  ngOnInit() {
  }
}
