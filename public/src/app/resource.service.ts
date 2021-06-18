import { Injectable } from '@angular/core';
import { Resource } from './resource';
import { RESOURCES } from './mock-resources';

@Injectable({
  providedIn: 'root'
})

export class ResourceService {

  constructor() { }

  getHeroes(): Resource[] {
    return RESOURCES;
  }

}
