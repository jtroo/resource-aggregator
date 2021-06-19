import { Injectable, isDevMode } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';

import { Resource } from './resource';
import { RESOURCES } from './mock-resources';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';
import { MessageService } from './message.service';

@Injectable({
  providedIn: 'root'
})

export class ResourceService {

  apiURL: string = '';

  constructor(
    private messageService: MessageService,
    private http: HttpClient,
  ) {
    if (isDevMode()) {
      this.apiURL = 'http://localhost:8000';
    } else {
      this.apiURL = '';
    }
  }

  private log(msg: string) {
    this.messageService.add(`ResourceService: ${msg}`);
  }

  private resourceURL(): string {
    return `${this.apiURL}/resource`;
  }

  getResources(): Observable<Resource[]> {
    return this.http.get<Resource[]>(this.resourceURL());
  }

  getResource(name: string): Observable<Resource | undefined> {
    return this.http.get<Resource[]>(this.resourceURL())
      .pipe(map((resources) => resources.find((res) => res.name === name)));
  }
}
