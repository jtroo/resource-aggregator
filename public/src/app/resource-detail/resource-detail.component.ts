import { Component, OnInit, Input } from '@angular/core';
import { ActivatedRoute } from '@angular/router';

import { ResourceService } from '../resource.service';
import { Resource } from '../resource';

@Component({
  selector: 'app-resource-detail',
  templateUrl: './resource-detail.component.html',
  styleUrls: ['./resource-detail.component.css']
})

export class ResourceDetailComponent implements OnInit {
  @Input() resource?: Resource;

  constructor(
    private route: ActivatedRoute,
    private resourceService: ResourceService,
  ) {}

  ngOnInit() {
    this.getHero();
  }

  getHero(): void {
    const name = String(this.route.snapshot.paramMap.get('name'));
    this.resourceService.getResource(name)
      .subscribe((resource) => {this.resource = resource});
  }
}
