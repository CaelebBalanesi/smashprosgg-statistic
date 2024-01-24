import { Component } from '@angular/core';
import { MatIconModule } from '@angular/material/icon'
import { BackendService } from '../../services/backend.service';

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
    private backend: BackendService,
  ) {

  }

  ngOnInit() {
    console.log("hello");
  }

  open_stats(username: string) {
    console.log(username);
  }

}
