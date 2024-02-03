import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { environment } from '../../enviroment/enviroment';

export interface GameMapData {
  map: number,
  win: boolean,
  date: string,
  user_character: number,
  opponent_character: number,
}

@Injectable({
  providedIn: 'root'
})

export class BackendService {

  constructor(
    private http: HttpClient,
  ) {}

  username!: string;

  get_set_winrate(username: string) {
    console.log(environment.backend_url + "/game_winrate/user?username=" + username);
    return this.http.get<any>(environment.backend_url + "/game_winrate/user?username=" + username);
  }

  get_game_winrate() {

  }

  set_username(username: string) {
    this.username = username;
  }

  get_map_winrate_over_time(username: string) {
    console.log(environment.backend_url + "/map_winrate_overtime/user?username=" + username);
    return this.http.get<GameMapData[]>(environment.backend_url + "/map_winrate_overtime/user?username=" + username);
  }
}
