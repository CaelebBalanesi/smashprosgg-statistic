import { Component } from '@angular/core';
import { MatIconModule } from '@angular/material/icon'
import { BackendService } from '../../services/backend.service';
import { Router } from '@angular/router';
import { HttpClientModule } from '@angular/common/http';

@Component({
  selector: 'app-landing-page',
  standalone: true,
  imports: [
    MatIconModule,
    HttpClientModule,
  ],
  templateUrl: './landing-page.component.html',
  styleUrl: './landing-page.component.scss'
})

export class LandingPageComponent {

  constructor (
    private router: Router,
    private backend: BackendService,
  ) {

  }

  ngOnInit() {
    console.log("hello");
  }

  open_stats(username: string) {
    console.log(username);
    this.backend.set_username(username);
    this.router.navigate(["stats"]);
  }

}
