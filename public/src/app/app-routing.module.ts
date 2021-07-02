import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { ResourcesComponent } from './resources/resources.component';
import { DashboardComponent } from './dashboard/dashboard.component';
import { ResourceDetailComponent } from './resource-detail/resource-detail.component';

const routes: Routes = [
  { path: '', redirectTo: '/resources', pathMatch: 'full' },
  { path: 'resources', component: ResourcesComponent },
  { path: 'dashboard', component: DashboardComponent },
  { path: 'detail/:name', component: ResourceDetailComponent },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {}
