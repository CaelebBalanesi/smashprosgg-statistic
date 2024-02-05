import { Component } from '@angular/core';
import { BackendService } from '../../services/backend.service';
import { CharacterSelecterComponent } from '../../components/character-selecter/character-selecter.component';

export interface DateData {
  wins: number,
  total: number,
}

@Component({
  selector: 'app-stats-page',
  standalone: true,
  templateUrl: './stats-page.component.html',
  styleUrl: './stats-page.component.scss',
  imports: [CharacterSelecterComponent],
  providers: []
})
export class StatsPageComponent {

  constructor(private backend: BackendService) {}
  
  statStats = {total: 24, wins: 15, losses: 9};

}
