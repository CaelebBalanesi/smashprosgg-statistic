import { Component } from '@angular/core';
import { MatIconModule } from '@angular/material/icon'
import { BackendService } from '../../services/backend.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-landing-page',
  standalone: true,
  imports: [
    MatIconModule,
  ],
  templateUrl: './landing-page.component.html',
  styleUrl: './landing-page.component.scss'
})

export class LandingPageComponent {

  constructor (
  ) {

  }

  ngOnInit() {
    console.log("hello");
  }

  open_stats(username: string) {
    console.log(username);
  }

}
