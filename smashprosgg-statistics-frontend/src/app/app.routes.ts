import { Routes } from '@angular/router';
import { LandingPageComponent } from './pages/landing-page/landing-page.component';
import { StatsPageComponent } from './pages/stats-page/stats-page.component';

export const routes: Routes = [
    { path: '', component: LandingPageComponent },
    { path: 'stats', component: StatsPageComponent }
];
