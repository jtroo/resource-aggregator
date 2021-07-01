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
  resourceOtherFieldsKeys: string[] = [];

  constructor(
    private route: ActivatedRoute,
    private resourceService: ResourceService,
  ) {
  }

  ngOnInit() {
    this.getResource();
  }

  reservedUntil(): string {
    if (!this.resource) {
      return 'N/A';
    }
    if (this.resource.reserved_until === 0) {
      if (this.resource.reserved_by) {
        return 'Until manually cleared';
      } else {
        return 'Unreserved';
      }
    }
    const date = new Date(this.resource.reserved_until * 1000);
    let hours = ('00' + date.getHours()).slice(-2);
    let minutes = ('00' + date.getMinutes()).slice(-2);
    let seconds = ('00' + date.getSeconds()).slice(-2);
    return `${date.toISOString().split('T')[0]} ${hours}:${minutes}:${seconds}`;
  }

  getResource(): void {
    const name = String(this.route.snapshot.paramMap.get('name'));
    this.resourceService.getResource(name)
      .subscribe((resource) => {
        this.resource = resource;
        if (resource) {
          this.resourceOtherFieldsKeys = Object.keys(resource.other_fields);
        }
      });
  }

  reserveResource(): void {
    if (!this.resource) {
      return;
    }
    this.resourceService.reserve(this.resource, 'ME!', 0)
      .subscribe((errmsg) => {
        if (errmsg.trim()) {
          console.log(`Error reserving ${this.resource && this.resource.name || ''}: ${errmsg}`);
        }
      });
  }

  clearReservation(): void {
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
