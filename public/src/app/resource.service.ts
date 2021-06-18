import { Injectable } from '@angular/core';
import { Resource } from './resource';
import { RESOURCES } from './mock-resources';
import { Observable, of } from 'rxjs';

@Injectable({
  providedIn: 'root'
})

export class ResourceService {

  constructor() { }

  getHeroes(): Observable<Resource[]> {
    const resources = of(RESOURCES);
    return resources
  }

}
