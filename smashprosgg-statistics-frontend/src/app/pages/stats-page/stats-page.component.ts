import { Component } from '@angular/core';
import { BackendService } from '../../services/backend.service';

@Component({
  selector: 'app-stats-page',
  standalone: true,
  imports: [],
  templateUrl: './stats-page.component.html',
  styleUrl: './stats-page.component.scss'
})
export class StatsPageComponent {

  constructor ( private backend: BackendService) {}

  data: string = "Hello world!";

  OnInit() {
    this.data = this.backend.username;
  }


}
