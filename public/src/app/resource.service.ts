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

  getResource(name: string): Observable<Resource> {
    const res = RESOURCES.find((r) => r.name === name)!;
    this.messageService.add(`fetched resource name=${res.name}`);
    return of(res);
  }
}
