import { Component, OnInit } from '@angular/core';
import { BackendService } from '../../services/backend.service';
import { CharacterSelecterComponent } from '../../components/character-selecter/character-selecter.component';
import { StageIdToNamePipe } from '../../pipes/stage-id-to-name.pipe';

export interface DateData {
  wins: number,
  total: number,
}

export interface Character {
  name: string,
  img: string,
}

export interface StageData {
  name: string, 
  img: string,
  wins: number,
  total: number
}

export interface CharacterPicks {
  user1: { user_id: number, character_id: number },
  user2: { user_id: number, character_id: number },
}

export interface Game {
  id: string,
  winner_id: number,
  stage_id: number,
  date: string,
  character_picks: CharacterPicks,
}

@Component({
  selector: 'app-stats-page',
  standalone: true,
  templateUrl: './stats-page.component.html',
  styleUrl: './stats-page.component.scss',
  imports: [
    CharacterSelecterComponent,
    StageIdToNamePipe
  ],
  providers: []
})
export class StatsPageComponent {

  constructor(private backend: BackendService) {}

  ngOnInit() {
    this.backend.get_games(this.backend.username).subscribe(
      data => {
        console.log(data);
        this.set_data(data[1]);
        this.create_stage_data(this.games, data[0]);
        this.create_game_data(this.games, data[0])
      }
    )
  }
  
  setStats = { total: 0, wins: 0, losses: 0 };
  gameStats = {  total: 0, wins: 0, losses: 0 };
  characters: Character[] = [
    {
      name: "Ness",
      img: "https://storage.googleapis.com/smashpros.gg/characters/icons/ness.png",
    },
    {
      name: "Aegis",
      img: "https://storage.googleapis.com/smashpros.gg/characters/icons/homura.png"
    }, {
      name: "Mario",
      img: "https://storage.googleapis.com/smashpros.gg/characters/icons/mario.png"
    }
  ]
  stages: StageData[] = []
  selectedCharacter: string = "All";
  games: Game[] = [];

  getData(selectedCharacter: string) {
    this.selectedCharacter = selectedCharacter;
  }

  compare( a: StageData, b: StageData ) {
    if ( a.wins / a.total < b.wins / b.total ){
      return -1;
    }
    if ( a.wins / a.total > b.wins / b.total ){
      return 1;
    }
    return 0;
  }

  set_data(data: any[]){
    let games: Game[] = [];
    data.forEach(game => {
      games.push({
        id: game.id,
        winner_id: game.winner_id,
        stage_id: game.stage_id,
        date: game.date,
        character_picks: game.character_picks,
      });
    });
    console.log(games);
    this.games = games;
  }

  create_game_data(games: Game[], user_id: number) {
    this.gameStats = {  total: 0, wins: 0, losses: 0 };
    games.forEach(game => {
      if (game.winner_id == user_id) {
        this.gameStats.wins++;
      } else {
        this.gameStats.losses++;
      }
      this.gameStats.total++;
    });
  }

  create_set_data() {

  }

  create_stage_data(games: Game[], user_id: number){
    this.stages = [];
    games.forEach(game => {
      let flag = false;
      this.stages.forEach(
        stage => {
          if (!flag) {
            if (parseInt(stage.name) == game.stage_id) {
              if (game.winner_id == user_id) {
                stage.wins++;
                stage.total++;
              }
              stage.total++;
              flag = true;
            }
          }
        }
      )
      if (!flag) {
        if (game.winner_id == user_id){
          this.stages.push({
            name: game.stage_id.toString(),
            img: "",
            wins: 1,
            total: 1,
          })
        } else {
          this.stages.push({
            name: game.stage_id.toString(),
            img: "",
            wins: 0,
            total: 1,
          })
        }
      }
    })
    console.log(this.stages);
  }
}