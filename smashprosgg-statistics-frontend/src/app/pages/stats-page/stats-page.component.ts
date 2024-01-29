import { Component } from '@angular/core';
import { BackendService } from '../../services/backend.service';
import { HttpClientModule } from '@angular/common/http';

@Component({
  selector: 'app-stats-page',
  standalone: true,
  imports: [
    HttpClientModule,
  ],
  templateUrl: './stats-page.component.html',
  styleUrl: './stats-page.component.scss'
})
export class StatsPageComponent {

  constructor ( private backend: BackendService) {}

  data: string = "Hello world!";
  flag: boolean = true;
  game_winrate: number = 0;

  yoo() {
    this.backend.get_set_winrate(this.backend.username).subscribe(data => {
      console.log(data);
      this.game_winrate = data;
      this.flag = false;
    });
  }

}
