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

  reserve(resource: Resource, reservedBy: string, reservedFor: number): Observable<string> {
    let reqResource = {
      name: resource.name,
      reserved_by: reservedBy,
      reserved_until: 0,
    };
    resource.reserved_by = reservedBy;

    if (reservedFor === 0) {
      reqResource.reserved_until = 0;
      resource.reserved_until = 0;
    } else {
      const t = Math.floor((Date.now() / 1000) + reservedFor);
      reqResource.reserved_until = t
      resource.reserved_until = t;
    }
    return this.http.post<string>(this.resourceURL(), reqResource);
  }

  clearReservation(resource: Resource): Observable<string> {
    resource.reserved_by = '';
    resource.reserved_until = 0;
    const reqResource = {
      name: resource.name,
      reserved_by: '',
      reserved_until: 0,
    };
    return this.http.post<string>(this.resourceURL(), reqResource);
  }
}
