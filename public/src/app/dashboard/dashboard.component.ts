import { Component, OnInit } from '@angular/core';
import { Resource } from '../resource';
import { ResourceService } from '../resource.service';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: [ './dashboard.component.css' ]
})
export class DashboardComponent implements OnInit {
  resources: Resource[] = [];

  constructor(private resourceService: ResourceService) { }

  ngOnInit() {
    this.getResources();
  }

  getResources(): void {
    this.resourceService.getResources()
      .subscribe((resources) => this.resources = resources.slice(1, 5));
  }
}
