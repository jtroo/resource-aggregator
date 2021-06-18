import { Component, OnInit } from '@angular/core';
import { Resource } from '../resource';

@Component({
  selector: 'app-resources',
  templateUrl: './resources.component.html',
  styleUrls: ['./resources.component.css']
})
export class ResourcesComponent implements OnInit {

  constructor() { }

  ngOnInit(): void {
  }

  resource: Resource = {
    name: "10.2.21.31",
    description: "A random server",
    status: "reserved",
  }
}
