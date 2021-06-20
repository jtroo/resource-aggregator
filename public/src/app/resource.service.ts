import { Injectable, isDevMode } from '@angular/core';
import { HttpClient } from '@angular/common/http';

import { Resource } from './resource';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

@Injectable({
  providedIn: 'root'
})

export class ResourceService {

  apiURL: string = '';

  constructor(
    private http: HttpClient,
  ) {
    if (isDevMode()) {
      this.apiURL = 'http://localhost:8000';
    }
  }

  private resourceURL(): string {
    return `${this.apiURL}/resource`;
  }

  getResources(): Observable<Resource[]> {
    return this.http.get<Resource[]>(this.resourceURL());
  }

  getResource(name: string): Observable<Resource | undefined> {
    return this.http.get<Resource[]>(this.resourceURL())
      .pipe(
        map((resources) => resources.find(
          (res) => res.name === name
        ))
      );
  }
}
