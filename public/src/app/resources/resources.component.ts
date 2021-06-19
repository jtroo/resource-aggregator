import { Component, OnInit } from '@angular/core';
import { Resource } from '../resource';
import { ResourceService } from '../resource.service';

@Component({
  selector: 'app-resources',
  templateUrl: './resources.component.html',
  styleUrls: ['./resources.component.css']
})
export class ResourcesComponent implements OnInit {

  resources: Resource[] = [];
  selectedResource?: Resource;

  constructor(
    private resourceService: ResourceService,
  ) {}

  ngOnInit(): void {
    this.getResources();
  }

  getResources(): void {
    this.resourceService
      .getResources()
      .subscribe((resources) => {this.resources = resources});
  }

}
