import { Injectable } from '@angular/core';
import { Resource } from './resource';
import { RESOURCES } from './mock-resources';
import { Observable, of } from 'rxjs';
import { MessageService } from './message.service';

@Injectable({
  providedIn: 'root'
})

export class ResourceService {

  constructor(private messageService: MessageService) {}

  getResources(): Observable<Resource[]> {
    const resources = of(RESOURCES);
    this.messageService.add('Fetched resources');
    return resources
  }

}
